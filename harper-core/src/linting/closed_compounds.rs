use crate::{
    patterns::{ExactPhrase, Pattern},
    Token, TokenStringExt,
};

use super::{Lint, LintKind, PatternLinter, Suggestion};

macro_rules! create_closed_compound_linter {
    ($name:ident, $phrase:literal, $correct:expr) => {
        pub struct $name {
            pattern: Box<dyn Pattern>,
        }

        impl Default for $name {
            fn default() -> Self {
                let pattern = ExactPhrase::from_phrase($phrase);

                Self {
                    pattern: Box::new(pattern),
                }
            }
        }

        impl PatternLinter for $name {
            fn pattern(&self) -> &dyn Pattern {
                self.pattern.as_ref()
            }

            fn match_to_lint(&self, matched_tokens: &[Token], source: &[char]) -> Lint {
                let span = matched_tokens.span().unwrap();
                let orig_chars = span.get_content(source);

                Lint {
                    span,
                    lint_kind: LintKind::WordChoice,
                    suggestions: vec![Suggestion::replace_with_match_case(
                        $correct.chars().collect(),
                        orig_chars,
                    )],
                    message: format!("Did you mean the closed compound `{}`?", $correct),
                    ..Default::default()
                }
            }

            fn description(&self) -> &'static str {
                concat!(
                    "Looks for incorrect spacing inside the closed compound `",
                    $correct,
                    "`."
                )
            }
        }
    };
}

create_closed_compound_linter!(Itself, "it self", "itself");

#[cfg(test)]
mod tests {
    use super::Itself;
    use crate::linting::tests::assert_suggestion_result;

    #[test]
    fn it_self() {
        let test_sentence = "The project, it self, was quite challenging.";
        let expected = "The project, itself, was quite challenging.";
        assert_suggestion_result(test_sentence, Itself::default(), expected);
    }
}
