use crate::{
    patterns::{Pattern, SequencePattern},
    Token,
};

use super::{Lint, LintKind, PatternLinter, Suggestion};

pub struct PiqueInterest {
    pattern: Box<dyn Pattern>,
}

impl Default for PiqueInterest {
    fn default() -> Self {
        let pattern = SequencePattern::aco("peak")
            .then_whitespace()
            .then_singular_subject()
            .then_whitespace()
            .t_aco("interest");

        Self {
            pattern: Box::new(pattern),
        }
    }
}

impl PatternLinter for PiqueInterest {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched_tokens: &[Token], source: &[char]) -> Lint {
        let span = matched_tokens[0].span;

        Lint {
            span,
            lint_kind: LintKind::WordChoice,
            suggestions: vec![Suggestion::replace_with_match_case(
                "pique".chars().collect(),
                matched_tokens[0].span.get_content(source),
            )],
            message: "Did you mean `pique` instead of `peak`?".to_owned(),
            priority: 31,
        }
    }

    fn description(&self) -> &'static str {
        "Ensures `pique interest` is used correctly instead of `peak interest`."
    }
}

#[cfg(test)]
mod tests {
    use super::PiqueInterest;
    use crate::linting::tests::assert_suggestion_result;

    #[test]
    fn corrects_peak_interest() {
        assert_suggestion_result(
            "The story managed to peak his interest.",
            PiqueInterest::default(),
            "The story managed to pique his interest.",
        );
    }

    #[test]
    fn does_not_correct_unrelated_peak() {
        assert_suggestion_result(
            "He reached the peak of the mountain.",
            PiqueInterest::default(),
            "He reached the peak of the mountain.",
        );
    }
}
