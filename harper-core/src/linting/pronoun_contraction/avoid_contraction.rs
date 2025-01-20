use crate::{
    patterns::{Pattern, SequencePattern},
    Token,
};

use super::super::{Lint, LintKind, PatternLinter, Suggestion};

pub struct AvoidContraction {
    pattern: Box<dyn Pattern>,
}

impl Default for AvoidContraction {
    fn default() -> Self {
        let pattern = SequencePattern::aco("you're").then_whitespace().then_noun();

        Self {
            pattern: Box::new(pattern),
        }
    }
}

impl PatternLinter for AvoidContraction {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched_tokens: &[Token], source: &[char]) -> Lint {
        let word = matched_tokens[0].span.get_content(source);

        Lint {
            span: matched_tokens[0].span,
            lint_kind: LintKind::WordChoice,
            suggestions: vec![Suggestion::replace_with_match_case(
                vec!['y', 'o', 'u', 'r'],
                word,
            )],
            message: "It appears you intended to use the possessive version of this word"
                .to_owned(),
            priority: 63,
        }
    }

    fn description(&self) -> &'static str {
        "This rule looks for situations where a contraction was used where it shouldn't have been."
    }
}

#[cfg(test)]
mod tests {
    use crate::linting::tests::assert_suggestion_result;

    use super::AvoidContraction;

    #[test]
    fn issue_139() {
        assert_suggestion_result(
            "it would be great if you're PR was merged into tower-lsp",
            AvoidContraction::default(),
            "it would be great if your PR was merged into tower-lsp",
        );
    }

    #[test]
    fn car() {
        assert_suggestion_result(
            "You're car is black.",
            AvoidContraction::default(),
            "Your car is black.",
        );
    }
}
