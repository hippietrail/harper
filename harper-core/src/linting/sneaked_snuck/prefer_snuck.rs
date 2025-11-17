use crate::expr::Expr;
use crate::linting::{ExprLinter, LintKind, Suggestion};
use crate::patterns::Word;
use crate::{Lint, Token};

pub struct PreferSnuck {
    expr: Box<dyn Expr>,
}

impl Default for PreferSnuck {
    fn default() -> Self {
        Self {
            expr: Box::new(Word::new("sneaked")),
        }
    }
}

impl ExprLinter for PreferSnuck {
    fn expr(&self) -> &dyn Expr {
        self.expr.as_ref()
    }

    fn match_to_lint(&self, toks: &[Token], src: &[char]) -> Option<Lint> {
        Some(Lint {
            span: toks[0].span,
            lint_kind: LintKind::Usage,
            suggestions: vec![Suggestion::replace_with_match_case(['s', 'n', 'u', 'c', 'k'].to_vec(), toks[0].span.get_content(src))],
            message: "Use `snuck` instead of `sneaked`.".to_string(),
            ..Default::default()
        })
    }

    fn description(&self) -> &str {
        "Prefer `snuck` over `sneaked`."
    }
}
