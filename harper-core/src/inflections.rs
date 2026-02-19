/// Helper functions for converting between different inflected forms of words.
/// Handles regular patterns for nouns (singular/plural) and verbs (lemma/3rd person singular).
///
/// Mirrors the structure of `irregular_nouns.rs` and `irregular_verbs.rs` by centralizing
/// ad-hoc inflection logic scattered across multiple linters.

pub mod nouns {
    use crate::spell::Dictionary;

    /// Attempts to convert a singular noun to its plural form(s).
    /// Tries common patterns: +s, +es, y→ies, fe→ves.
    /// Returns only forms validated by the dictionary.
    pub fn singular_to_plural<D: Dictionary>(
        singular: &[char],
        dict: &D,
    ) -> Vec<Vec<char>> {
        let mut candidates = Vec::new();

        // +s
        let mut plural_s = singular.to_vec();
        plural_s.push('s');
        candidates.push(plural_s);

        // +es
        let mut plural_es = singular.to_vec();
        plural_es.extend(['e', 's']);
        candidates.push(plural_es);

        // y → ies
        if singular.ends_with(&['y']) {
            let mut plural_ies = singular[..singular.len() - 1].to_vec();
            plural_ies.extend(['i', 'e', 's']);
            candidates.push(plural_ies);
        }

        // fe → ves
        if singular.ends_with(&['f', 'e']) {
            let mut plural_ves = singular[..singular.len() - 2].to_vec();
            plural_ves.extend(['v', 'e', 's']);
            candidates.push(plural_ves);
        }

        // Filter to only those that are valid plurals in the dictionary
        candidates
            .into_iter()
            .filter(|word| {
                dict.get_word_metadata(word)
                    .is_some_and(|md| md.is_plural_noun())
            })
            .collect()
    }

    /// Attempts to convert a plural noun to its singular form(s).
    /// Tries common patterns: -s, -es, ies→y, ves→fe.
    /// Returns only forms validated by the dictionary.
    pub fn plural_to_singular<D: Dictionary>(
        plural: &[char],
        dict: &D,
    ) -> Vec<Vec<char>> {
        let mut candidates = Vec::new();

        if plural.ends_with(&['s']) {
            let stem = &plural[..plural.len() - 1];

            // -s → (base)
            candidates.push(stem.to_vec());

            if stem.ends_with(&['e']) {
                let stem2 = &stem[..stem.len() - 1];

                // -es → (base)
                candidates.push(stem2.to_vec());

                // -ies → -y
                if stem2.ends_with(&['i']) {
                    let stem3 = &stem2[..stem2.len() - 1];
                    let mut singular_y = stem3.to_vec();
                    singular_y.push('y');
                    candidates.push(singular_y);
                }

                // -ves → -fe
                if stem2.ends_with(&['v']) {
                    let stem3 = &stem2[..stem2.len() - 1];
                    let mut singular_fe = stem3.to_vec();
                    singular_fe.extend(['f', 'e']);
                    candidates.push(singular_fe);
                }
            }
        }

        // Filter to only those that are valid singulars in the dictionary
        candidates
            .into_iter()
            .filter(|word| {
                dict.get_word_metadata(word)
                    .is_some_and(|md| md.is_singular_noun())
            })
            .collect()
    }
}

/// Identifies the type of verb conjugation based on ending.
/// Used for pattern matching across multiple linters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerbConjugation {
    /// Base infinitive form (no ending)
    Lemma,
    /// Past tense: ends in -ed or -d
    PastTense,
    /// 3rd person singular present: ends in -s or -es
    ThirdPersonSingular,
    /// Present progressive: ends in -ing
    Progressive,
}

impl VerbConjugation {
    /// Identifies the conjugation type from the word's ending.
    /// Checks in order: -ing, -ed, -es, -s, then assumes lemma.
    pub fn identify(word: &[char]) -> Self {
        if word.ends_with(&['i', 'n', 'g']) {
            VerbConjugation::Progressive
        } else if word.ends_with(&['e', 'd']) {
            VerbConjugation::PastTense
        } else if word.ends_with(&['d']) && word.len() > 1 {
            // Single 'd' after vowel (e.g., "pad", "bid")
            VerbConjugation::PastTense
        } else if word.ends_with(&['e', 's']) {
            VerbConjugation::ThirdPersonSingular
        } else if word.ends_with(&['s']) {
            VerbConjugation::ThirdPersonSingular
        } else {
            VerbConjugation::Lemma
        }
    }
}

pub mod verbs {
    use crate::spell::Dictionary;
    use super::VerbConjugation;

