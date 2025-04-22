use crate::{Dialect, Dictionary, Document, Span, Token, TokenStringExt};

use super::{Lint, LintKind, Linter, Suggestion};

/// Get the next or previous word token relative to a base index, iff separated by whitespace.
/// Returns None if the next/previous token is not a word or does not exist.
fn getword<'a>(document: &'a Document, base: usize, offset: isize) -> Option<&'a Token> {
    // Look for whitespace at the expected offset
    if !document
        .get_token_offset(base, offset)?
        .kind
        .is_whitespace()
    {
        return None;
    }
    // Now look beyond the whitespace for a word token
    let word_token = document.get_token_offset(base, offset + offset.signum());
    let word_token = word_token?;
    word_token.kind.is_word().then_some(word_token)
}

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
            let prep_span = document.get_token(pi).unwrap().span;
            let prep_to = document.get_span_content(&prep_span);
            if prep_to != ['t', 'o'] && prep_to != ['T', 'o'] {
                continue;
            }

            let Some(main_word) = getword(document, pi, 1) else {
                continue;
            };

            let chars = document.get_span_content(&main_word.span);

            // Must end with -ed, -es, -s, or -ing
            let has_applicable_ending = (chars.ends_with(&['e', 'd']) && chars.len() >= 5) ||   // baked
                (chars.ends_with(&['e', 's']) && chars.len() >= 4) ||   // does
                (chars.ends_with(&['i', 'n', 'g']) && chars.len() >= 5) ||   // going
                (chars.ends_with(&['s']) && chars.len() >= 4); // runs

            if !has_applicable_ending {
                // eprintln!(
                //     "*** '{}' doesn't look like an inflected verb ***",
                //     chars.iter().collect::<String>()
                // );
                continue;
            }

            // eprintln!(
            //     "*** \"TO {}\" ***",
            //     document
            //         .get_span_content(&main_word.span)
            //         .iter()
            //         .collect::<String>(),
            // );

            let mut push_lint = |prep_span: &Span, main_word_span: &Span, stem: &[char]| {
                lints.push(Lint {
                    span: Span::new(prep_span.start, main_word_span.end),
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
            };

            // outputs a string made up of the word with appended parts of speech
            let annotate_word = |word: &[char]| -> String {
                let mut str = word.iter().collect::<String>();
                if let Some(wmd) = self.dictionary.get_word_metadata(word) {
                    // .is_noun and .is_verb -> .n and .v
                    let tags = [
                        ("n", wmd.is_noun()),
                        ("v", wmd.is_verb()),
                        ("adj", wmd.is_adjective()),
                        ("adv", wmd.is_adverb()),
                        ("prep", wmd.preposition),
                        ("conj", wmd.is_conjunction()),
                        ("det", wmd.determiner),
                    ]
                    .into_iter()
                    .filter_map(|(tag, is_tagged)| is_tagged.then(|| format!(".{}", tag)))
                    .collect::<Vec<_>>();
                    str.push_str(&tags.join(""));
                }
                str
            };

            let mut check_stem = |stem: &[char]| {
                if let Some(stem_metadata) = self.dictionary.get_word_metadata(stem) {
                    let pw_text = getword(document, pi, -1)
                        .map(|pw| document.get_span_content(&pw.span))
                        .unwrap_or_default();
                    let nw_text = getword(document, pi, 3)
                        .map(|nw| document.get_span_content(&nw.span))
                        .unwrap_or_default();
                    eprintln!(
                        "*** {} \x1b[1m{} {}\x1b[0m{}{}\" {} ***\n",
                        annotate_word(&pw_text),
                        prep_to.iter().collect::<String>(),
                        document
                            .get_span_content(&main_word.span)
                            .iter()
                            .collect::<String>(),
                        stem_metadata.is_verb().then(|| ".v").unwrap_or_default(),
                        stem_metadata.is_noun().then(|| ".n").unwrap_or_default(),
                        annotate_word(&nw_text),
                    );

                    if stem_metadata.is_verb() {
                        // heuristics go here
                        if !stem_metadata.is_noun() {
                            push_lint(&prep_span, &main_word.span, &stem);
                        }
                    }
                }
            };

            if chars.ends_with(&['e', 'd']) {
                check_stem(&chars[..chars.len() - 2]); // waited -> wait
                check_stem(&chars[..chars.len() - 1]); // resumed -> resume
            }
            if chars.ends_with(&['e', 's']) {
                check_stem(&chars[..chars.len() - 2]); // does -> do
            }
            if chars.ends_with(&['s']) {
                check_stem(&chars[..chars.len() - 1]); // runs -> run
            }
            if chars.ends_with(&['i', 'n', 'g']) {
                if chars.len() > 3 {
                    check_stem(&chars[..chars.len() - 3]); // doing -> do
                    let stem = &chars[..chars.len() - 3];
                    let mut stem_with_e = stem.to_vec();
                    stem_with_e.push('e');
                    check_stem(&stem_with_e); // coming -> come
                }
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
            "from cash to check",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            0,
        );
    }

    #[test]
    fn dont_flag_to_checks_both_verb_and_noun() {
        assert_lint_count(
            "from cash to checks",
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
    #[test]
    fn flag_to_checking() {
        assert_lint_count(
            "from savings account to checking",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            0,
        );
    }

    // -ed forms can act as adjectives, current heuristics cannot distinguish
    #[test]
    fn dont_flag_check_ed() {
        assert_lint_count(
            "from unchecked to checked",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            0,
        );
    }

    #[test]
    fn dont_flag_noun_belief_s() {
        assert_lint_count(
            "from hopes to beliefs",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            0,
        );
    }

    #[test]
    fn dont_flag_noun_meat_s() {
        assert_lint_count(
            "from vegetables to meats",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            0,
        );
    }

    #[test]
    fn check_993_suggestions() {
        // "captures" is a verb because "to" is part of "attempt to"
        assert_suggestion_result(
            "A location-agnostic structure that attempts to captures the context and content that a Lint occurred.",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            "A location-agnostic structure that attempts to capture the context and content that a Lint occurred.",
        );
    }

    #[test]
    fn dont_flag_embarrass_not_in_dictionary() {
        // "embarrass" is not inflected, it just ends with -s
        assert_lint_count(
            "Second I'm going to embarrass you for a.",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            0,
        );
    }

    #[test]
    fn corrects_exist_s() {
        // "exists" is a verb because "to" is part of "expect to"
        assert_suggestion_result(
            "A valid solution is expected to exists.",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            "A valid solution is expected to exist.",
        );
    }

    #[test]
    fn corrects_es_ending() {
        // "catches" is a verb because "to" is part of "need (it) to"
        assert_suggestion_result(
            "I need it to catches every exception.",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            "I need it to catch every exception.",
        );
    }

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
    #[test]
    fn corrects_explor_ed() {
        // "explored distant galaxies" could be a type of galaxies.
        assert_suggestion_result(
            "I went to explored distant galaxies.",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            "I went to explore distant galaxies.",
        );
    }

    // #[test]
    fn cant_flag_express_ed_also_noun() {
        assert_lint_count(
            "I failed to clearly expressed my point.",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            1,
        );
    }

    #[test]
    fn correct_feign_ed() {
        // In "feigned ignorance" "feigned" would be an adjective, but after "able to" it is a verb.
        assert_suggestion_result(
            "I was able to feigned ignorance.",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            "I was able to feign ignorance.",
        );
    }

    #[test]
    fn dont_correct_encrypted_data() {
        // Here 'encrypted' acts as an adjective, not a verb.
        assert_lint_count(
            "... access to encrypted data",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            0,
        );
    }

    #[test]
    fn issue_1091_a() {
        assert_lint_count(
            "Authorities argue that they require lawful access to encrypted data to efficiently combat terrorism, organized crime, and exploitation.",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            0,
        );
    }

    #[test]
    fn issue_1091_b() {
        assert_lint_count(
            "High-profile legal battles, including Apple's refusal to unlock a terrorist's iPhone for the FBI in 2016, Microsoft's resistance to handing over user data stored overseas, and the ongoing Apple vs. United Kingdom case over access to encrypted iCloud backups, highlight the tensions and stakes involved in balancing privacy rights and security demands.",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            0,
        );
    }

    #[test]
    fn flag_to_going() {
        assert_lint_count(
            "to going",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            1,
        );
    }

    #[test]
    fn flag_to_coming() {
        assert_lint_count(
            "to coming",
            InflectedVerbAfterTo::new(FstDictionary::curated(), Dialect::American),
            1,
        );
    }
}
