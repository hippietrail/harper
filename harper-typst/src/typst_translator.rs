use crate::OffsetCursor;
use harper_core::{
    parsers::{PlainEnglish, StrParser},
    Punctuation, Token, TokenKind, WordMetadata,
};
use itertools::Itertools;
use typst_syntax::{
    ast::{
        Arg, ArrayItem, AstNode, DestructuringItem, DictItem, Expr, Ident, LetBindingKind, Param,
        Pattern, Spread,
    },
    Source,
};

macro_rules! def_token {
    ($doc:expr, $a:expr, $kind:expr, $offset:ident) => {{
        let range = $doc.range($a.span()).unwrap();
        let start = $offset.push_to(range.start);
        let end_char_loc = start.push_to(range.end).char;

        Some(vec![Token {
            span: harper_core::Span {
                start: start.char,
                end: end_char_loc,
            },
            kind: $kind,
        }])
    }};
}

macro_rules! merge {
    ($($inner:expr),*) => {
        Some(
            [$($inner),*]
                .into_iter()
                .flatten()
                .flatten()
                .collect_vec(),
        )
    };
}

/// Contains values used in parsing so they don't have to be passed around so much
#[derive(Clone, Copy)]
pub struct TypstTranslator<'a> {
    parser: PlainEnglish,
    doc: &'a Source,
}

impl<'a> TypstTranslator<'a> {
    pub fn new(doc: &'a Source) -> Self {
        Self {
            parser: PlainEnglish,
            doc,
        }
    }

    fn parse_english(self, str: impl Into<String>, offset: OffsetCursor) -> Option<Vec<Token>> {
        Some(
            self.parser
                .parse_str(str.into())
                .into_iter()
                .map(|mut t| {
                    t.span.push_by(offset.char);
                    t
                })
                .collect_vec(),
        )
    }

    fn parse_pattern(self, pat: Pattern, offset: OffsetCursor) -> Option<Vec<Token>> {
        macro_rules! token {
            ($a:expr, $kind:expr) => {
                def_token!(self.doc, $a, $kind, offset)
            };
        }

        match pat {
            Pattern::Normal(expr) => self.parse_expr(expr, offset),
            Pattern::Placeholder(underscore) => token!(underscore, TokenKind::Unlintable),
            Pattern::Parenthesized(parenthesized) => merge!(
                self.parse_expr(parenthesized.expr(), offset),
                self.parse_pattern(parenthesized.pattern(), offset)
            ),
            Pattern::Destructuring(destructuring) => Some(
                destructuring
                    .items()
                    .filter_map(|item| match item {
                        DestructuringItem::Pattern(pattern) => self.parse_pattern(pattern, offset),
                        DestructuringItem::Named(named) => merge!(
                            token!(named.name(), TokenKind::Word(WordMetadata::default())),
                            self.parse_pattern(named.pattern(), offset)
                        ),
                        DestructuringItem::Spread(spread) => merge!(
                            spread
                                .sink_ident()
                                .and_then(|ident| self.parse_ident(ident, offset)),
                            spread
                                .sink_expr()
                                .and_then(|expr| self.parse_expr(expr, offset))
                        ),
                    })
                    .flatten()
                    .collect(),
            ),
        }
    }

    fn parse_ident(self, ident: Ident, offset: OffsetCursor) -> Option<Vec<Token>> {
        self.parse_expr(Expr::Ident(ident), offset)
    }

    /// Do not use for spreads contained in DestructuringItem
    fn parse_spread(self, spread: Spread, offset: OffsetCursor) -> Option<Vec<Token>> {
        merge!(
            self.parse_expr(spread.expr(), offset),
            spread
                .sink_ident()
                .and_then(|ident| self.parse_ident(ident, offset))
        )
    }

