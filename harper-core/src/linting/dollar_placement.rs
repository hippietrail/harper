use crate::{
    patterns::{EitherPattern, Pattern, SequencePattern},
    Punctuation, Token, TokenKind, TokenStringExt,
};

use super::{Lint, LintKind, PatternLinter, Suggestion};

static DESCRIPTION: &str = "Dollar signs should always come before the quantity being described.";

pub struct DollarPlacement {
    pattern: Box<dyn Pattern>,
}

impl Default for DollarPlacement {
    fn default() -> Self {
        let dollar_pat = Box::new(|t: &Token, _source: &[char]| {
            matches!(t.kind, TokenKind::Punctuation(Punctuation::Dollar))
        });

        let pattern = EitherPattern::new(vec![
            Box::new(
                SequencePattern::default()
                    .then_number()
                    .then_whitespace()
                    .then(dollar_pat.clone()),
            ),
            Box::new(SequencePattern::default().then_number().then(dollar_pat)),
        ]);

        Self {
            pattern: Box::new(pattern),
        }
    }
}

impl PatternLinter for DollarPlacement {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched_tokens: &[Token], _source: &[char]) -> Lint {
        // We can unwrap like this thanks to the pattern.
        let number_tok = matched_tokens.first_number().unwrap();
        let (value, suffix) = number_tok.kind.as_number().unwrap();

        let mut fix = vec!['$'];

        fix.extend(value.to_string().chars());

        if let Some(suffix) = suffix {
            fix.extend(suffix.to_chars());
        }

        Lint {
            span: matched_tokens.span().unwrap(),
            lint_kind: LintKind::Formatting,
            suggestions: vec![Suggestion::ReplaceWith(fix)],
            message: DESCRIPTION.to_string(),
            priority: 63,
        }
    }

    fn description(&self) -> &'static str {
        DESCRIPTION
    }
}

#[cfg(test)]
mod tests {
    use crate::linting::tests::assert_suggestion_result;

    use super::DollarPlacement;

    #[test]
    fn eof() {
        assert_suggestion_result(
            "It was my last bill worth more than 4$.",
            DollarPlacement::default(),
            "It was my last bill worth more than $4.",
        );
    }

    #[test]
    fn blog_title() {
        assert_suggestion_result(
            "The Best 25$ I Ever Spent",
            DollarPlacement::default(),
            "The Best $25 I Ever Spent",
        );
    }

    #[test]
    fn blog_title_with_space() {
        assert_suggestion_result(
            "The Best 25   $ I Ever Spent",
            DollarPlacement::default(),
            "The Best $25 I Ever Spent",
        );
    }

    #[test]
    fn multiple() {
        assert_suggestion_result(
            "They were either 25$ 24$ or 23$.",
            DollarPlacement::default(),
            "They were either $25 $24 or $23.",
        );
    }

    #[test]
    fn suffix() {
        assert_suggestion_result(
            "It was my 20th$.",
            DollarPlacement::default(),
            "It was my $20th.",
        );
    }
}
