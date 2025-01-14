use crate::{
    patterns::{Pattern, SequencePattern},
    Punctuation, Token, TokenKind, TokenStringExt,
};

use super::{Lint, LintKind, PatternLinter, Suggestion};

static DESCRIPTION: &str = "Dollar signs should always come before the quantity being described.";

pub struct DollarPlacement {
    pattern: Box<dyn Pattern>,
}

impl Default for DollarPlacement {
    fn default() -> Self {
        let pattern = SequencePattern::default().then_number().then(Box::new(
            |t: &Token, _source: &[char]| {
                matches!(t.kind, TokenKind::Punctuation(Punctuation::Dollar))
            },
        ));

        Self {
            pattern: Box::new(pattern),
        }
    }
}

impl PatternLinter for DollarPlacement {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched_tokens: &[Token], source: &[char]) -> Lint {
        let _ = source;
        // We can unwrap like this thanks to the pattern.
        let (value, suffix) = matched_tokens.first().unwrap().kind.as_number().unwrap();

        let mut fix = vec!['$'];

        fix.extend(value.to_string().chars());

        if let Some(suffix) = suffix {
            fix.extend(suffix.to_chars());
        }

        Lint {
            span: matched_tokens[0..2].span().unwrap(),
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
    fn suffix() {
        assert_suggestion_result(
            "It was my 20th$.",
            DollarPlacement::default(),
            "It was my $20th.",
        );
    }
}
