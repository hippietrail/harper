mod offset_cursor;
mod typst_translator;

use offset_cursor::OffsetCursor;
use typst_translator::TypstTranslator;

use harper_core::{
    parsers::Parser,
    patterns::{PatternExt, SequencePattern},
    ConjunctionData, Lrc, NounData, Token, TokenKind, VecExt, WordMetadata,
};
use itertools::Itertools;
use typst_syntax::{
    ast::{AstNode, Markup},
    Source,
};

/// A parser that wraps the [`PlainEnglish`] parser allowing one to parse Typst files.
pub struct Typst;

thread_local! {
    static WORD_APOSTROPHE_WORD: Lrc<SequencePattern> = Lrc::new(SequencePattern::default()
                .then_any_word()
                .then_apostrophe()
                .then_any_word());
}

impl Parser for Typst {
    fn parse(&self, source: &[char]) -> Vec<Token> {
        let source_str: String = source.iter().collect();
        let typst_document = Source::detached(source_str);
        let typst_tree = Markup::from_untyped(typst_document.root())
            .expect("Unable to create typst document from parsed tree!");
        let parse_helper = TypstTranslator::new(&typst_document);

        let mut tokens = typst_tree
            .exprs()
            .filter_map(|ex| parse_helper.parse_expr(ex, OffsetCursor::new(&typst_document)))
            .flatten()
            .collect_vec();

        // Consolidate conjunctions
        let mut to_remove = std::collections::VecDeque::default();
        for tok_span in WORD_APOSTROPHE_WORD
            .with(|v| v.clone())
            .find_all_matches(&tokens, source)
        {
            let start_tok = &tokens[tok_span.start];
            let end_tok = &tokens[tok_span.end - 1];
            let char_span = harper_core::Span::new(start_tok.span.start, end_tok.span.end);

            if let TokenKind::Word(metadata) = start_tok.kind {
                tokens[tok_span.start].kind =
                    TokenKind::Word(if end_tok.span.get_content(source) == ['s'] {
                        WordMetadata {
                            noun: Some(NounData {
                                is_possessive: Some(true),
                                ..metadata.noun.unwrap_or_default()
                            }),
                            conjunction: None,
                            ..metadata
                        }
                    } else {
                        WordMetadata {
                            noun: metadata.noun.map(|noun| NounData {
                                is_possessive: Some(false),
                                ..noun
                            }),
                            conjunction: Some(ConjunctionData {}),
                            ..metadata
                        }
                    });

                tokens[tok_span.start].span = char_span;
                to_remove.extend(tok_span.start + 1..tok_span.end);
            } else {
                panic!("Apostrophe consolidation does not start with Word Token!")
            }
        }
        tokens.remove_indices(to_remove.into_iter().sorted().unique().collect());

        tokens
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use ordered_float::OrderedFloat;

    use super::Typst;
    use harper_core::{parsers::StrParser, NounData, Punctuation, TokenKind, WordMetadata};

    #[test]
    fn conjunction() {
        let source = "doesn't";

        let tokens = Typst.parse_str(source);
        let token_kinds = tokens.iter().map(|t| t.kind).collect_vec();
        dbg!(&token_kinds);

        assert_eq!(token_kinds.len(), 1);
        assert!(token_kinds.into_iter().all(|t| t.is_conjunction()))
    }

    #[test]
    fn possessive() {
        let source = "person's";

        let tokens = Typst.parse_str(source);
        let token_kinds = tokens.iter().map(|t| t.kind).collect_vec();
        dbg!(&token_kinds);

        assert_eq!(token_kinds.len(), 1);
        assert!(token_kinds.into_iter().all(|t| {
            matches!(
                t,
                TokenKind::Word(WordMetadata {
                    noun: Some(NounData {
                        is_possessive: Some(true),
                        ..
                    }),
                    ..
                })
            )
        }))
    }

    #[test]
    fn number() {
        let source = "12 is larger than 11, but much less than 11!";

        let tokens = Typst.parse_str(source);
        let token_kinds = tokens.iter().map(|t| t.kind).collect_vec();
        dbg!(&token_kinds);

        assert!(matches!(
            token_kinds.as_slice(),
            &[
                TokenKind::Number(OrderedFloat(12.0), None),
                TokenKind::Space(1),
                TokenKind::Word(_),
                TokenKind::Space(1),
                TokenKind::Word(_),
                TokenKind::Space(1),
                TokenKind::Word(_),
                TokenKind::Space(1),
                TokenKind::Number(OrderedFloat(11.0), None),
                TokenKind::Punctuation(Punctuation::Comma),
                TokenKind::Space(1),
                TokenKind::Word(_),
                TokenKind::Space(1),
                TokenKind::Word(_),
                TokenKind::Space(1),
                TokenKind::Word(_),
                TokenKind::Space(1),
                TokenKind::Word(_),
                TokenKind::Space(1),
                TokenKind::Number(OrderedFloat(11.0), None),
                TokenKind::Punctuation(Punctuation::Bang),
            ]
        ))
    }

    #[test]
    fn math_unlintable() {
        let source = "$12 > 11$, $12 << 11!$";

        let tokens = Typst.parse_str(source);
        let token_kinds = tokens.iter().map(|t| t.kind).collect_vec();
        dbg!(&token_kinds);

        assert!(matches!(
            token_kinds.as_slice(),
            &[
                TokenKind::Unlintable,
                TokenKind::Punctuation(Punctuation::Comma),
                TokenKind::Space(1),
                TokenKind::Unlintable,
            ]
        ))
    }

    #[test]
    fn dict_parsing() {
        let source = r#"#let dict = (
                        name: "Typst",
                        born: 2019,
                      )"#;

        let tokens = Typst.parse_str(source);
        let token_kinds = tokens.iter().map(|t| t.kind).collect_vec();
        dbg!(&token_kinds);

        let charslice = source.chars().collect_vec();
        assert_eq!(tokens[2].span.get_content_string(&charslice), "Typst");

        assert!(matches!(
            token_kinds.as_slice(),
            &[
                TokenKind::Unlintable, // Ident
                TokenKind::Unlintable, // Key 1
                TokenKind::Word(_),    // Value 1
                TokenKind::Unlintable, // Key 2
                TokenKind::Unlintable, // Value 2
            ]
        ))
    }

