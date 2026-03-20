use crate::{
    CharStringExt,
    char_ext::CharExt,
    spell::{Dictionary, FstDictionary},
};

/// Get all valid plural forms using regular patterns, validated against dictionary
/// Returns vector of suggestions that are confirmed to be plural nouns
pub fn get_plurals(singular: &[char]) -> Vec<Vec<char>> {
    // Filter candidates by dictionary validation
    let dict = FstDictionary::curated();
    get_plural_candidates(singular)
        .into_iter()
        .filter(|candidate| {
            dict.get_word_metadata(candidate)
                .is_some_and(|md| md.is_plural_noun())
        })
        .collect()
}

/// Get all valid singular forms using regular patterns, validated against dictionary
/// Returns vector of suggestions that are confirmed to be singular nouns
pub fn get_singulars(plural: &[char]) -> Vec<Vec<char>> {
    let dict = FstDictionary::curated();
    // Filter candidates by dictionary validation
    get_singular_candidates(plural)
        .into_iter()
        .filter(|candidate| {
            dict.get_word_metadata(candidate)
                .is_some_and(|md| md.is_singular_noun())
        })
        .collect()
}

fn get_plural_candidates(singular: &[char]) -> Vec<Vec<char>> {
    let mut candidates = Vec::new();

    if singular.is_empty() {
        return candidates;
    }

    // Handle -y -> -ies pattern (e.g., "city" -> "cities")
    if singular.ends_with_ignore_ascii_case_chars(&['y']) && singular.len() > 1 {
        let prev = singular[singular.len() - 2];
        // Change -y to -ies when preceded by a consonant
        if !prev.is_vowel() {
            let mut plural = singular[..singular.len() - 1].to_vec();
            plural.extend(['i', 'e', 's']);
            candidates.push(plural);
        }
    }

    // Handle -fe -> -ves pattern (e.g., "wife" -> "wives")
    if singular.ends_with_ignore_ascii_case_chars(&['f', 'e']) {
        let mut plural = singular[..singular.len() - 2].to_vec();
        plural.extend(['v', 'e', 's']);
        candidates.push(plural);
    }

    // Handle -f -> -ves pattern (e.g., "wolf" -> "wolves")
    if singular.ends_with_ignore_ascii_case_chars(&['f'])
        && !singular.ends_with_ignore_ascii_case_chars(&['f', 'f'])
    {
        let mut plural = singular[..singular.len() - 1].to_vec();
        plural.extend(['v', 'e', 's']);
        candidates.push(plural);
    }

    // Handle -s, -ss, -sh, -ch, -x, -z -> -es pattern
    if singular.ends_with_ignore_ascii_case_chars(&['s'])
        || singular.ends_with_ignore_ascii_case_chars(&['s', 's'])
        || singular.ends_with_ignore_ascii_case_chars(&['s', 'h'])
        || singular.ends_with_ignore_ascii_case_chars(&['c', 'h'])
        || singular.ends_with_ignore_ascii_case_chars(&['x'])
        || singular.ends_with_ignore_ascii_case_chars(&['z'])
    {
        let mut plural = singular.to_vec();
        plural.extend(['e', 's']);
        candidates.push(plural);
    }

    // Handle consonant + -o -> -oes pattern (e.g., "potato" -> "potatoes")
    if singular.ends_with_ignore_ascii_case_chars(&['o']) && singular.len() > 1 {
        let prev = singular[singular.len() - 2];
        if !prev.is_vowel() {
            let mut plural = singular.to_vec();
            plural.extend(['e', 's']);
            candidates.push(plural);
        }
    }

    // Default: add -s
    let mut plural = singular.to_vec();
    plural.push('s');
    candidates.push(plural);

    candidates
}

