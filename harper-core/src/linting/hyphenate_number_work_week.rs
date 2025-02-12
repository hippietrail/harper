use crate::{
    patterns::{Pattern, SequencePattern, WordSet},
    Token,
};

use super::{Lint, LintKind, PatternLinter, Suggestion};

pub struct HyphenateNumberWorkWeek {
    pattern: Box<dyn Pattern>,
}

impl Default for HyphenateNumberWorkWeek {
    fn default() -> Self {
        let pattern = SequencePattern::default()
            .then_number()
            .then_whitespace()
            .t_aco("day")
            .then_whitespace()
            .then_word_set(WordSet::all(&["work", "working"]))
            .then_whitespace()
            .then_word_set(WordSet::all(&["week", "weeks"]));

        Self {
            pattern: Box::new(pattern),
        }
    }
}

impl PatternLinter for HyphenateNumberWorkWeek {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched_tokens: &[Token], _source: &[char]) -> Lint {
        let number = matched_tokens[0].kind.expect_number();
        let space = matched_tokens[1];

        Lint {
            span: space.span,
            lint_kind: LintKind::Miscellaneous,
            suggestions: vec![Suggestion::ReplaceWith(vec!['-'])],
            message: format!(
                "Use a hyphen in `{}-day` when forming an adjectival compound.",
                number
            ),
            priority: 31,
        }
    }

    fn description(&self) -> &'static str {
        "Ensures a hyphen is used in `X-day` when it is part of a compound adjective, such as `4-day work week`."
    }
}

#[cfg(test)]
mod tests {
    use super::HyphenateNumberWorkWeek;
    use crate::linting::tests::assert_suggestion_result;

    #[test]
    fn corrects_four_day_work_week() {
        assert_suggestion_result(
            "Sign the contract and get 4 day work weeks.",
            HyphenateNumberWorkWeek::default(),
            "Sign the contract and get 4-day work weeks.",
        );
    }

    #[test]
    fn does_not_correct_unrelated_usage() {
        assert_suggestion_result(
            "The trip lasted 4 days, including travel.",
            HyphenateNumberWorkWeek::default(),
            "The trip lasted 4 days, including travel.",
        );
    }
}