    /// Attempts to convert a verb lemma to its 3rd person singular present form(s).
    /// Tries common patterns: +s, +es, y→ies.
    /// Returns only forms validated by the dictionary.
    pub fn lemma_to_3ps<D: Dictionary>(lemma: &str, dict: &D) -> Vec<Vec<char>> {
        let mut candidates: Vec<Vec<char>> = Vec::new();

        // +s
        candidates.push(format!("{lemma}s").chars().collect::<Vec<char>>());

        // +es
        candidates.push(format!("{lemma}es").chars().collect::<Vec<char>>());

        // y → ies
        if lemma.ends_with('y') {
            candidates.push(
                format!("{}ies", &lemma[0..lemma.len() - 1])
                    .chars()
                    .collect::<Vec<char>>(),
            );
        }

        // Filter to only those that are valid 3ps forms in the dictionary
        candidates
            .into_iter()
            .filter(|word| {
                dict.get_word_metadata(word)
                    .is_some_and(|md| md.is_verb_third_person_singular_present_form())
            })
            .collect()
    }

    /// Attempts to convert a 3rd person singular present verb form to its lemma (base form).
    /// Tries common patterns: -s, -es, ies→y.
    /// Returns only forms validated by the dictionary.
    pub fn ps3_to_lemma<D: Dictionary>(form: &[char], dict: &D) -> Vec<Vec<char>> {
        let mut candidates = Vec::new();

        if form.ends_with(&['s']) {
            let stem = &form[..form.len() - 1];
            candidates.push(stem.to_vec());

            if stem.ends_with(&['e']) {
                let stem2 = &stem[..stem.len() - 1];
                candidates.push(stem2.to_vec());

                // -ies → -y
                if stem2.ends_with(&['i']) {
                    let stem3 = &stem2[..stem2.len() - 1];
                    let mut lemma_y = stem3.to_vec();
                    lemma_y.push('y');
                    candidates.push(lemma_y);
                }
            }
        }

        // Filter to only those that are valid lemmas in the dictionary
        candidates
            .into_iter()
            .filter(|word| {
                dict.get_word_metadata(word)
                    .is_some_and(|md| md.is_verb_lemma())
            })
            .collect()
    }

    /// Attempts to convert past tense form to lemma.
    /// Tries patterns: -ed, -d, y→ies (becomes y), doubled consonant.
    /// Returns only forms validated as verbs (not nouns) in the dictionary.
    pub fn past_to_lemma<D: Dictionary>(past: &[char], dict: &D) -> Vec<Vec<char>> {
        use crate::CharStringExt;

        let mut candidates = Vec::new();

        // Try stripping -d first, then check for further -e
        if past.ends_with_ignore_ascii_case_chars(&['d']) && past.len() > 1 {
            let without_d = &past[..past.len() - 1];
            candidates.push(without_d.to_vec());

            // If without_d ends with 'e', try removing that too (for -ed pattern)
            if without_d.ends_with_ignore_ascii_case_chars(&['e']) {
                let without_ed = &without_d[..without_d.len() - 1];
                candidates.push(without_ed.to_vec());

                // -ied → -y
                if without_ed.ends_with_ignore_ascii_case_chars(&['i']) {
                    let mut with_y = without_ed[..without_ed.len() - 1].to_vec();
                    with_y.push('y');
                    candidates.push(with_y);
                }

                // doubled consonant: if stem ends in non-vowel, try with one less
                if !without_ed.is_empty() {
                    if let Some(&last_char) = without_ed.last() {
                        if !matches!(last_char.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u') {
                            candidates.push(without_ed[..without_ed.len() - 1].to_vec());
                        }
                    }
                }
            }
        }

        // Filter to only those that are verb lemmas in the dictionary
        candidates
            .into_iter()
            .filter(|word| {
                dict.get_word_metadata(word)
                    .is_some_and(|md| md.is_verb_lemma())
            })
            .collect::<Vec<_>>()
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }

