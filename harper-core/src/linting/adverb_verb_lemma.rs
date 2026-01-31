use crate::{
    CharStringExt, Lint, Token, TokenKind,
    expr::{Expr, SequenceExpr},
    linting::{ExprLinter, Suggestion, debug::format_lint_match, expr_linter::Chunk},
};

pub struct AdverbVerbLemma {
    expr: Box<dyn Expr>,
}

impl Default for AdverbVerbLemma {
    fn default() -> Self {
        Self {
            expr: Box::new(
                SequenceExpr::default()
                    .then_indefinite_article()
                    .t_ws()
                    .then(|tok: &Token, src: &[char]| {
                        tok.kind.is_manner_adverb()
                            && tok
                                .span
                                .get_content(src)
                                .ends_with_ignore_ascii_case_chars(&['l', 'y'])
                    })
                    .t_ws()
                    .then_kind_is_but_is_not(TokenKind::is_verb_lemma, TokenKind::is_adjective),
            ),
        }
    }
}

impl ExprLinter for AdverbVerbLemma {
    type Unit = Chunk;

    fn description(&self) -> &str {
        "Looks for and adverb of manner followed by a verb lemma"
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
        eprintln!("💋 {}", format_lint_match(toks, ctx, src));
        None
    }
}

#[cfg(test)]
mod tests {
    use super::AdverbVerbLemma;
    use crate::linting::tests::assert_suggestion_result;

    #[test]
    fn foobar() {
        assert_suggestion_result(
            "a highly request ability",
            AdverbVerbLemma::default(),
            "a highly requested ability",
        );
    }
}
