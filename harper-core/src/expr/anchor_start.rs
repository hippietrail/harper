use crate::{Token, TokenKind, TokenStringExt};

use super::Step;

/// A [`Step`] which will match only if the cursor is over the first word-like of a token stream.
/// It will return that token.
pub struct AnchorStart;

impl Step for AnchorStart {
    fn step(&self, tokens: &[Token], cursor: usize, _source: &[char]) -> Option<isize> {
        // if tokens.iter_word_like_indices().next() == Some(cursor) {
        //     Some(0)
        // } else {
        //     None
        // }
        for (i, token) in tokens.iter().enumerate() {
            if token.kind.is_word_like() {
                return if i == cursor { Some(0) } else { None };
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::{Expr, ExprExt, SequenceExpr};
    use crate::linting::tests::assert_suggestion_result;
    use crate::linting::{ExprLinter, Lint, Suggestion};
    use crate::{Document, Span, Token, TokenKind, TokenStringExt};

    use super::AnchorStart;

    #[test]
    fn matches_first_word() {
        let document = Document::new_markdown_default_curated("This is a test.");
        let matches: Vec<_> = AnchorStart.iter_matches_in_doc(&document).collect();

        assert_eq!(matches, vec![Span::new(0, 0)])
    }

    #[test]
    fn does_not_match_empty() {
        let document = Document::new_markdown_default_curated("");
        let matches: Vec<_> = AnchorStart.iter_matches_in_doc(&document).collect();

        assert_eq!(matches, vec![])
    }

    pub struct Start {
        expr: Box<dyn Expr>,
    }

    impl Default for Start {
        fn default() -> Self {
            Self {
                expr: Box::new(
                    SequenceExpr::default().then(AnchorStart)
                    // .then_kind_any_except(&[TokenKind::is_word_like], &["START"])
                    // .then_any_word()
                    .then(|tok: &Token, src: &[char]| {
                        tok.kind.is_word_like() && 
                        tok.span.get_content_string(src) != "START"
                    })
                ),
            }
        }
    }

    impl ExprLinter for Start {
        fn expr(&self) -> &dyn Expr {
            self.expr.as_ref()
        }

        fn match_to_lint(&self, toks: &[Token], src: &[char]) -> Option<Lint> {
            eprintln!("💚 {} 💚", toks.span()?.get_content_string(src));
            let span = toks[0].span;
            // if span.get_content(src) == &['S', 'T', 'A', 'R', 'T'] {
            //     return None;
            // }
            Some(Lint {
                span: span,
                suggestions: vec![Suggestion::ReplaceWith("START".chars().collect())],
                ..Default::default()
            })
        }

        fn description(&self) -> &str {
            "Testing `AnchorStart`."
        }
    }

    #[test]
    fn one_word() {
        assert_suggestion_result(
            "foo",
            Start::default(),
            "START",
        );
    }

    #[test]
    fn two_words() {
        assert_suggestion_result(
            "foo bar",
            Start::default(),
            "START bar",
        );
    }

    #[test]
    fn flags_single_token() {
        assert_suggestion_result(
            "Hello, world! One two three four five.",
            Start::default(),
            "START, START! START two three four five.",
        );
    }
}