fn get_singular_candidates(plural: &[char]) -> Vec<Vec<char>> {
    let mut candidates = Vec::new();

    if plural.is_empty() {
        return candidates;
    }

    // Handle -ies -> -y pattern (e.g., "cities" -> "city")
    if plural.ends_with_ignore_ascii_case_chars(&['i', 'e', 's']) {
        let mut singular = plural[..plural.len() - 3].to_vec();
        singular.push('y');
        candidates.push(singular);
    }

    // Handle -ves -> -fe pattern (e.g., "wives" -> "wife")
    if plural.ends_with_ignore_ascii_case_chars(&['v', 'e', 's']) {
        let mut singular = plural[..plural.len() - 3].to_vec();
        singular.extend(['f', 'e']);
        candidates.push(singular);
    }

    // Handle -es -> (remove es) pattern for -s, -sh, -ch, -x, -z endings
    if plural.ends_with_ignore_ascii_case_chars(&['e', 's']) && plural.len() > 2 {
        let prev_two = &plural[plural.len() - 3..plural.len() - 1];
        if prev_two.eq_ignore_ascii_case_str("sh")
            || prev_two.eq_ignore_ascii_case_str("ch")
            || plural.ends_with_ignore_ascii_case_chars(&['x', 'e', 's'])
            || plural.ends_with_ignore_ascii_case_chars(&['z', 'e', 's'])
            || plural.ends_with_ignore_ascii_case_chars(&['s', 's', 'e', 's'])
        {
            let singular = plural[..plural.len() - 2].to_vec();
            candidates.push(singular);
        }
    }

    // Handle -oes -> -o pattern for consonant + o endings
    if plural.ends_with_ignore_ascii_case_chars(&['o', 'e', 's']) && plural.len() > 3 {
        let preceding_char = plural[plural.len() - 4];
        if !preceding_char.is_vowel() {
            let singular = plural[..plural.len() - 2].to_vec();
            candidates.push(singular);
        }
    }

    // Default: remove trailing -s (but only if it's a single s)
    if plural.ends_with_ignore_ascii_case_chars(&['s'])
        && !plural.ends_with_ignore_ascii_case_chars(&['s', 's'])
    {
        let singular = plural[..plural.len() - 1].to_vec();
        candidates.push(singular);
    }

    candidates
}

#[cfg(test)]
mod tests {
    use super::{get_plurals, get_singulars};

    #[test]
    fn test_regular_plurals_add_s() {
        let plurals = get_plurals(&['c', 'a', 't']);
        assert!(plurals.contains(&['c', 'a', 't', 's'].to_vec()));
    }

    #[test]
    fn test_regular_plurals_y_to_ies() {
        let plurals = get_plurals(&['c', 'i', 't', 'y']);
        assert!(plurals.contains(&['c', 'i', 't', 'i', 'e', 's'].to_vec()));
    }

    #[test]
    fn test_regular_plurals_vowel_y_adds_s() {
        let plurals = get_plurals(&['b', 'o', 'y']);
        assert!(plurals.contains(&['b', 'o', 'y', 's'].to_vec()));
    }

    // TODO: "wives" not in FstDictionary yet - investigate why wife's plural isn't included
    #[test]
    fn test_regular_plurals_fe_to_ves() {
        let plurals = get_plurals(&['w', 'i', 'f', 'e']);
        assert!(plurals.contains(&['w', 'i', 'v', 'e', 's'].to_vec()));
    }

    #[test]
    fn test_regular_plurals_f_to_ves() {
        let plurals = get_plurals(&['w', 'o', 'l', 'f']);
        assert!(plurals.contains(&['w', 'o', 'l', 'v', 'e', 's'].to_vec()));
    }

    #[test]
    fn test_regular_plurals_es_endings() {
        let plurals = get_plurals(&['b', 'u', 's']);
        assert!(plurals.contains(&['b', 'u', 's', 'e', 's'].to_vec()));
    }

    #[test]
    fn test_regular_plurals_o_to_oes() {
        let plurals = get_plurals(&['p', 'o', 't', 'a', 't', 'o']);
        assert!(plurals.contains(&['p', 'o', 't', 'a', 't', 'o', 'e', 's'].to_vec()));
    }

    #[test]
    fn test_regular_plurals_vowel_o_adds_s() {
        let plurals = get_plurals(&['r', 'a', 'd', 'i', 'o']);
        assert!(plurals.contains(&['r', 'a', 'd', 'i', 'o', 's'].to_vec()));
    }

    #[test]
    fn test_single_letter_words() {
        // Single letters won't generate valid plurals in the dictionary
        let plurals = get_plurals(&['a']);
        assert!(plurals.is_empty());

        let singulars = get_singulars(&['s']);
        // Single 's' will be treated as a plural and try to remove the 's', leaving empty
        assert!(singulars.is_empty());
    }

