use crate::Token;
use crate::TokenStringExt;
use crate::expr::{Expr, SequenceExpr};
use crate::linting::expr_linter::Chunk;
use crate::linting::{ExprLinter, Lint, LintKind, Suggestion};

pub struct WillNonLemma {
    expr: Box<dyn Expr>,
}

impl Default for WillNonLemma {
    fn default() -> Self {
        Self {
            expr: Box::new(
                SequenceExpr::word_set(&["will", "shall"])
                    .t_ws()
                    .then_kind_where(|kind| {
                        kind.is_verb() && !kind.is_verb_lemma() && !kind.is_noun()
                    }),
            ),
        }
    }
}

impl ExprLinter for WillNonLemma {
    type Unit = Chunk;

    fn expr(&self) -> &dyn Expr {
        self.expr.as_ref()
    }

    fn match_to_lint_with_context(
        &self,
        toks: &[Token],
        src: &[char],
        ctx: Option<(&[Token], &[Token])>,
    ) -> Option<Lint> {
        let (pre, post) = match ctx {
            Some((prev, next)) => (
                prev.iter()
                    .map(|t| t.span.get_content_string(src))
                    .collect::<String>(),
                next.iter()
                    .map(|t| t.span.get_content_string(src))
                    .collect::<String>(),
            ),
            None => (String::new(), String::new()),
        };

        eprintln!(
            "❤️ \x1b[31m{}\x1b[0m{}\x1b[32m{}\x1b[0m",
            pre,
            toks.span()?.get_content_string(src),
            post
        );

        Some(Lint {
            span: toks.span()?,
            lint_kind: LintKind::Grammar,
            message: "`Will` should be followed by a verb in its lemma form.".to_string(),
            ..Default::default()
        })
    }

    fn description(&self) -> &str {
        "Flags wrong verb forms after `will` or `shall`"
    }
}

#[cfg(test)]
mod tests {
    use super::WillNonLemma;
    use crate::linting::tests::{
        assert_good_and_bad_suggestions, assert_lint_count, assert_suggestion_result,
    };

    #[test]
    fn fix_will_ran() {
        assert_good_and_bad_suggestions(
            "The brown fox will ran thru the meadow.",
            WillNonLemma::default(),
            &[
                "The brown fox will run thru the meadow.",
                "The brown fox ran thru the meadow.",
            ],
            &[],
        );
    }

    #[test]
    fn fix_will_exists() {
        assert_good_and_bad_suggestions(
            "there is a good chance duplicate Rule IDs will exists.",
            WillNonLemma::default(),
            &[
                "there is a good chance duplicate Rule IDs will exist.",
                "there is a good chance duplicate Rule IDs exists.",
                "there is a good chance duplicate Rule IDs exist.",
            ],
            &[],
        );
    }

    #[test]
    fn ignore_shall_vessels() {
        assert_lint_count(
            "No Preference shall be given by any Regulation of Commerce or Revenue to the Ports of one State over those of another; nor shall Vessels bound to, or from, one State, be obliged to enter, clear, or pay Duties in another.",
            WillNonLemma::default(),
            0,
        );
    }

    #[test]
    fn ignore_will_tools() {
        assert_lint_count("Give your AI free will tools.", WillNonLemma::default(), 0);
    }

    #[test]
    fn fix_will_coming_soon() {
        assert_good_and_bad_suggestions(
            "More advanced features will coming soon, so stay tuned!",
            WillNonLemma::default(),
            &[],
            &[
                "More advanced features will come soon, so stay tuned!",
                "More advanced features coming soon, so stay tuned!",
                "More advanced features will be coming soon, so stay tuned!",
            ],
        );
    }

    // on CPU and GPU (NPU support will coming next)
    fn fix_will_coming_next() {
        assert_good_and_bad_suggestions(
            "on CPU and GPU (NPU support will coming next)",
            WillNonLemma::default(),
            &[
                "on CPU and GPU (NPU support will come next)",
                "on CPU and GPU (NPU support coming next)",
                "on CPU and GPU (NPU support will be coming next)",
            ],
            &[],
        );
    }
}