    pub fn parse_expr(self, ex: Expr, offset: OffsetCursor) -> Option<Vec<Token>> {
        let offset = offset.push_to_span(ex.span());

        macro_rules! token {
            ($a:expr, $kind:expr) => {
                def_token!(self.doc, $a, $kind, offset)
            };
        }
        macro_rules! recurse {
        ($inner:expr) => {
            self.parse_expr($inner, offset)
        };
        ($($inner:expr),*) => {
            merge!(
                $(recurse!($inner)),*
            )
        };
    }

        let iter_recurse = |exprs: &mut dyn Iterator<Item = Expr>| {
            Some(exprs.filter_map(|e| recurse!(e)).flatten().collect_vec())
        };
        let parse_dict = |dict: &mut dyn Iterator<Item = DictItem>| {
            Some(
                dict.filter_map(|di| match di {
                    DictItem::Named(named) => {
                        merge!(
                            self.parse_ident(named.name(), offset),
                            recurse!(named.expr())
                        )
                    }
                    DictItem::Keyed(keyed) => recurse!(keyed.key(), keyed.expr()),
                    DictItem::Spread(spread) => self.parse_spread(spread, offset),
                })
                .flatten()
                .collect_vec(),
            )
        };
        let parse_params = |params: &mut dyn Iterator<Item = Param>| {
            Some(
                params
                    .filter_map(|p| match p {
                        Param::Pos(pattern) => self.parse_pattern(pattern, offset),
                        Param::Named(named) => merge!(
                            self.parse_ident(named.name(), offset),
                            recurse!(named.expr())
                        ),
                        Param::Spread(spread) => self.parse_spread(spread, offset),
                    })
                    .flatten()
                    .collect_vec(),
            )
        };
        let parse_args = |params: &mut dyn Iterator<Item = Arg>| {
            Some(
                params
                    .filter_map(|a| match a {
                        Arg::Pos(expr) => recurse!(expr),
                        Arg::Named(named) => merge!(
                            self.parse_ident(named.name(), offset),
                            recurse!(named.expr())
                        ),
                        Arg::Spread(spread) => self.parse_spread(spread, offset),
                    })
                    .flatten()
                    .collect_vec(),
            )
        };

        match ex {
            Expr::Text(text) => self.parse_english(text.get(), offset.push_to_span(text.span())),
            Expr::Space(a) => {
                let mut chars = self
                    .doc
                    .get(self.doc.range(a.span()).unwrap())
                    .unwrap()
                    .chars();
                let first_char = chars.next().unwrap();
                let length = chars.count() + 1;

                if first_char == '\n' {
                    token!(a, TokenKind::Newline(1))
                } else {
                    token!(a, TokenKind::Space(length))
                }
            }
            Expr::Linebreak(a) => token!(a, TokenKind::Newline(1)),
            Expr::Parbreak(a) => token!(a, TokenKind::ParagraphBreak),
            Expr::SmartQuote(quote) => {
                if quote.double() {
                    token!(
                        quote,
                        TokenKind::Punctuation(Punctuation::Quote(harper_core::Quote {
                            twin_loc: None
                        }))
                    )
                } else {
                    token!(quote, TokenKind::Punctuation(Punctuation::Apostrophe))
                }
            }
            Expr::Strong(strong) => iter_recurse(&mut strong.body().exprs()),
            Expr::Emph(emph) => iter_recurse(&mut emph.body().exprs()),
            Expr::Link(a) => token!(a, TokenKind::Url),
            Expr::Ref(a) => {
                token!(a, TokenKind::Word(WordMetadata::default()))
            }
            Expr::Heading(heading) => iter_recurse(&mut heading.body().exprs()),
            Expr::List(list_item) => iter_recurse(&mut list_item.body().exprs()),
            Expr::Enum(enum_item) => iter_recurse(&mut enum_item.body().exprs()),
            Expr::Term(term_item) => iter_recurse(
                &mut term_item
                    .term()
                    .exprs()
                    .chain(term_item.description().exprs()),
            ),
            Expr::Str(text) => {
                let offset = offset.push_to_span(text.span()).char + 1;
                let string = text.to_untyped().text();

                Some(
                    self.parser
                        .parse_str(&string[1..string.len() - 1])
                        .into_iter()
                        .map(|mut t| {
                            t.span.push_by(offset);
                            t
                        })
                        .collect_vec(),
                )
            }
            Expr::Content(content_block) => iter_recurse(&mut content_block.body().exprs()),
            Expr::Parenthesized(parenthesized) => recurse!(parenthesized.expr()),
            Expr::Array(array) => Some(
                array
                    .items()
                    .filter_map(|i| {
                        if let ArrayItem::Pos(e) = i {
                            recurse!(e)
                        } else {
                            None
                        }
                    })
                    .flatten()
                    .collect_vec(),
            ),
            Expr::Dict(a) => parse_dict(&mut a.items()),
            Expr::FieldAccess(field_access) => merge!(
                recurse!(field_access.target()),
                token!(
                    field_access.field(),
                    TokenKind::Word(WordMetadata::default())
                )
            ),
            Expr::Let(let_binding) => merge!(
                match let_binding.kind() {
                    LetBindingKind::Normal(pattern) => self.parse_pattern(pattern, offset),
                    LetBindingKind::Closure(ident) => self.parse_ident(ident, offset),
                },
                let_binding.init().and_then(|e| recurse!(e))
            ),
            Expr::DestructAssign(destruct_assignment) => {
                recurse!(destruct_assignment.value())
            }
            Expr::Set(set_rule) => merge!(
                recurse!(set_rule.target()),
                set_rule.condition().and_then(|expr| recurse!(expr)),
                parse_args(&mut set_rule.args().items())
            ),
            Expr::Show(show_rule) => merge!(
                recurse!(show_rule.transform()),
                show_rule.selector().and_then(|expr| recurse!(expr))
            ),
            Expr::Contextual(contextual) => recurse!(contextual.body()),
            Expr::Conditional(conditional) => merge!(
                recurse!(conditional.condition(), conditional.if_body()),
                conditional.else_body().and_then(|expr| recurse!(expr))
            ),
            Expr::While(while_loop) => recurse!(while_loop.condition(), while_loop.body()),
            Expr::For(for_loop) => recurse!(for_loop.iterable(), for_loop.body()),
            Expr::Code(code) => iter_recurse(&mut code.body().exprs()),
            Expr::Closure(closure) => merge!(
                closure
                    .name()
                    .and_then(|ident| self.parse_ident(ident, offset)),
                parse_params(&mut closure.params().children()),
                recurse!(closure.body())
            ),
            Expr::FuncCall(func) => merge!(
                token!(func.callee(), TokenKind::Unlintable),
                parse_args(&mut func.args().items())
            ),
            a => token!(a, TokenKind::Unlintable),
        }
    }
}
