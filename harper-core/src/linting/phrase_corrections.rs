use super::{Lint, LintKind, PatternLinter};
use crate::linting::Suggestion;
use crate::patterns::{Pattern, SimilarToPhrase};
use crate::{Token, TokenStringExt};

/// Generate a linter that will look for a common phrase and correct mild errors that
/// are still composed of real words.
macro_rules! create_linter_for_phrase {
    ($name:ident, $correct_form:literal, $dist:literal) => {
        create_linter_for_phrase!(
            $name,
            SimilarToPhrase::from_phrase($correct_form, $dist),
            $correct_form,
            concat!("Did you mean the phrase `", $correct_form, "`?"),
            concat!(
                "Looks for slight improper modifications to the phrase `",
                $correct_form,
                "`."
            )
        );
    };
    ($name:ident, $pattern:expr, $correct_form:expr, $message:expr, $description:expr) => {
        #[doc = $description]
        pub struct $name {
            pattern: Box<dyn Pattern>,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    pattern: Box::new($pattern),
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl PatternLinter for $name {
            fn pattern(&self) -> &dyn Pattern {
                self.pattern.as_ref()
            }

            fn match_to_lint(&self, matched_tokens: &[Token], source: &[char]) -> Lint {
                let span = matched_tokens.span().unwrap();
                let matched_text = span.get_content(source);

                Lint {
                    span,
                    lint_kind: LintKind::Miscellaneous,
                    suggestions: vec![Suggestion::replace_with_match_case(
                        $correct_form.chars().collect(),
                        matched_text,
                    )],
                    message: $message.to_string(),
                    priority: 31,
                }
            }

            fn description(&self) -> &'static str {
                $description
            }
        }
    };
}

create_linter_for_phrase!(TurnItOff, "turn it off", 1);
create_linter_for_phrase!(HumanLife, "human life", 1);
create_linter_for_phrase!(ThatChallenged, "that challenged", 2);
create_linter_for_phrase!(NoLonger, "no longer", 1);
create_linter_for_phrase!(NeedHelp, "need help", 1);
create_linter_for_phrase!(AndThis, "and this", 1);
create_linter_for_phrase!(Decision, "make a decision", 1);
create_linter_for_phrase!(OfCourse, "of course", 1);

#[cfg(test)]
mod tests {
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    use super::{Decision, OfCourse, TurnItOff};

    #[test]
    fn issue_574() {
        assert_lint_count("run by one", TurnItOff::default(), 0);
    }

    #[test]
    fn turn_it_off_clean_lower() {
        assert_lint_count("turn it off", TurnItOff::default(), 0);
    }

    #[test]
    fn turn_it_off_clean_upper() {
        assert_lint_count("Turn it off", TurnItOff::default(), 0);
    }

    #[test]
    fn of_confusion() {
        assert_suggestion_result("Turn it of", TurnItOff::default(), "Turn it off");
    }

    #[test]
    fn i_and_of_confusion() {
        assert_suggestion_result("Turn i of", TurnItOff::default(), "Turn it off");
    }

    #[test]
    fn take_a_decision() {
        assert_suggestion_result(
            "we should take a decision on this",
            Decision::default(),
            "we should make a decision on this",
        );
    }

    #[test]
    fn off_course() {
        assert_suggestion_result(
            "Yes, off course we should do that.",
            OfCourse::default(),
            "Yes, of course we should do that.",
        );
    }

    #[test]
    fn o_course() {
        assert_suggestion_result(
            "Yes, o course we should do that.",
            OfCourse::default(),
            "Yes, of course we should do that.",
        );
    }
}
