use super::{Lint, LintKind, Linter, Suggestion};
use crate::TokenStringExt;
use crate::{Document, Token, TokenKind};

#[derive(Debug, Default)]
pub struct Spaces;

impl Linter for Spaces {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut output = Vec::new();

        for sentence in document.iter_sentences() {
            for space_idx in sentence.iter_space_indices() {
                if space_idx == 0 {
                    continue;
                }

                let space = &sentence[space_idx];

                let TokenKind::Space(count) = space.kind else {
                    panic!("The space iterator should only return spaces.")
                };

                if count > 1 {
                    output.push(Lint {
                        span: space.span,
                        lint_kind: LintKind::Formatting,
                        suggestions: vec![Suggestion::ReplaceWith(vec![' '])],
                        message: format!(
                            "There are {count} spaces where there should be only one."
                        ),
                        priority: 15,
                    })
                }
            }

            if matches!(
                sentence,
                [
                    ..,
                    Token {
                        kind: TokenKind::Word(_),
                        ..
                    },
                    Token {
                        kind: TokenKind::Space(_),
                        ..
                    },
                    Token {
                        kind: TokenKind::Punctuation(_),
                        ..
                    }
                ]
            ) {
                output.push(Lint {
                    span: sentence[sentence.len() - 2..sentence.len() - 1]
                        .span()
                        .unwrap(),
                    lint_kind: LintKind::Formatting,
                    suggestions: vec![Suggestion::Remove],
                    message: "Unnecessary space at the end of the sentence.".to_string(),
                    priority: 63,
                })
            }
        }

        output
    }

    fn description(&self) -> &'static str {
        "Words should be separated by at most one space."
    }
}

#[cfg(test)]
mod tests {
    use super::Spaces;
    use crate::linting::tests::{assert_lint_count, assert_no_lints};

    #[test]
    fn detects_space_before_period() {
        let source = "There is a space at the end of this sentence .";

        assert_lint_count(source, Spaces, 1)
    }

    #[test]
    fn allows_period_without_space() {
        let source = "There isn't a space at the end of this sentence.";

        assert_lint_count(source, Spaces, 0)
    }

    #[test]
    fn ignores_french_spacing() {
        assert_no_lints(
            "This is a short sentence.  This is another short sentence.",
            Spaces,
        );
    }
}
