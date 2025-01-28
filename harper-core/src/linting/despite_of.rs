use itertools::Itertools;

use crate::{
    patterns::{Pattern, SequencePattern, WordPatternGroup},
    Lrc, Span, Token,
};

use super::{Lint, LintKind, PatternLinter, Suggestion};

pub struct DespiteOf {
    pattern: Box<dyn Pattern>,
}

impl Default for DespiteOf {
    fn default() -> Self {
        let mut pattern = WordPatternGroup::default();

        let matching_pattern = Lrc::new(
            SequencePattern::default()
                .then_exact_word_or_lowercase("Despite")
                .then_whitespace()
                .then_exact_word("of"),
        );

        pattern.add("in spite of", Box::new(matching_pattern.clone()));
        pattern.add("In spite of", Box::new(matching_pattern.clone()));
        pattern.add("despite", Box::new(matching_pattern.clone()));
        pattern.add("Despite", Box::new(matching_pattern));

        Self {
            pattern: Box::new(pattern),
        }
    }
}

impl PatternLinter for DespiteOf {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched: &[Token], source: &[char]) -> Lint {
        let despite_of = 0..3; // despite + space + of

        let cap = matched[0]
            .span
            .get_content(source)
            .first()
            .unwrap()
            .is_uppercase();

        // generate suggestion based on case
        let suggest_case = |lower: &str, upper: &str| {
            if cap {
                upper.to_string()
            } else {
                lower.to_string()
            }
        };

        let despite = suggest_case("despite", "Despite").chars().collect_vec();
        let in_spite_of = suggest_case("in spite of", "In spite of")
            .chars()
            .collect_vec();

        Lint {
            span: Span::new(matched[despite_of.start].span.start, matched[despite_of.end - 1].span.end),
            lint_kind: LintKind::WordChoice,
            suggestions: vec![Suggestion::ReplaceWith(despite), Suggestion::ReplaceWith(in_spite_of)],
            message: "The phrase “despite of” is incorrect in English. Use either “despite” or “in spite of”.".to_string(),
            priority: 126,
        }
    }

    fn description(&self) -> &'static str {
        "Don\"t use \"despite of\". Both \"despite\" and \"in spite of\" are correct English."
    }
}

#[cfg(test)]
mod tests {
    use super::DespiteOf;
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    #[test]
    fn catches_uppercase() {
        assert_suggestion_result(
            "Despite of the rain, we went for a walk.",
            DespiteOf::default(),
            "Despite the rain, we went for a walk.",
        );
    }

    #[test]
    fn catches_lowercase() {
        assert_suggestion_result(
            "The team performed well, despite the difficulties they faced.",
            DespiteOf::default(),
            "The team performed well, despite the difficulties they faced.",
        );
    }

    #[test]
    fn catches_different_cases() {
        assert_lint_count(
            "Despite of the rain, we went for a walk.",
            DespiteOf::default(),
            1,
        );
    }

    #[test]
    fn likes_correction() {
        assert_lint_count(
            "The team performed well, despite the difficulties they faced. In spite of the rain, we went for a walk.",
            DespiteOf::default(),
            0,
        );
    }
}
