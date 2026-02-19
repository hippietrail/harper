//! Helper functions for converting between different inflected forms of words.
//! Handles regular patterns for nouns (singular/plural) and verbs (lemma/3rd person singular).
//!
//! Mirrors the structure of `irregular_nouns.rs` and `irregular_verbs.rs` by centralizing
//! ad-hoc inflection logic scattered across multiple linters.
pub mod nouns {
    use crate::spell::Dictionary;

    /// Attempts to convert a singular noun to its plural form(s).
    /// Tries common patterns: +s, +es, y→ies, fe→ves.
    /// Returns only forms validated by the dictionary.
    pub fn singular_to_plural<D: Dictionary>(singular: &[char], dict: &D) -> Vec<Vec<char>> {
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
    pub fn plural_to_singular<D: Dictionary>(plural: &[char], dict: &D) -> Vec<Vec<char>> {
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
        } else if word.ends_with(&['s']) {
            VerbConjugation::ThirdPersonSingular
        } else {
            VerbConjugation::Lemma
        }
    }
}

pub mod verbs {
    use crate::char_ext::CharExt;
    use crate::spell::Dictionary;

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
    /// Returns only forms validated in the dictionary, prioritized by likelihood.
    pub fn past_to_lemma<D: Dictionary>(past: &[char], dict: &D) -> Vec<Vec<char>> {
        use crate::CharStringExt;

        let mut candidates = Vec::new();

        // Try -ed first (remove both 'e' and 'd')
        if past.ends_with_ignore_ascii_case_chars(&['e', 'd']) && past.len() > 2 {
            // Prioritize -ied → -y first (more specific)
            let without_ed = &past[..past.len() - 2];
            if without_ed.ends_with_ignore_ascii_case_chars(&['i']) {
                let mut with_y = without_ed[..without_ed.len() - 1].to_vec();
                with_y.push('y');
                candidates.push(with_y);
            }

            // Try removing just the 'd' (e.g., "used" → "use")
            let without_d = &past[..past.len() - 1];
            candidates.push(without_d.to_vec());

            // Try removing -ed for simple cases
            let without_ed = &past[..past.len() - 2];
            candidates.push(without_ed.to_vec());

            // doubled consonant: if stem (without -ed) ends in non-vowel, try with one less
            let without_ed = &past[..past.len() - 2];
            if !without_ed.is_empty()
                && let Some(&last_char) = without_ed.last()
                && !last_char.is_vowel()
            {
                candidates.push(without_ed[..without_ed.len() - 1].to_vec());
            }
        }

        // Try just -d (for words ending in -d but not -ed, e.g., "pad" → "pa")
        if past.ends_with_ignore_ascii_case_chars(&['d'])
            && past.len() > 1
            && !past.ends_with_ignore_ascii_case_chars(&['e', 'd'])
        {
            let without_d = &past[..past.len() - 1];
            candidates.push(without_d.to_vec());
        }

        // Filter to only those in the dictionary and deduplicate while preserving order
        let mut seen = hashbrown::HashSet::new();
        let mut valid_lemmas = Vec::new();
        let mut valid_other = Vec::new();

        for candidate in candidates {
            if !seen.contains(&candidate) {
                seen.insert(candidate.clone());
                if let Some(md) = dict.get_word_metadata(&candidate) {
                    // Prioritize verb lemmas
                    if md.is_verb_lemma() {
                        valid_lemmas.push(candidate);
                    } else {
                        valid_other.push(candidate);
                    }
                }
            }
        }

        // Return lemmas first, then other valid forms
        valid_lemmas.into_iter().chain(valid_other).collect()
    }

    /// Attempts to find verb lemma(s) from any inflected form.
    /// Tries stripping common verb endings: -ed, -d, -es, -s.
    /// Returns only forms validated as verbs (not nouns) in the dictionary.
    /// Useful for finding the base form when the conjugation type is unknown.
    pub fn any_inflected_to_lemma<D: Dictionary>(inflected: &[char], dict: &D) -> Vec<Vec<char>> {
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
            .collect::<hashbrown::HashSet<_>>()
            .into_iter()
            .collect()
    }

