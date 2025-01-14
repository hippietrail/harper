use crate::{
    patterns::{EitherPattern, Pattern, SequencePattern},
    Punctuation, Token, TokenKind, TokenStringExt,
};

use super::{Lint, LintKind, PatternLinter, Suggestion};

static DESCRIPTION: &str =
    "Currency symbols should always come before the quantity being described.";

pub struct CurrencyPlacement {
    pattern: Box<dyn Pattern>,
}

impl Default for CurrencyPlacement {
    fn default() -> Self {
        let currency_pat = Box::new(|t: &Token, _source: &[char]| {
            matches!(t.kind, TokenKind::Punctuation(Punctuation::Currency(..)))
        });

        let pattern = EitherPattern::new(vec![
            Box::new(
                SequencePattern::default()
                    .then_number()
                    .then_whitespace()
                    .then(currency_pat.clone()),
            ),
            Box::new(SequencePattern::default().then_number().then(currency_pat)),
        ]);

        Self {
            pattern: Box::new(pattern),
        }
    }
}

impl PatternLinter for CurrencyPlacement {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched_tokens: &[Token], _source: &[char]) -> Lint {
        let currency_tok = matched_tokens.first_punctuation().unwrap();
        let currency = currency_tok
            .kind
            .as_punctuation()
            .unwrap()
            .expect_currency();

        // We can unwrap like this thanks to the pattern.
        let number_tok = matched_tokens.first_number().unwrap();
        let (value, suffix) = number_tok.kind.expect_number();

        let mut fix = vec![currency.to_char()];

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

    use super::CurrencyPlacement;

    #[test]
    fn eof() {
        assert_suggestion_result(
            "It was my last bill worth more than 4$.",
            CurrencyPlacement::default(),
            "It was my last bill worth more than $4.",
        );
    }

    #[test]
    fn blog_title() {
        assert_suggestion_result(
            "The Best 25$ I Ever Spent",
            CurrencyPlacement::default(),
            "The Best $25 I Ever Spent",
        );
    }

    #[test]
    fn blog_title_with_space() {
        assert_suggestion_result(
            "The Best 25   $ I Ever Spent",
            CurrencyPlacement::default(),
            "The Best $25 I Ever Spent",
        );
    }

    #[test]
    fn multiple_dollar() {
        assert_suggestion_result(
            "They were either 25$ 24$ or 23$.",
            CurrencyPlacement::default(),
            "They were either $25 $24 or $23.",
        );
    }

    #[test]
    fn multiple_pound() {
        assert_suggestion_result(
            "They were either 25£ 24£ or 23£.",
            CurrencyPlacement::default(),
            "They were either £25 £24 or £23.",
        );
    }

    #[test]
    fn suffix() {
        assert_suggestion_result(
            "It was my 20th$.",
            CurrencyPlacement::default(),
            "It was my $20th.",
        );
    }
}
