use crate::Token;
use crate::token_string_ext::TokenStringExt;

use super::Step;

/// A [`Step`] which will match only if the cursor is over the last non-whitespace character in stream.
/// It will return that token.
///
/// For example, if you built `SequenceExpr::default().t_aco("word").then(AnchorEnd)` and ran it on `This is a word`, the resulting `Span` would only cover the final word token.
pub struct AnchorEnd;

impl Step for AnchorEnd {
    fn step(&self, tokens: &[Token], cursor: usize, _source: &[char]) -> Option<isize> {
        eprint!("👀 {} / {} : '{}'", cursor, tokens.len(), tokens.span()?.get_content_string(_source));
        // at the end of a chunk without a chunk terminator?
        if cursor == tokens.len() {
            eprintln!("✅a");
            Some(0)
        // TODO at the end of a chunk with a chunk terminator?
        } else if cursor == tokens.len() - 1 {
            eprintln!("✅b");
            eprintln!("{:#?}", tokens.last());
            eprintln!("{:#?}", tokens.last().unwrap().kind.is_chunk_terminator());
            Some(0)
        } else {
            eprintln!("❌");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::{Expr, SequenceExpr};
    use crate::linting::tests::assert_suggestion_result;
    use crate::linting::{ExprLinter, Lint, Suggestion};
    use crate::{Token, TokenKind, TokenStringExt};

    use super::AnchorEnd;

    // #[test]
    // fn matches_period() {
    //     let document = Document::new_markdown_default_curated("This is a test.");
    //     let matches: Vec<_> = AnchorEnd.iter_matches_in_doc(&document).collect();

    //     assert_eq!(matches, vec![Span::new(7, 7)])
    // }

    // #[test]
    // fn does_not_match_empty() {
    //     let document = Document::new_markdown_default_curated("");
    //     let matches: Vec<_> = AnchorEnd.iter_matches_in_doc(&document).collect();

    //     assert_eq!(matches, vec![])
    // }

    pub struct End {
        expr: Box<dyn Expr>,
    }

    impl Default for End {
        fn default() -> Self {
            Self {
                expr: Box::new(
                    SequenceExpr::default()
                    .then_kind_any_except(
                        &[TokenKind::is_word_like],
                        &["END"]
                    ).then(AnchorEnd)
                ), // Only replaces the very first word
                // expr: Box::new(SequenceExpr::default().then(AnchorEnd).then_any_word()), // Doesn't match anything!
                // expr: Box::new(SequenceExpr::default().then(AnchorEnd)), // Doesn't match anything!
            }
        }
    }

    impl ExprLinter for End {
        fn expr(&self) -> &dyn Expr {
            self.expr.as_ref()
        }

        fn match_to_lint(&self, toks: &[Token], src: &[char]) -> Option<Lint> {
            let span = toks[0].span;
            if span.get_content(src) == &['E', 'N', 'D'] {
                return None;
            }
            eprintln!("❤️ {} ❤️", toks.span()?.get_content_string(src));
            Some(Lint {
                span,
                suggestions: vec![Suggestion::ReplaceWith("END".chars().collect())],
                ..Default::default()
            })
        }

        fn description(&self) -> &str {
            "Testing `AnchorEnd`."
        }
    }

    #[test]
    fn just_one_word() {
        println!("\n=== Test: just_one_word ===");
        let result = assert_suggestion_result(
            "foo",
            End::default(),
            "END",
        );
        println!("Test 'just_one_word' completed with result: {:?}", result);
    }

    #[test]
    fn one_word_after_space() {
        println!("\n=== Test: one_word_after_space ===");
        let result = assert_suggestion_result(
            " foo",
            End::default(),
            " END",
        );
        println!("Test 'one_word_after_space' completed with result: {:?}", result);
    }

    #[test]
    fn two_words() {
        println!("\n=== Test: two_words ===");
        let result = assert_suggestion_result(
            "foo bar",
            End::default(),
            "foo END",
        );
        println!("Test 'two_words' completed with result: {:?}", result);
    }

    #[test]
    fn flags_single_token() {
        println!("\n=== Test: flags_single_token ===");
        let result = assert_suggestion_result(
            "Hello, World! One two three four five.",
            End::default(),
            "END, END! One two three four END.",
        );
        println!("Test 'flags_single_token' completed with result: {:?}", result);
    }
}
