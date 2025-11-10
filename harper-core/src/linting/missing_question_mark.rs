use super::{Lint, Linter, Suggestion};
use crate::linting::LintKind;
use crate::{CharStringExt, Document, TokenStringExt};

#[derive(Default)]
pub struct MissingQuestionMark;

impl MissingQuestionMark {}

impl Linter for MissingQuestionMark {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut lints = Vec::new();
        for token_slice in document.iter_sentences() {
            if token_slice.is_empty() {
                continue;
            }
            let sentence = if token_slice.first().unwrap().kind.is_whitespace() {
                // Find the first non-whitespace token
                token_slice
                    .iter()
                    .position(|t| !t.kind.is_whitespace())
                    .map(|i| &token_slice[i..])
                    .unwrap_or(&token_slice[token_slice.len()..]) // handle case where all tokens are whitespace
            } else {
                token_slice
            };

            if let Some(_sspan) = sentence.span() {
                let tok1 = sentence.first().unwrap();
                if !tok1.kind.is_word() {
                    continue;
                }
                let w1 = document.get_span_content(&tok1.span);
                if !w1.eq_any_ignore_ascii_case_chars(&[
                    // Interrogatives
                    &['w', 'h', 'a', 't'],
                    &['w', 'h', 'e', 'n'],
                    &['w', 'h', 'e', 'r', 'e'],
                    &['w', 'h', 'i', 'c', 'h'],
                    &['w', 'h', 'o'],
                    &['h', 'o', 'w'],
                    // Auxiliary verbs (do)
                    &['d', 'o'],
                    &['d', 'o', 'n', '\'', 't'],
                    &['d', 'o', 'e', 's'],
                    &['d', 'o', 'e', 's', 'n', '\'', 't'],
                    &['d', 'i', 'd'],
                    &['d', 'i', 'd', 'n', '\'', 't'],
                    // Auxiliary verbs (be)
                    &['a', 'm'],
                    &['a', 'r', 'e'],
                    &['a', 'r', 'e', 'n', '\'', 't'],
                    &['i', 's'],
                    &['i', 's', 'n', '\'', 't'],
                    &['w', 'a', 's'],
                    &['w', 'a', 's', 'n', '\'', 't'],
                    &['w', 'e', 'r', 'e'],
                    &['w', 'e', 'r', 'e', 'n', '\'', 't'],
                    // Modal verbs
                    &['c', 'a', 'n'],
                    &['c', 'a', 'n', 'n', '\'', 't'],
                    &['c', 'o', 'u', 'l', 'd'],
                    &['c', 'o', 'u', 'l', 'd', 'n', '\'', 't'],
                    &['m', 'a', 'y'],
                    &['m', 'i', 'g', 'h', 't'],
                    &['m', 'i', 'g', 'h', 't', 'n', '\'', 't'],
                    &['m', 'u', 's', 't'],
                    &['m', 'u', 's', 't', 'n', '\'', 't'],
                    &['s', 'h', 'o', 'u', 'l', 'd'],
                    &['s', 'h', 'o', 'u', 'l', 'd', 'n', '\'', 't'],
                    &['w', 'i', 'l', 'l'],
                    &['w', 'o', 'n', 't'],
                    &['w', 'o', 'u', 'l', 'd'],
                    &['w', 'o', 'u', 'l', 'd', 'n', '\'', 't'],
                ]) {
                    continue;
                }

                let tokn = sentence.last().unwrap();
                if tokn.kind.is_question() {
                    continue;
                }

                let suggestions = vec![if tokn.kind.is_punctuation() {
                    Suggestion::ReplaceWith(vec!['?'])
                } else {
                    Suggestion::InsertAfter(vec!['?'])
                }];

                lints.push(Lint {
                    span: tokn.span,
                    lint_kind: LintKind::Punctuation,
                    message: "Questions should end with a question mark".to_string(),
                    suggestions,
                    ..Default::default()
                })
            }
        }

        lints
    }

    fn description(&self) -> &str {
        "Missing question mark"
    }
}

#[cfg(test)]
mod tests {
    use super::MissingQuestionMark;
    use crate::linting::tests::{assert_no_lints, assert_suggestion_result};

    #[test]
    fn no_spaces() {
        assert_no_lints(
            "What is the meaning of life?",
            MissingQuestionMark::default(),
        );
    }

    #[test]
    fn starts_with_a_space() {
        assert_no_lints(" When is tomorrow?", MissingQuestionMark::default());
    }

    #[test]
    fn ends_with_a_space() {
        assert_no_lints("Where is the toilet? ", MissingQuestionMark::default());
    }

    #[test]
    fn starts_with_a_space_then_a_tab() {
        assert_suggestion_result(
            " \tWhich one of you does this",
            MissingQuestionMark::default(),
            " \tWhich one of you does this?",
        );
    }

    #[test]
    fn starts_with_a_space_then_a_tab_then_a_newline() {
        assert_suggestion_result(
            " \t\nHow are you",
            MissingQuestionMark::default(),
            " \t\nHow are you?",
        );
    }

    #[test]
    fn only_question_mark() {
        assert_no_lints("?", MissingQuestionMark::default());
    }

    #[test]
    fn only_whitespace_and_question_mark() {
        assert_no_lints(" \t\n?", MissingQuestionMark::default());
    }

    #[test]
    fn only_what() {
        assert_suggestion_result("What", MissingQuestionMark::default(), "What?");
    }

    #[test]
    fn what_question_mark() {
        assert_no_lints("What?", MissingQuestionMark::default());
    }
}
