use crate::{
    Lrc, Token,
    expr::{Expr, LongestMatchOf, SequenceExpr},
    linting::{ExprLinter, Lint},
};

pub struct AffectEffect {
    expr: Box<dyn Expr>,
}

impl Default for AffectEffect {
    fn default() -> Self {
        let affect_or_effect = Lrc::new(SequenceExpr::word_set(&[
            "affect",
            "affected",
            "affecting",
            "affects",
            "effect",
            "effected",
            "effecting",
            "effects",
        ]));

        let word_then_affect_or_effect = SequenceExpr::default()
            .then_any_word()
            .t_ws()
            .then(affect_or_effect.clone());

        let affect_or_effect_then_word = SequenceExpr::default()
            .then(affect_or_effect.clone())
            .t_ws()
            .then_any_word();

        let word_then_affect_or_effect_then_word = SequenceExpr::default()
            .then_any_word()
            .t_ws()
            .then(affect_or_effect)
            .t_ws()
            .then_any_word();

        Self {
            expr: Box::new(
                // SequenceExpr::default()
                //     .then_any_word()
                //     .t_ws()
                //     .then(affect_or_effect)
                //     .t_ws()
                //     .then_any_word(),
                LongestMatchOf::new(vec![
                    Box::new(word_then_affect_or_effect),
                    Box::new(word_then_affect_or_effect_then_word),
                    Box::new(affect_or_effect_then_word),
                ]),
            ),
        }
    }
}

impl ExprLinter for AffectEffect {
    fn expr(&self) -> &dyn Expr {
        self.expr.as_ref()
    }

    fn match_to_lint(&self, toks: &[Token], src: &[char]) -> Option<Lint> {
        None
    }

    fn description(&self) -> &'static str {
        "Fixes mixups between `affect` and `effect`."
    }
}

#[cfg(test)]
mod tests {
    use crate::linting::{AffectEffect, tests::assert_lint_count};

    #[test]
    fn dont_flag_lone_affect() {
        assert_lint_count("affect", AffectEffect::default(), 0);
    }

    #[test]
    fn dont_flag_lone_affected() {
        assert_lint_count("affected", AffectEffect::default(), 0);
    }

    #[test]
    fn dont_flag_lone_affecting() {
        assert_lint_count("affecting", AffectEffect::default(), 0);
    }

    #[test]
    fn dont_flag_lone_affects() {
        assert_lint_count("affects", AffectEffect::default(), 0);
    }
}