    #[test]
    fn str_parsing() {
        let source = r#"#let ident = "This is a string""#;

        let token_kinds = Typst.parse_str(source).iter().map(|t| t.kind).collect_vec();
        dbg!(&token_kinds);

        assert!(matches!(
            &token_kinds.as_slice(),
            &[
                TokenKind::Unlintable,
                TokenKind::Word(_), // This
                TokenKind::Space(1),
                TokenKind::Word(_), // Is
                TokenKind::Space(1),
                TokenKind::Word(_), // A
                TokenKind::Space(1),
                TokenKind::Word(_), // String
            ]
        ))
    }

    #[test]
    fn non_adjacent_spaces_not_condensed() {
        let source = r#"#authors_slice.join(", ", last: ", and ")  bob"#;

        let token_kinds = Typst.parse_str(source).iter().map(|t| t.kind).collect_vec();
        dbg!(&token_kinds);

        assert!(matches!(
            &token_kinds.as_slice(),
            &[
                TokenKind::Unlintable, // authors_slice.join
                TokenKind::Punctuation(Punctuation::Comma),
                TokenKind::Space(1),
                TokenKind::Unlintable, // Ident
                TokenKind::Punctuation(Punctuation::Comma),
                TokenKind::Space(1),
                TokenKind::Word(_), // and
                TokenKind::Space(1),
                TokenKind::Space(2),
                TokenKind::Word(_),
            ]
        ))
    }

    #[test]
    fn header_parsing() {
        let source = r"= Header
                       Paragraph";

        let tokens = Typst.parse_str(source);
        let token_kinds = tokens.iter().map(|t| t.kind).collect_vec();
        dbg!(&token_kinds);

        let charslice = source.chars().collect_vec();
        assert_eq!(tokens[0].span.get_content_string(&charslice), "Header");
        assert_eq!(tokens[2].span.get_content_string(&charslice), "Paragraph");

        assert!(matches!(
            &token_kinds.as_slice(),
            &[
                TokenKind::Word(_),
                TokenKind::Newline(1),
                TokenKind::Word(_)
            ]
        ))
    }

    #[test]
    fn parbreak() {
        let source = r"Paragraph

                       Paragraph";

        let token_kinds = Typst.parse_str(source).iter().map(|t| t.kind).collect_vec();
        dbg!(&token_kinds);

        assert!(matches!(
            &token_kinds.as_slice(),
            &[
                TokenKind::Word(_),
                TokenKind::ParagraphBreak,
                TokenKind::Word(_),
            ]
        ))
    }

    #[test]
    fn label_unlintable() {
        let source = r"= Header
                       <label>
                       Paragraph";

        let tokens = Typst.parse_str(source);
        let token_kinds = tokens.iter().map(|t| t.kind).collect_vec();
        dbg!(&token_kinds);

        assert!(matches!(
            &token_kinds.as_slice(),
            &[
                TokenKind::Word(_),
                TokenKind::Newline(1),
                TokenKind::Unlintable,
                TokenKind::Newline(1),
                TokenKind::Word(_),
            ]
        ))
    }

    #[test]
    fn sentence() {
        let source = "This is a sentence, it is not interesting.";

        let tokens = Typst.parse_str(source);
        let token_kinds = tokens.iter().map(|t| t.kind).collect_vec();
        dbg!(&token_kinds);

        assert!(matches!(
            token_kinds.as_slice(),
            &[
                TokenKind::Word(_),
                TokenKind::Space(1),
                TokenKind::Word(_),
                TokenKind::Space(1),
                TokenKind::Word(_),
                TokenKind::Space(1),
                TokenKind::Word(_),
                TokenKind::Punctuation(Punctuation::Comma),
                TokenKind::Space(1),
                TokenKind::Word(_),
                TokenKind::Space(1),
                TokenKind::Word(_),
                TokenKind::Space(1),
                TokenKind::Word(_),
                TokenKind::Space(1),
                TokenKind::Word(_),
                TokenKind::Punctuation(Punctuation::Period),
            ]
        ))
    }

    #[test]
    fn smart_apostrophe_newline() {
        let source = r#"group’s
writing"#;

        let tokens = Typst.parse_str(source);
        let token_kinds = tokens.iter().map(|t| t.kind).collect_vec();
        dbg!(&token_kinds);

        let charslice = source.chars().collect_vec();
        assert_eq!(tokens[2].span.get_content_string(&charslice), "writing");

        assert!(matches!(
            token_kinds.as_slice(),
            &[
                TokenKind::Word(WordMetadata {
                    noun: Some(NounData {
                        is_possessive: Some(true),
                        ..
                    }),
                    ..
                }),
                TokenKind::Newline(1),
                TokenKind::Word(_),
            ]
        ));
    }
}
