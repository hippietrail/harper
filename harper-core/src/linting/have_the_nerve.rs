use crate::{
    CharStringExt, Lint, Token, TokenStringExt,
    expr::{Expr, SequenceExpr},
    linting::{ExprLinter, Suggestion, debug::format_lint_match, expr_linter::Chunk},
};

pub struct HaveTheNerve {
    expr: Box<dyn Expr>,
}

impl Default for HaveTheNerve {
    fn default() -> Self {
        Self {
            expr: Box::new(
                SequenceExpr::word_set(&["had", "have", "having", "has"])
                    .t_ws()
                    .t_aco("the")
                    .t_ws()
                    .then_word_set(&["nerve", "nerves"])
                    .t_ws()
                    .t_aco("to"),
            ),
        }
    }
}

impl ExprLinter for HaveTheNerve {
    type Unit = Chunk;

    fn description(&self) -> &str {
        "Flags `have the nerves` used for audacity or `have the nerve` used for patience."
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
        let apprehensive_emoji = "😰";
        let rude_emoji = "😠";

        // I can't believe they had the nerve to do that! -> rude emoji
        // Oh I don't have the nerves for that kind of thing! -> apprehensive emoji
        let emoji = if toks
            // .get(toks.len().checked_sub(3)?)?
            .get_rel(-3)?
            .span
            .get_content(src)
            .ends_with_ignore_ascii_case_chars(&['s'])
        {
            apprehensive_emoji
        } else {
            rude_emoji
        };

        eprintln!("{emoji} {}", format_lint_match(toks, ctx, src));
        None
    }
}
