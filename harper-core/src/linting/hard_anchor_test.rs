
use crate::expr::{AnchorHardEnd, AnchorHardStart, Expr, SequenceExpr};
use crate::linting::{ExprLinter, Lint, LintKind, Suggestion};
use crate::{Token, TokenKind};
use crate::token_string_ext::TokenStringExt;

/// A [`Linter`] purely for testing [`AnchorHardStart`] and [`AnchorHardEnd`].
/// Matches any word immediately bordered by hard anchors.
/// (Actually any word except "hard", since it's the replacement and would cause an infinite loop.)
pub struct HardAnchorTest {
    expr: Box<dyn Expr>,
}

impl Default for HardAnchorTest {
    fn default() -> Self {
        Self {
            expr: Box::new(
                SequenceExpr::default()
                .then(AnchorHardStart)
                .then_kind_any_except(&[TokenKind::is_word], &["hard"])
                .then(AnchorHardEnd)
            ),
        }
    }
}
    
impl ExprLinter for HardAnchorTest {
    fn expr(&self) -> &dyn Expr {
        self.expr.as_ref()
    }

    fn match_to_lint(&self, toks: &[Token], src: &[char]) -> std::option::Option<Lint> {
        eprintln!("⚽️ '{}'", toks.span()?.get_content_string(src));
        Some(Lint {
            span: toks.span()?,
            lint_kind: LintKind::Miscellaneous,
            suggestions: vec![Suggestion::ReplaceWith(
                ['H', 'a', 'r', 'D'].to_vec()
            )],
            message: "Just testing".to_string(),
            priority: 0,
        })
    }

    fn description(&self) -> &str {
        "Just testing"
    }
}

#[cfg(test)]
mod tests {
    use crate::linting::tests::{assert_no_lints, assert_suggestion_result};

    use super::HardAnchorTest;

    #[test]
    fn two_words() {
        assert_no_lints(
            "Hello world",
            HardAnchorTest::default()
        );
    }

    #[test]
    fn one_word() {
        assert_suggestion_result(
            "Helloworld",
            HardAnchorTest::default(),
            "HarD",
        );
    }

    #[test]
    fn space_then_one_word() {
        assert_no_lints(
            " Helloworld",
            HardAnchorTest::default(),
        );
    }

    #[test]
    fn one_word_then_space() {
        assert_no_lints(
            "Helloworld ",
            HardAnchorTest::default(),
        );
    }

    #[test]
    fn one_word_then_period() {
        assert_suggestion_result(
            "Helloworld.",
            HardAnchorTest::default(),
            "HarD.",
        );
    }

    #[test]
    fn one_word_then_comma() {
        assert_suggestion_result(
            "Helloworld,",
            HardAnchorTest::default(),
            "HarD,",
        );
    }

    #[test]
    fn dot_then_word() {
        assert_suggestion_result(
            ".Helloworld",
            HardAnchorTest::default(),
            ".HarD",
        );
    }

    #[test]
    fn fantsy() {
        assert_suggestion_result(
            " one.two. three .four . five",
            HardAnchorTest::default(),
            " one.HarD. three .four . five",
        );
    }
}
