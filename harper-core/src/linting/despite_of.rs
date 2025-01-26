use itertools::Itertools;

use crate::{
    patterns::{Pattern, SequencePattern, WordPatternGroup},
    Lrc, Token, TokenStringExt,
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

        // TODO I don't actually know what these seemingly redundant lines are for
        // TODO should it be the words I'm replacing, the substitutions, one uppercase and one lowercase?
        pattern.add("of", Box::new(matching_pattern.clone()));
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

    fn match_to_lint(&self, matched_tokens: &[Token], source: &[char]) -> Lint {
        let suggestion = format!(
            // TODO I think the {} don't belong here. Also how to handle upper vs lower case?
            "in spite of {}", // "despite {}", // TODO how to make multiple suggestions?
            matched_tokens[0]
                .span
                .get_content(source)
                .iter()
                .collect::<String>()
        )
        .chars()
        .collect_vec();

        Lint {
            span: matched_tokens.span().unwrap(),
            lint_kind: LintKind::Repetition,
            suggestions: vec![Suggestion::ReplaceWith(suggestion)],
            message: "The phrase “despite of” is incorrect in English. Use either “despite” or “in spite of”.".to_string(),
            priority: 126,
        }
    }

    fn description(&self) -> &'static str {
        "Flag “despite of”."
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::assert_lint_count;
    use super::DespiteOf;

    #[test]
    fn catches_lowercase() {
        assert_lint_count(
            "The team performed well, despite of the difficulties they faced.",
            DespiteOf::default(),
            1,
        );
    }

    #[test]
    fn catches_different_cases() {
        assert_lint_count("Despite the rain, we went for a walk.", DespiteOf::default(), 1);
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