    /// Attempts to find verb lemma(s) from any inflected form.
    /// Tries stripping common verb endings: -ed, -d, -es, -s.
    /// Returns only forms validated as verbs (not nouns) in the dictionary.
    /// Useful for finding the base form when the conjugation type is unknown.
    pub fn any_inflected_to_lemma<D: Dictionary>(
        inflected: &[char],
        dict: &D,
    ) -> Vec<Vec<char>> {
        let mut candidates = Vec::new();

        // Try past tense forms
        let past_lemmas = past_to_lemma(inflected, dict);
        candidates.extend(past_lemmas);

        // Try -es and -s (3rd person singular)
        let ps3_lemmas = ps3_to_lemma(inflected, dict);
        candidates.extend(ps3_lemmas);

        // Deduplicate
        candidates
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spell::FstDictionary;

    mod noun_singular_to_plural {
        use super::*;

        #[test]
        fn adds_s() {
            let dict = FstDictionary::curated();
            let result = nouns::singular_to_plural(&['c', 'a', 't'], &dict);
            assert!(result.iter().any(|w| w.iter().collect::<String>() == "cats"));
        }

        #[test]
        fn adds_es() {
            let dict = FstDictionary::curated();
            let result = nouns::singular_to_plural(&['b', 'o', 'x'], &dict);
            assert!(result.iter().any(|w| w.iter().collect::<String>() == "boxes"));
        }

        #[test]
        fn converts_y_to_ies() {
            let dict = FstDictionary::curated();
            let result = nouns::singular_to_plural(&['c', 'i', 't', 'y'], &dict);
            assert!(result.iter().any(|w| w.iter().collect::<String>() == "cities"));
        }

        #[test]
        fn converts_fe_to_ves() {
            let dict = FstDictionary::curated();
            let result = nouns::singular_to_plural(&['k', 'n', 'i', 'f', 'e'], &dict);
            assert!(result.iter().any(|w| w.iter().collect::<String>() == "knives"));
        }

        #[test]
        fn filters_invalid_forms() {
            let dict = FstDictionary::curated();
            let result = nouns::singular_to_plural(&['x', 'y', 'z'], &dict);
            assert!(result.is_empty());
        }
    }

    mod noun_plural_to_singular {
        use super::*;

        #[test]
        fn strips_s() {
            let dict = FstDictionary::curated();
            let result = nouns::plural_to_singular(&['c', 'a', 't', 's'], &dict);
            assert!(result.iter().any(|w| w.iter().collect::<String>() == "cat"));
        }

        #[test]
        fn strips_es() {
            let dict = FstDictionary::curated();
            let result = nouns::plural_to_singular(&['b', 'o', 'x', 'e', 's'], &dict);
            assert!(result.iter().any(|w| w.iter().collect::<String>() == "box"));
        }

        #[test]
        fn converts_ies_to_y() {
            let dict = FstDictionary::curated();
            let result = nouns::plural_to_singular(&['c', 'i', 't', 'i', 'e', 's'], &dict);
            assert!(result.iter().any(|w| w.iter().collect::<String>() == "city"));
        }

        #[test]
        fn converts_ves_to_fe() {
            let dict = FstDictionary::curated();
            let result = nouns::plural_to_singular(&['k', 'n', 'i', 'v', 'e', 's'], &dict);
            assert!(result.iter().any(|w| w.iter().collect::<String>() == "knife"));
        }

        #[test]
        fn filters_invalid_forms() {
            let dict = FstDictionary::curated();
            let result = nouns::plural_to_singular(&['x', 'y', 'z', 's'], &dict);
            assert!(result.is_empty());
        }
    }

    mod verb_lemma_to_3ps {
        use super::*;

        #[test]
        fn adds_s() {
            let dict = FstDictionary::curated();
            let result = verbs::lemma_to_3ps("run", &dict);
            assert!(result.iter().any(|w| w.iter().collect::<String>() == "runs"));
        }

        #[test]
        fn adds_es() {
            let dict = FstDictionary::curated();
            let result = verbs::lemma_to_3ps("go", &dict);
            assert!(result.iter().any(|w| w.iter().collect::<String>() == "goes"));
        }

        #[test]
        fn converts_y_to_ies() {
            let dict = FstDictionary::curated();
            let result = verbs::lemma_to_3ps("cry", &dict);
            assert!(result.iter().any(|w| w.iter().collect::<String>() == "cries"));
        }

        #[test]
        fn filters_invalid_forms() {
            let dict = FstDictionary::curated();
            let result = verbs::lemma_to_3ps("xyz", &dict);
            assert!(result.is_empty());
        }
    }

    mod verb_ps3_to_lemma {
        use super::*;

        #[test]
        fn strips_s() {
            let dict = FstDictionary::curated();
            let result = verbs::ps3_to_lemma(&['r', 'u', 'n', 's'], &dict);
            assert!(result.iter().any(|w| w.iter().collect::<String>() == "run"));
        }

        #[test]
        fn strips_es() {
            let dict = FstDictionary::curated();
            let result = verbs::ps3_to_lemma(&['g', 'o', 'e', 's'], &dict);
            assert!(result.iter().any(|w| w.iter().collect::<String>() == "go"));
        }

        #[test]
        fn converts_ies_to_y() {
            let dict = FstDictionary::curated();
            let result = verbs::ps3_to_lemma(&['c', 'r', 'i', 'e', 's'], &dict);
            assert!(result.iter().any(|w| w.iter().collect::<String>() == "cry"));
        }

        #[test]
        fn filters_invalid_forms() {
            let dict = FstDictionary::curated();
            let result = verbs::ps3_to_lemma(&['x', 'y', 'z', 's'], &dict);
            assert!(result.is_empty());
        }
    }
}
