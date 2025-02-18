use crate::{
    patterns::{Pattern, SequencePattern, WordSet},
    Token, TokenStringExt,
};

use super::{Lint, LintKind, PatternLinter, Suggestion};

pub struct ModalOf {
    pattern: Box<dyn Pattern>,
}

impl Default for ModalOf {
    fn default() -> Self {
        Self {
            pattern: Box::new(
                SequencePattern::default()
                    .then(Box::new(WordSet::all(&[
                        "could",
                        "couldn't",
                        "had",
                        "hadn't",
                        "must",
                        "mustn't",
                        "should",
                        "shouldn't",
                        "would",
                        "wouldn't",
                    ])))
                    .then_whitespace()
                    .then_exact_word("of"),
            ),
        }
    }
}

impl PatternLinter for ModalOf {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched_toks: &[Token], source: &[char]) -> Lint {
        // let's get the span of the word "of" from the matched tokens
        let modal_ws_of = matched_toks.span().unwrap();
        // let span = matched_toks[2].span;

        Lint {
            span: modal_ws_of,
            lint_kind: LintKind::WordChoice,
            suggestions: vec![
                Suggestion::replace_with_match_case(
                    // value - a vector of chars
                    "of".chars().collect(),
                    // template - a slice of chars
                    modal_ws_of.get_content(source),
                )
            ],
            message: "After an auxiliary verb, the correct word is `have`.".to_owned(),
            priority: 126,
        }
    }

    fn description(&self) -> &'static str {
        "Detects incorrect use of the word `of` after modal verbs."
    }
}

#[cfg(test)]
mod tests {
    use super::ModalOf;
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    // a lint_count test
    #[test]
    fn test_could_of() {
        assert_lint_count(
            "could of",
            ModalOf::default(), 
            1
        );
    }

    // a suggestion result test
    #[test]
    fn test_could_of_suggestion() {
        assert_suggestion_result(
            "could of",
            ModalOf::default(), 
            "have"
        );
    }
}