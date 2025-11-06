use crate::expr::{Expr, SequenceExpr};
use crate::linting::{ExprLinter, LintKind};
use crate::token_string_ext::TokenStringExt;
use crate::{Lint, Token};

use crate::char_string::CharStringExt;

pub struct ThereIsThereAreWrongNumber {
    expr: Box<dyn Expr>,
}

impl Default for ThereIsThereAreWrongNumber {
    fn default() -> Self {
        Self {
            expr: Box::new(
                SequenceExpr::default().then_any_of(vec![
                    Box::new(
                        SequenceExpr::any_of(vec![
                            Box::new(SequenceExpr::default().then_fixed_phrase("there are")),
                            Box::new(SequenceExpr::default().then_fixed_phrase("are there")),
                        ])
                        .t_ws()
                        .then_any_of(vec![Box::new(
                            SequenceExpr::default().then_singular_noun_only(),
                        )]),
                    ),
                    Box::new(
                        SequenceExpr::any_of(vec![
                            Box::new(SequenceExpr::default().then_fixed_phrase("is there")),
                            Box::new(SequenceExpr::default().then_fixed_phrase("there is")),
                        ])
                        .t_ws()
                        .then_any_of(vec![Box::new(
                            SequenceExpr::default() //.then_plural_noun_only())
                                .then(|tok: &Token, src: &[char]| {
                                    tok.kind.is_plural_noun_only()
                                        && !tok
                                            .span
                                            .get_content(src)
                                            .eq_ignore_ascii_case_str("other")
                                }),
                        )]),
                    ),
                ]),
            ),
        }
    }
}

impl ExprLinter for ThereIsThereAreWrongNumber {
    fn expr(&self) -> &dyn Expr {
        self.expr.as_ref()
    }

    fn match_to_lint(&self, toks: &[Token], src: &[char]) -> Option<Lint> {
        Some(Lint {
            span: toks.span()?,
            lint_kind: LintKind::Agreement,
            message: "".to_string(),
            suggestions: vec![],
            ..Default::default()
        })
    }

    fn description(&self) -> &str {
        "Makes sure `is` is paired with singular nominals and `are` with plural in `there is`, `are there` etc. constructions"
    }
}

#[cfg(test)]
mod tests {
    use crate::linting::{ThereIsThereAreWrongNumber, tests::assert_lint_count};

    #[test]
    fn there_is() {
        assert_lint_count("There is people", ThereIsThereAreWrongNumber::default(), 1);
    }

    #[test]
    fn there_are() {
        assert_lint_count("There are stuff", ThereIsThereAreWrongNumber::default(), 1);
    }

    #[test]
    fn is_there() {
        assert_lint_count("Is there crowds?", ThereIsThereAreWrongNumber::default(), 1);
    }

    #[test]
    fn are_there() {
        assert_lint_count(
            "Are there something?",
            ThereIsThereAreWrongNumber::default(),
            1,
        );
    }
}
