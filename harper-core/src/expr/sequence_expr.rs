use paste::paste;

use crate::{
    Span, Token, TokenKind,
    patterns::{AnyPattern, IndefiniteArticle, WhitespacePattern, Word},
};

use super::{Expr, Optional, Repeating, Step, UnlessStep};

#[derive(Default)]
pub struct SequenceExpr {
    exprs: Vec<Box<dyn Expr>>,
}

/// Generate a `then_*` method from an available `is_*` function on [`TokenKind`].
macro_rules! gen_then_from_is {
    ($quality:ident) => {
        paste! {
            #[doc = concat!("Adds a step matching a token where [`TokenKind::is_", stringify!($quality), "()`] returns true.")]
            pub fn [< then_$quality >] (self) -> Self{
                self.then(|tok: &Token, _source: &[char]| {
                    tok.kind.[< is_$quality >]()
                })
            }

            #[doc = concat!("Adds an optional step matching a token where [`TokenKind::is_", stringify!($quality), "()`] returns true.")]
            pub fn [< then_optional_$quality >] (self) -> Self{
                self.then_optional(|tok: &Token, _source: &[char]| {
                    tok.kind.[< is_$quality >]()
                })
            }

            #[doc = concat!("Adds a step matching one or more consecutive tokens where [`TokenKind::is_", stringify!($quality), "()`] returns true.")]
            pub fn [< then_one_or_more_$quality s >] (self) -> Self{
                self.then_one_or_more(Box::new(|tok: &Token, _source: &[char]| {
                    tok.kind.[< is_$quality >]()
                }))
            }

            #[doc = concat!("Adds a step matching a token where [`TokenKind::is_", stringify!($quality), "()`] returns false.")]
            pub fn [< then_anything_but_$quality >] (self) -> Self{
                self.then(|tok: &Token, _source: &[char]| {
                    if tok.kind.[< is_$quality >](){
                        false
                    }else{
                        true
                    }
                })
            }
        }
    };
}

impl Expr for SequenceExpr {
    /// Run the expression starting at an index, returning the total matched window.
    ///
    /// If any step returns `None`, the entire expression does as well.
    fn run(&self, mut cursor: usize, tokens: &[Token], source: &[char]) -> Option<Span> {
        let mut window = Span::new_with_len(cursor, 0);

        for cur_expr in &self.exprs {
            let out = cur_expr.run(cursor, tokens, source)?;

            // Only expand the window if the match actually covers some tokens
            if out.end > out.start {
                window.expand_to_include(out.start);
                window.expand_to_include(out.end.checked_sub(1).unwrap_or(out.start));
            }

            // Only advance cursor if we actually matched something
            if out.end > cursor {
                cursor = out.end;
            } else if out.start < cursor {
                cursor = out.start;
            }
            // If both start and end are equal to cursor, don't move the cursor
        }

        Some(window)
    }
}

impl SequenceExpr {
    // Constructor methods

    /// Construct a new sequence with a [`Word`] at the beginning of the operation list.
    pub fn any_capitalization_of(word: &'static str) -> Self {
        Self::default().then_any_capitalization_of(word)
    }

    /// Shorthand for [`Self::any_capitalization_of`].
    pub fn aco(word: &'static str) -> Self {
        Self::any_capitalization_of(word)
    }

    // General builder methods

    /// Push an [expression](Expr) to the operation list.
    pub fn then(mut self, expr: impl Expr + 'static) -> Self {
        self.exprs.push(Box::new(expr));
        self
    }

    /// Pushes an expression that could move the cursor to the sequence, but does not require it.
    pub fn then_optional(mut self, expr: impl Expr + 'static) -> Self {
        self.exprs.push(Box::new(Optional::new(expr)));
        self
    }

    /// Appends the steps in `other` onto the end of `self`.
    /// This is more efficient than [`Self::then`] because it avoids pointer redirection.
    pub fn then_seq(mut self, mut other: Self) -> Self {
        self.exprs.append(&mut other.exprs);
        self
    }

