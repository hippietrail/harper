use crate::{Dialect, Dictionary, Document, Span, TokenStringExt};

use super::{Lint, LintKind, Linter, Suggestion};

pub struct InflectedVerbAfterTo<T>
where
    T: Dictionary,
{
    dictionary: T,
    dialect: Dialect,
}

impl<T: Dictionary> InflectedVerbAfterTo<T> {
    pub fn new(dictionary: T, dialect: Dialect) -> Self {
        Self {
            dictionary,
            dialect,
        }
    }
}

impl<T: Dictionary> Linter for InflectedVerbAfterTo<T> {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut lints = Vec::new();
        for pi in document.iter_preposition_indices() {
            let prep = document.get_token(pi).unwrap();
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

            if chars.len() < 4 {
                continue;
            }

            let mut check_stem = |stem: &[char]| {
                if let Some(metadata) = self.dictionary.get_word_metadata(stem) {
                    if metadata.is_verb() && !metadata.is_noun() {
                        lints.push(Lint {
                            span: Span::new(prep.span.start, word.span.end),
                            lint_kind: LintKind::WordChoice,
                            message: "The base form of the verb is needed here.".to_string(),
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
                }
            };

            if chars.ends_with(&['e', 'd']) {
                check_stem(&chars[..chars.len() - 2]);
                check_stem(&chars[..chars.len() - 1]);
            }
            if chars.ends_with(&['e', 's']) {
                check_stem(&chars[..chars.len() - 2]);
            }
            if chars.ends_with(&['s']) {
                check_stem(&chars[..chars.len() - 1]);
            }
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
    use crate::{
        Dialect, FstDictionary,
        linting::tests::{assert_lint_count, assert_suggestion_result},
    };

    #[test]
    fn dont_flag_to_check_both_verb_and_noun() {
        assert_lint_count(
            "to check",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            0,
        );
    }

    #[test]
    fn dont_flag_to_checks_both_verb_and_noun() {
        assert_lint_count(
            "to checks",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            0,
        );
    }

    #[test]
    fn dont_flag_to_cheques_not_a_verb() {
        assert_lint_count(
            "to cheques",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            0,
        );
    }

    // -ing forms can act as nouns, current heuristics cannot distinguish
    // #[test]
    // fn flag_to_checking() {
    //     assert_lint_count("to checking", InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American), 1);
    // }

    #[test]
    fn dont_flag_check_ed() {
        assert_lint_count(
            "to checked",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            0,
        );
    }

    #[test]
    fn dont_flag_noun_belief_s() {
        assert_lint_count(
            "to beliefs",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            0,
        );
    }

    #[test]
    fn dont_flag_noun_meat_s() {
        assert_lint_count(
            "to meats",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            0,
        );
    }

    // can't check yet. 'capture' is noun as well as verb. "to nouns" is good English. we can't disambiguate verbs from nouns.
    // #[test]
    // fn check_993_suggestions() {
    //     assert_suggestion_result(
    //         "A location-agnostic structure that attempts to captures the context and content that a Lint occurred.",
    //         InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
    //         "A location-agnostic structure that attempts to capture the context and content that a Lint occurred.",
    //     );
    // }

    #[test]
    fn dont_flag_embarrass_not_in_dictionary() {
        assert_lint_count(
            "Second I'm going to embarrass you for a.",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            0,
        );
    }

    #[test]
    fn corrects_exist_s() {
        assert_suggestion_result(
            "A valid solution is expected to exists.",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            "A valid solution is expected to exist.",
        );
    }

    // can't check yet. 'catch' is noun as well as verb. "to nouns" is good English. we can't disambiguate verbs from nouns.
    // #[test]
    // fn corrects_es_ending() {
    //     assert_suggestion_result(
    //         "I need it to catches every exception.",
    //         InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
    //         "I need it to catch every exception.",
    //     );
    // }

    #[test]
    fn corrects_ed_ending() {
        assert_suggestion_result(
            "I had to expanded my horizon.",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            "I had to expand my horizon.",
        );
    }

    #[test]
    fn flags_expire_d() {
        assert_lint_count(
            "I didn't know it was going to expired.",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            1,
        );
    }

    #[test]
    fn corrects_explain_ed() {
        assert_suggestion_result(
            "To explained the rules to the team.",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            "To explain the rules to the team.",
        );
    }

    // can't check yet. surprisingly, 'explore' is noun as well as verb. "to nouns" is good English. we can't disambiguate verbs from nouns.
    // #[test]
    // fn corrects_explor_ed() {
    //     assert_suggestion_result(
    //         "I went to explored distant galaxies.",
    //         InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
    //         "I went to explore distant galaxies.",
    //     );
    // }

    #[test]
    fn cant_flag_express_ed_also_noun() {
        assert_lint_count(
            "I failed to clearly expressed my point.",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            0,
        );
    }

    #[test]
    fn correct_feign_ed() {
        assert_suggestion_result(
            "I was able to feigned ignorance.",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            "I was able to feign ignorance.",
        );
    }
}
