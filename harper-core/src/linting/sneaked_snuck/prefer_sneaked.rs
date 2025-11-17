use crate::expr::Expr;
use crate::linting::{ExprLinter, LintKind, Suggestion};
use crate::patterns::Word;
use crate::{Lint, Token};

pub struct PreferSneaked {
    expr: Box<dyn Expr>,
}

impl Default for PreferSneaked {
    fn default() -> Self {
        Self {
            expr: Box::new(Word::new("snuck")),
        }
    }
}

impl ExprLinter for PreferSneaked {
    fn expr(&self) -> &dyn Expr {
        self.expr.as_ref()
    }

    fn match_to_lint(&self, toks: &[Token], src: &[char]) -> Option<Lint> {
        Some(Lint {
            span: toks[0].span,
            lint_kind: LintKind::Usage,
            suggestions: vec![Suggestion::replace_with_match_case(['s', 'n', 'e', 'a', 'k', 'e', 'd'].to_vec(), toks[0].span.get_content(src))],
            message: "Use `sneaked` instead of `snuck`.".to_string(),
            ..Default::default()
        })
    }

    fn description(&self) -> &str {
        "Prefer 'sneaked' over 'snuck'"
    }
}
