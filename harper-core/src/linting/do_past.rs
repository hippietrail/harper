use crate::expr::{Expr, SequenceExpr};
use crate::linting::{ExprLinter, Lint};
use crate::TokenStringExt;
use crate::Token;

pub struct DoPast {
    expr: Box<dyn Expr>,
}

impl Default for DoPast {
    fn default() -> Self {
        Self {
            expr: Box::new(
                SequenceExpr::aco("did").t_ws().then_verb_lemma()
            ),
        }
    }
}

impl ExprLinter for DoPast {
    fn expr(&self) -> &dyn Expr {
        self.expr.as_ref()
    }

    fn match_to_lint(&self, toks: &[Token], src: &[char]) -> Option<Lint> {
        Some(Lint {
            span: toks.span()?,
            ..Default::default()
        })
    }

    fn description(&self) -> &'static str {
        "Detects 'do past'"
    }
}