    /// Attempts to convert a verb lemma to its progressive form (gerund, verb + -ing).
    /// Handles common patterns: just +ing, or remove final -e then +ing.
    /// Returns only forms validated by the dictionary.
    pub fn lemma_to_progressive<D: Dictionary>(lemma: &str, dict: &D) -> Vec<Vec<char>> {
        // Exceptions: verbs where -e is kept before -ing or special spelling
        let exceptions = [
            "see",
            "flee",
            "agree",
            "knee",
            "guarantee", // -ee verbs
            "hoe",
            "toe", // -oe verbs
            "dye",
            "eye", // -ye verbs
            "singe",
            "tinge", // irregular spelling
        ];

        let gerund = if lemma.ends_with('e') && !exceptions.contains(&lemma) {
            format!("{}ing", &lemma[0..lemma.len() - 1])
        } else {
            format!("{lemma}ing")
        };

        let gerund_chars: Vec<char> = gerund.chars().collect();

        // Only return if it's in the dictionary
        if dict.get_word_metadata(&gerund_chars).is_some() {
            vec![gerund_chars]
        } else {
            Vec::new()
        }
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
            assert!(
                result
                    .iter()
                    .any(|w| w.iter().collect::<String>() == "cats")
            );
        }

        #[test]
        fn adds_es() {
            let dict = FstDictionary::curated();
            let result = nouns::singular_to_plural(&['b', 'o', 'x'], &dict);
            assert!(
                result
                    .iter()
                    .any(|w| w.iter().collect::<String>() == "boxes")
            );
        }

        #[test]
        fn converts_y_to_ies() {
            let dict = FstDictionary::curated();
            let result = nouns::singular_to_plural(&['c', 'i', 't', 'y'], &dict);
            assert!(
                result
                    .iter()
                    .any(|w| w.iter().collect::<String>() == "cities")
            );
        }

        #[test]
        fn converts_fe_to_ves() {
            let dict = FstDictionary::curated();
            let result = nouns::singular_to_plural(&['k', 'n', 'i', 'f', 'e'], &dict);
            assert!(
                result
                    .iter()
                    .any(|w| w.iter().collect::<String>() == "knives")
            );
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
            assert!(
                result
                    .iter()
                    .any(|w| w.iter().collect::<String>() == "city")
            );
        }

        #[test]
        fn converts_ves_to_fe() {
            let dict = FstDictionary::curated();
            let result = nouns::plural_to_singular(&['k', 'n', 'i', 'v', 'e', 's'], &dict);
            assert!(
                result
                    .iter()
                    .any(|w| w.iter().collect::<String>() == "knife")
            );
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
            assert!(
                result
                    .iter()
                    .any(|w| w.iter().collect::<String>() == "runs")
            );
        }

        #[test]
        fn adds_es() {
            let dict = FstDictionary::curated();
            let result = verbs::lemma_to_3ps("go", &dict);
            assert!(
                result
                    .iter()
                    .any(|w| w.iter().collect::<String>() == "goes")
            );
        }

        #[test]
        fn converts_y_to_ies() {
            let dict = FstDictionary::curated();
            let result = verbs::lemma_to_3ps("cry", &dict);
            assert!(
                result
                    .iter()
                    .any(|w| w.iter().collect::<String>() == "cries")
            );
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

    mod verb_past_to_lemma {
        use super::*;

        #[test]
        fn strips_ed() {
            let dict = FstDictionary::curated();
            let result = verbs::past_to_lemma(&['f', 'o', 'r', 'k', 'e', 'd'], &dict);
            assert!(
                result
                    .iter()
                    .any(|w| w.iter().collect::<String>() == "fork")
            );
        }

        #[test]
        fn strips_d() {
            let dict = FstDictionary::curated();
            let result = verbs::past_to_lemma(&['u', 's', 'e', 'd'], &dict);
            // "used" ends with -ed, so should give "use", not "used"
            assert!(result.iter().any(|w| w.iter().collect::<String>() == "use"));
        }

        #[test]
        fn converts_ied_to_y() {
            let dict = FstDictionary::curated();
            let result = verbs::past_to_lemma(&['f', 'r', 'i', 'e', 'd'], &dict);
            assert!(result.iter().any(|w| w.iter().collect::<String>() == "fry"));
        }

        #[test]
        fn handles_doubled_consonant() {
            let dict = FstDictionary::curated();
            let result = verbs::past_to_lemma(&['l', 'o', 'g', 'g', 'e', 'd'], &dict);
            assert!(result.iter().any(|w| w.iter().collect::<String>() == "log"));
        }

        #[test]
        fn filters_invalid_forms() {
            let dict = FstDictionary::curated();
            let result = verbs::past_to_lemma(&['x', 'y', 'z', 'e', 'd'], &dict);
            assert!(result.is_empty());
        }
    }
}
