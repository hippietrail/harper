use std::sync::Arc;

use itertools::Itertools;

use crate::{CharString, Dictionary, Document, FstDictionary, Span};

use super::{Lint, LintKind, Linter, Suggestion};

pub struct CompoundWords {
    dict: Arc<FstDictionary>,
}

impl CompoundWords {
    pub fn new() -> Self {
        Self {
            dict: FstDictionary::curated(),
        }
    }
}

impl Default for CompoundWords {
    fn default() -> Self {
        Self::new()
    }
}

impl Linter for CompoundWords {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut lints = Vec::new();

        let mut merged_word = CharString::new();
        let mut potential_compounds = Vec::new();

        for (a, w, b) in document.tokens().tuple_windows() {
            if !a.kind.is_word() || !w.kind.is_whitespace() || !b.kind.is_word() {
                continue;
            }

            let a_chars = document.get_span_content(a.span);
            let b_chars = document.get_span_content(b.span);

            // Not super helpful in this case, so we skip it
            if matches!(a_chars, ['a']) {
                continue;
            }

            potential_compounds.clear();

            merged_word.clear();
            merged_word.extend_from_slice(a_chars);
            merged_word.extend_from_slice(b_chars);

            // Check for closed compound words
            if self.dict.contains_word(&merged_word)
                && !a.kind.is_common_word()
                && !b.kind.is_common_word()
            {
                potential_compounds.push(merged_word.clone());
            }

            if !potential_compounds.is_empty() {
                lints.push(Lint {
                    span: Span::new(a.span.start, b.span.end),
                    lint_kind: LintKind::Spelling,
                    suggestions: potential_compounds
                        .drain(..)
                        .map(|v| Suggestion::ReplaceWith(v.to_vec()))
                        .collect(),
                    message:
                        "These two words are often combined to form a hyphenated compound word."
                            .to_owned(),
                    priority: 63,
                });
            }
        }

        lints
    }

    fn description(&self) -> &str {
        "Accidentally inserting a space inside a word is common. This rule looks for valid words that are split by whitespace."
    }
}

#[cfg(test)]
mod tests {
    use crate::linting::tests::{assert_lint_count, assert_suggestion_count};

    use super::CompoundWords;

    #[test]
    fn scarecrow() {
        assert_lint_count(
            "I saw a scare crow in the field today.",
            CompoundWords::default(),
            1,
        );
    }

    #[test]
    fn clean() {
        assert_lint_count(
            "When referring to the political party, make sure to treat them as a proper noun.",
            CompoundWords::default(),
            0,
        );
    }

    #[test]
    fn bookshelf() {
        assert_lint_count(
            "I have a big book shelf in my room.",
            CompoundWords::default(),
            1,
        );
    }

    #[test]
    fn sunscreen() {
        assert_lint_count(
            "Don't forget to apply your sunscreen before going out.",
            CompoundWords::default(),
            0,
        );
    }

    #[test]
    fn birthday() {
        assert_lint_count(
            "We're having a big party to celebrate the couple's birthday today.",
            CompoundWords::default(),
            0,
        );
    }

    #[test]
    fn hometown() {
        assert_lint_count(
            "My home town is a beautiful place with many historical land marks.",
            CompoundWords::default(),
            2,
        );
    }

    #[test]
    fn assertions() {
        assert_lint_count(
            "Make sure to compile with debug ass ertions disabled.",
            CompoundWords::default(),
            1,
        );
    }

    #[test]
    fn break_up() {
        assert_suggestion_count(
            "Like if you break up words you shouldn't.",
            CompoundWords::default(),
            0,
        );
    }
}