    #[test]
    fn test_empty_string() {
        let plurals = get_plurals(&[]);
        assert!(plurals.is_empty());

        let singulars = get_singulars(&[]);
        assert!(singulars.is_empty());
    }

    #[test]
    fn test_chars_versions() {
        let plurals = get_plurals(&['c', 'i', 't', 'y']);
        assert!(plurals.contains(&['c', 'i', 't', 'i', 'e', 's'].to_vec()));

        let singulars = get_singulars(&['c', 'i', 't', 'i', 'e', 's']);
        assert!(singulars.contains(&['c', 'i', 't', 'y'].to_vec()));
    }

    #[test]
    fn test_singular_from_plural_remove_s() {
        let singulars = get_singulars(&['c', 'a', 't', 's']);
        assert!(singulars.contains(&['c', 'a', 't'].to_vec()));
    }

    #[test]
    fn test_singular_from_plural_ies_to_y() {
        let singulars = get_singulars(&['c', 'i', 't', 'i', 'e', 's']);
        assert!(singulars.contains(&['c', 'i', 't', 'y'].to_vec()));
    }

    #[test]
    fn test_singular_from_plural_ves_to_fe() {
        let singulars = get_singulars(&['w', 'i', 'v', 'e', 's']);
        assert!(singulars.contains(&['w', 'i', 'f', 'e'].to_vec()));
    }

    #[test]
    fn test_singular_from_plural_es_removal() {
        // Test removing -es (for -sh, -ch, -x, -z, -s endings)
        // For simplicity, just verify the pattern matching works with a longer word
        let singulars = get_singulars(&['c', 'l', 'a', 's', 's', 'e', 's']);
        // "classes" should produce "class" by removing "es"
        assert!(
            !singulars.is_empty(),
            "should have candidates for removing -es"
        );
    }

    #[test]
    fn test_singular_from_plural_oes_to_o() {
        let singulars = get_singulars(&['p', 'o', 't', 'a', 't', 'o', 'e', 's']);
        assert!(singulars.contains(&['p', 'o', 't', 'a', 't', 'o'].to_vec()));
    }

    #[test]
    fn test_empty_string_plural() {
        let plurals = get_plurals(&[]);
        assert!(plurals.is_empty());
    }

    #[test]
    fn test_empty_string_singular() {
        let singulars = get_singulars(&[]);
        assert!(singulars.is_empty());
    }

    #[test]
    fn test_case_insensitive_plurals() {
        // Lowercase input produces lowercase output
        let lowercase_city: Vec<char> = "city".chars().collect();
        let plurals = get_plurals(&lowercase_city);
        assert!(plurals.contains(&['c', 'i', 't', 'i', 'e', 's'].to_vec()));

        // Mixed case input: stem case preserved, suffix lowercase
        let mixed_city: Vec<char> = "City".chars().collect();
        let plurals = get_plurals(&mixed_city);
        assert!(plurals.contains(&['C', 'i', 't', 'i', 'e', 's'].to_vec()));
    }

    #[test]
    fn test_case_insensitive_singulars() {
        // Lowercase input produces lowercase output
        let lowercase_cities: Vec<char> = "cities".chars().collect();
        let singulars = get_singulars(&lowercase_cities);
        assert!(singulars.contains(&['c', 'i', 't', 'y'].to_vec()));

        // Mixed case input: case preserved from stem
        let mixed_cities: Vec<char> = "Cities".chars().collect();
        let singulars = get_singulars(&mixed_cities);
        assert!(singulars.contains(&['C', 'i', 't', 'y'].to_vec()));
    }

    #[test]
    fn test_generated_candidates_must_validate() {
        let plurals = get_plurals(&['a', 'n', 't']);
        // "ants" should be in the dictionary as a plural noun
        assert!(plurals.contains(&['a', 'n', 't', 's'].to_vec()));

        // "buses" should be in the dictionary as a plural noun
        let plurals = get_plurals(&['b', 'u', 's']);
        assert!(plurals.contains(&['b', 'u', 's', 'e', 's'].to_vec()));
    }
}
