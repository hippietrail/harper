use std::collections::VecDeque;

use crate::{Document, Span, Token, VecExt};

mod any_pattern;
mod consumes_remaining_pattern;
mod either_pattern;
mod invert;
mod is_not_title_case;
mod naive_pattern_group;
mod noun_phrase;
mod repeating_pattern;
mod sequence_pattern;
mod token_kind_pattern_group;
mod whitespace_pattern;
mod word_pattern_group;
mod word_set;

pub use any_pattern::AnyPattern;
use blanket::blanket;
pub use consumes_remaining_pattern::ConsumesRemainingPattern;
pub use either_pattern::EitherPattern;
pub use invert::Invert;
pub use is_not_title_case::IsNotTitleCase;
pub use naive_pattern_group::NaivePatternGroup;
pub use noun_phrase::NounPhrase;
pub use repeating_pattern::RepeatingPattern;
pub use sequence_pattern::SequencePattern;
pub use token_kind_pattern_group::TokenKindPatternGroup;
pub use whitespace_pattern::WhitespacePattern;
pub use word_pattern_group::WordPatternGroup;
pub use word_set::WordSet;

#[cfg(not(feature = "concurrent"))]
#[blanket(derive(Rc, Arc))]
pub trait Pattern {
    fn matches(&self, tokens: &[Token], source: &[char]) -> usize;
}

#[cfg(feature = "concurrent")]
#[blanket(derive(Arc))]
pub trait Pattern: Send + Sync {
    fn matches(&self, tokens: &[Token], source: &[char]) -> usize;
}

pub trait PatternExt {
    /// Search through all tokens to locate all non-overlapping pattern matches.
    fn find_all_matches(&self, tokens: &[Token], source: &[char]) -> Vec<Span>;
}

impl<P> PatternExt for P
where
    P: Pattern,
{
    fn find_all_matches(&self, tokens: &[Token], source: &[char]) -> Vec<Span> {
        let mut found = Vec::new();

        for i in 0..tokens.len() {
            let len = self.matches(&tokens[i..], source);

            if len > 0 {
                found.push(Span::new_with_len(i, len));
            }
        }

        if found.len() < 2 {
            return found;
        }

        let mut remove_indices = VecDeque::new();

        for i in 0..found.len() - 1 {
            let cur = &found[i];
            let next = &found[i + 1];

            if cur.overlaps_with(*next) {
                remove_indices.push_back(i + 1);
            }
        }

        found.remove_indices(remove_indices);

        found
    }
}

#[cfg(feature = "concurrent")]
impl<F> Pattern for F
where
    F: Fn(&Token, &[char]) -> bool,
    F: Send + Sync,
{
    fn matches(&self, tokens: &[Token], source: &[char]) -> usize {
        if tokens.is_empty() {
            return 0;
        }

        let tok = &tokens[0];

        if self(tok, source) {
            1
        } else {
            0
        }
    }
}

#[cfg(not(feature = "concurrent"))]
impl<F> Pattern for F
where
    F: Fn(&Token, &[char]) -> bool,
{
    fn matches(&self, tokens: &[Token], source: &[char]) -> usize {
        if tokens.is_empty() {
            return 0;
        }

        let tok = &tokens[0];

        if self(tok, source) {
            1
        } else {
            0
        }
    }
}

pub trait DocPattern {
    fn find_all_matches_in_doc(&self, document: &Document) -> Vec<Span>;
}

impl<P: PatternExt> DocPattern for P {
    fn find_all_matches_in_doc(&self, document: &Document) -> Vec<Span> {
        self.find_all_matches(document.get_tokens(), document.get_source())
    }
}
