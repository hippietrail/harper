use crate::{
    Lint, Token, TokenKind,
    expr::{All, Expr, FirstMatchOf, OwnedExprExt, SequenceExpr},
    linting::{ExprLinter, LintKind, Suggestion, debug::format_lint_match, expr_linter::Chunk},
    patterns::ModalVerb,
};

pub struct RelativeClauseNounVerbAgreement {
    expr: Box<dyn Expr>,
}

impl Default for RelativeClauseNounVerbAgreement {
    fn default() -> Self {
        Self {
            // Flags: types that eats
            // Flags: type that eat
            // But not: type that can
            expr: Box::new(All::new(vec![
                Box::new(
                    SequenceExpr::default()
                        .then_noun()
                        .t_ws()
                        .then_word_set(&["that", "which"])
                        .t_ws()
                        .then_verb(),
                ),
                Box::new(FirstMatchOf::new(vec![
                    Box::new(
                        SequenceExpr::default()
                            .then_plural_noun()
                            .t_any()
                            .t_any()
                            .t_any()
                            .then_verb_third_person_singular_present_form(),
                    ),
                    Box::new(
                        SequenceExpr::default()
                            .then_singular_noun()
                            .t_any()
                            .t_any()
                            .t_any()
                            // .then_verb_lemma()
                            .then_kind_is_but_is_not(TokenKind::is_verb_lemma, TokenKind::is_noun)
                            .and_not(
                                SequenceExpr::anything()
                                    .t_any()
                                    .t_any()
                                    .t_any()
                                    .then(ModalVerb::with_common_errors()),
                            ),
                    ),
                ])),
            ])),
        }
    }
}

impl ExprLinter for RelativeClauseNounVerbAgreement {
    type Unit = Chunk;

    fn description(&self) -> &str {
        "Fixes grammatical number of the noun not matching that of the verb in a relative clause."
    }

    fn expr(&self) -> &dyn Expr {
        self.expr.as_ref()
    }

    fn match_to_lint_with_context(
        &self,
        toks: &[Token],
        src: &[char],
        ctx: Option<(&[Token], &[Token])>,
    ) -> Option<Lint> {
        eprintln!("🌼 {}", format_lint_match(toks, ctx, src));
        None
    }
}

#[cfg(test)]
mod tests {
    use super::RelativeClauseNounVerbAgreement;
    use crate::linting::tests::assert_suggestion_result;

    #[test]
    fn fix_plural_that_has() {
        assert_suggestion_result(
            "Here are some of the file types that has built-in support.",
            RelativeClauseNounVerbAgreement::default(),
            "Here are some of the file types that have built-in support.",
        );
    }
}
