use std::sync::Arc;

use crate::{Dictionary, Document, FstDictionary, Span, TokenStringExt};

use super::{Lint, LintKind, Linter, Suggestion};

pub struct InflectedVerbAfterTo {
    pub dictionary: Arc<FstDictionary>,
}

impl InflectedVerbAfterTo {
    pub fn new() -> Self {
        Self {
            dictionary: FstDictionary::curated(),
        }
    }
}

impl Default for InflectedVerbAfterTo {
    fn default() -> Self {
        Self::new()
    }
}

impl Linter for InflectedVerbAfterTo {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut lints = Vec::new();
        for pi in document.iter_preposition_indices() {
            let Some(prep) = document.get_token(pi) else {
                continue;
            };
            let Some(space) = document.get_token(pi + 1) else {
                continue;
            };
            let Some(word) = document.get_token(pi + 2) else {
                continue;
            };
            if !space.kind.is_whitespace() || !word.kind.is_word() {
                continue;
            }
            let prep_to = document.get_span_content(&prep.span);
            if prep_to != ['t', 'o'] && prep_to != ['T', 'o'] {
                continue;
            }

            let chars = document.get_span_content(&word.span);
            let (len, form) = match word.kind {
                _ if word.kind.is_verb() => match chars {
                    // [.., 'i', 'n', 'g'] => (3, "continuous"), // breaks the Laravel test at "prior to deploying the application"
                    [.., 'e', 'd'] => (2, "past"),
                    [.., 'e', 's'] => (2, "3rd person singular present"),
                    [.., 's'] => (1, "3rd person singular present"),
                    _ => continue,
                },
                _ if word.kind.is_plural_noun() => match chars {
                    [.., 'e', 's'] => (2, "plural"),
                    [.., 's'] => (1, "plural"),
                    _ => continue,
                },
                _ => continue,
            };
            let stem = chars[..chars.len() - len].to_vec();
            let Some(md) = self.dictionary.get_word_metadata(&stem) else {
                continue;
            };
            if !md.is_verb() {
                continue;
            }
            if word.kind.is_plural_noun() && md.is_noun() {
                continue;
            }

            lints.push(Lint {
                span: Span::new(prep.span.start, word.span.end),
                lint_kind: LintKind::WordChoice,
                message: format!("This verb seems to be in the {} form.", form).to_string(),
                suggestions: vec![Suggestion::ReplaceWith(
                    prep_to
                        .iter()
                        .chain([' '].iter())
                        .chain(stem.iter())
                        .copied()
                        .collect(),
                )],
                ..Default::default()
            });
        }
        lints
    }

    fn description(&self) -> &str {
        "This rule looks for `to verb` where `verb` is not in the infinitive form."
    }
}

#[cfg(test)]
mod tests {
    use super::InflectedVerbAfterTo;
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    #[test]
    fn dont_flag_to_check() {
        assert_lint_count("to check", InflectedVerbAfterTo::default(), 0);
    }

    #[test]
    fn dont_flag_to_checks() {
        assert_lint_count("to checks", InflectedVerbAfterTo::default(), 0);
    }

    #[test]
    fn dont_flag_to_cheques() {
        assert_lint_count("to cheques", InflectedVerbAfterTo::default(), 0);
    }

    // #[test]
    // fn flag_to_checking() {
    //     assert_lint_count("to checking", InflectedVerbAfterTo::default(), 1);
    // }

    #[test]
    fn flag_to_checked() {
        assert_lint_count("to checked", InflectedVerbAfterTo::default(), 1);
    }

    #[test]
    fn dont_flag_plural_noun() {
        assert_lint_count("to beliefs", InflectedVerbAfterTo::default(), 0);
    }

    #[test]
    fn dont_flag_plural_meats() {
        assert_lint_count("to meats", InflectedVerbAfterTo::default(), 0);
    }

    // #[test]
    // fn check_993_suggestions() {
    //     assert_suggestion_result(
    //         "A location-agnostic structure that attempts to captures the context and content that a Lint occurred.",
    //         InflectedVerbAfterTo::default(),
    //         "A location-agnostic structure that attempts to capture the context and content that a Lint occurred.",
    //     );
    // }

    #[test]
    fn dont_flag_plural_embarrass() {
        assert_lint_count(
            "Second I'm going to embarrass you for a.",
            InflectedVerbAfterTo::default(),
            0,
        );
    }
}