    /// Matches any token whose `Kind` exactly matches.
    pub fn then_strict(self, kind: TokenKind) -> Self {
        self.then(move |tok: &Token, _source: &[char]| tok.kind == kind)
    }

    /// Match against one or more whitespace tokens.
    pub fn then_whitespace(self) -> Self {
        self.then(WhitespacePattern)
    }

    /// Shorthand for [`Self::then_whitespace`].
    pub fn t_ws(self) -> Self {
        self.then_whitespace()
    }

    pub fn then_one_or_more(self, expr: impl Expr + 'static) -> Self {
        self.then(Repeating::new(Box::new(expr), 1))
    }

    /// Create a new condition that will step one token forward if met.
    /// If the condition is _not_ met, the whole expression returns `None`.
    ///
    /// This can be used to build out exceptions to other rules.
    ///
    /// See [`UnlessStep`] for more info.
    pub fn then_unless(self, condition: impl Expr + 'static) -> Self {
        self.then(UnlessStep::new(condition, |_tok: &Token, _src: &[char]| {
            true
        }))
    }

    /// Match any single token.
    ///
    /// See [`AnyPattern`] for more info.
    pub fn then_anything(self) -> Self {
        self.then(AnyPattern)
    }

    /// Match any single token.
    ///
    /// Shorthand for [`Self::then_anything`].
    pub fn t_any(self) -> Self {
        self.then_anything()
    }

    // Word matching methods

    /// Matches any word.
    pub fn then_any_word(self) -> Self {
        self.then(|tok: &Token, _source: &[char]| tok.kind.is_word())
    }

    /// Match examples of `word` that have any capitalization.
    pub fn then_any_capitalization_of(self, word: &'static str) -> Self {
        self.then(Word::new(word))
    }

    /// Shorthand for [`Self::then_any_capitalization_of`].
    pub fn t_aco(self, word: &'static str) -> Self {
        self.then_any_capitalization_of(word)
    }

    /// Match examples of `word` case-sensitively.
    pub fn then_exact_word(self, word: &'static str) -> Self {
        self.then(Word::new_exact(word))
    }

    // Part-of-speech matching methods

    // Nominals (nouns and pronouns)

    gen_then_from_is!(nominal);
    gen_then_from_is!(plural_nominal);
    gen_then_from_is!(non_plural_nominal);
    gen_then_from_is!(possessive_nominal);

    // Nouns

    gen_then_from_is!(noun);
    gen_then_from_is!(proper_noun);
    gen_then_from_is!(mass_noun_only);

    // Pronouns

    gen_then_from_is!(pronoun);
    gen_then_from_is!(first_person_singular_pronoun);
    gen_then_from_is!(first_person_plural_pronoun);
    gen_then_from_is!(second_person_pronoun);
    gen_then_from_is!(third_person_pronoun);
    gen_then_from_is!(third_person_singular_pronoun);
    gen_then_from_is!(third_person_plural_pronoun);

    // Verbs

    // POS - Verbs
    gen_then_from_is!(verb);
    gen_then_from_is!(auxiliary_verb);
    gen_then_from_is!(linking_verb);

    // Adjectives and adverbs

    gen_then_from_is!(adjective);
    gen_then_from_is!(adverb);

    // Determiners

    gen_then_from_is!(determiner);

    /// Push an [`IndefiniteArticle`] to the end of the operation list.
    pub fn then_indefinite_article(self) -> Self {
        self.then(IndefiniteArticle::default())
    }

    // Other parts of speech

    gen_then_from_is!(conjunction);
    gen_then_from_is!(preposition);

    // Punctuation

    gen_then_from_is!(punctuation);
    gen_then_from_is!(apostrophe);
    gen_then_from_is!(comma);
    gen_then_from_is!(hyphen);
    gen_then_from_is!(period);
    gen_then_from_is!(semicolon);

    // Other

    gen_then_from_is!(number);
    gen_then_from_is!(case_separator);
}

impl<S> From<S> for SequenceExpr
where
    S: Step + 'static,
{
    fn from(step: S) -> Self {
        Self {
            exprs: vec![Box::new(step)],
        }
    }
}
