mod affix_replacement;
mod attribute_list;
mod error;
mod expansion;
mod matcher;
pub mod word_list;

pub use attribute_list::AttributeList;
use attribute_list::HumanReadableAttributeList;
pub use error::Error;

pub use self::word_list::MarkedWord;
use self::word_list::parse_word_list;

pub fn parse_default_word_list() -> Result<Vec<MarkedWord>, Error> {
    parse_word_list(include_str!("../../../dictionary.dict"))
}

pub fn parse_default_attribute_list() -> AttributeList {
    let human_readable: HumanReadableAttributeList =
        serde_json::from_str(include_str!("../../../affixes.json"))
            .expect("The built-in affix list should always be valid.");

    human_readable
        .into_normal()
        .expect("All expressions in the built-in attribute list should be valid.")
}

#[cfg(test)]
mod tests {
    use hashbrown::HashSet;
    use serde_json::json;

    use super::super::word_map::WordMap;
    use super::word_list::parse_word_list;
    use super::{parse_default_attribute_list, parse_default_word_list};
    use crate::CharStringExt;
    use crate::spell::hunspell::attribute_list::HumanReadableAttributeList;

    pub const TEST_WORD_LIST: &str = "3\nhello\ntry/B\nwork/AB";

    #[test]
    fn correctly_expands_test_files() {
        let words = parse_word_list(TEST_WORD_LIST).unwrap();
        let attributes: HumanReadableAttributeList = serde_json::from_value(json!({
            "affixes": {
                "A": {
                    "suffix": false,
                    "cross_product": true,
                    "replacements": [
                      {
                        "remove": "",
                        "add": "re",
                        "condition": "."
                      }
                    ],
                    "adds_metadata": {
                      "kind": null,
                      "tense": null
                    },
                    "gifts_metadata": {}
                },
                "B": {
                    "suffix": true,
                    "cross_product": true,
                    "replacements": [
                      {
                        "remove": "",
                        "add": "ed",
                        "condition": "[^y]"
                      },
                      {
                        "remove": "y",
                        "add": "ied",
                        "condition": "y"
                      }
                    ],
                    "adds_metadata": {
                      "kind": null,
                      "tense": null
                    },
                    "gifts_metadata": {}
                }
            }
        }))
        .unwrap();
        let attributes = attributes.into_normal().unwrap();

        let mut expanded = WordMap::default();

        attributes.expand_marked_words(words, &mut expanded);
        let expanded: HashSet<String> = expanded
            .into_iter()
            .map(|v| v.canonical_spelling.to_string())
            .collect();

        assert_eq!(
            expanded,
            vec![
                "hello", "tried", "reworked", "rework", "worked", "work", "try"
            ]
            .into_iter()
            .map(|v| v.into())
            .collect()
        )
    }

    #[test]
    fn plural_giants() {
        let words = parse_word_list("1\ngiant/SM").unwrap();

        let attributes: HumanReadableAttributeList = serde_json::from_value(json!({
            "affixes": {
                "S": {
                    "suffix": true,
                    "cross_product": true,
                    "replacements": [
                      {
                        "remove": "y",
                        "add": "ies",
                        "condition": "[^aeiou]"
                      },
                      {
                        "remove": "",
                        "add": "s",
                        "condition": "[aeiou]y"
                      },
                      {
                        "remove": "",
                        "add": "s",
                        "condition": "[^sxzhy]"
                      }
                    ],
                    "adds_metadata": {
                        "noun": {
                            "is_plural": true
                        }
                    },
                    "gifts_metadata": {
                        "noun": {}
                    }
                },
                "M": {
                    "suffix": true,
                    "cross_product": true,
                    "replacements": [
                      {
                        "remove": "",
                        "add": "'s",
                        "condition": "."
                      }
                    ],
                    "adds_metadata": {},
                    "gifts_metadata": {}
                }
            }
        }))
        .unwrap();
        let attributes = attributes.into_normal().unwrap();

        let mut expanded = WordMap::default();

        attributes.expand_marked_words(words, &mut expanded);

        let giant_data = expanded.get_with_str("giant").unwrap();
        assert!(giant_data.metadata.is_noun());

        let giants_data = expanded.get_with_str("giants").unwrap();
        assert!(giants_data.metadata.is_plural_noun());
    }

    fn build_expanded() -> WordMap {
        let words = parse_default_word_list().unwrap();
        let attributes = parse_default_attribute_list();

        let mut expanded = WordMap::default();

        attributes.expand_marked_words(words, &mut expanded);

        expanded
    }

    #[test]
    fn can_expand_default() {
        build_expanded();
    }

    #[test]
    fn expanded_contains_giants() {
        assert!(build_expanded().contains_str("giants"));
    }

    #[test]
    fn expanded_contains_deallocate() {
        assert!(build_expanded().contains_str("deallocate"));
    }

    #[test]
    fn expanded_contains_repo() {
        let expanded = build_expanded();

        assert!(expanded.contains_str("repo"));
        assert!(expanded.contains_str("repos"));
        assert!(expanded.contains_str("repo's"));
    }

    #[test]
    fn expanded_contains_possessive_abandonment() {
        assert!(
            build_expanded()
                .get_with_str("abandonment's")
                .unwrap()
                .metadata
                .is_possessive_noun()
        )
    }

    #[test]
    fn has_is_not_a_nominal() {
        let expanded = build_expanded();

        let has = expanded.get_with_str("has");
        assert!(has.is_some());

        assert!(!has.unwrap().metadata.is_nominal())
    }

    #[test]
    fn is_is_linking_verb() {
        let expanded = build_expanded();

        let is = expanded.get_with_str("is");

        dbg!(&is);
        assert!(is.is_some());
        assert!(is.unwrap().metadata.is_linking_verb());
    }

    #[test]
    fn are_merged_attrs_same_as_spread_attrs() {
        let merged_word = parse_word_list("1\nblork/DGS").unwrap();
        let spread_word = parse_word_list("2\nblork/DG\nblork/S").unwrap();

        let merged_attrs = parse_default_attribute_list();
        let spread_attrs = parse_default_attribute_list();

        let mut expanded1 = WordMap::default();
        let mut expanded2 = WordMap::default();

        merged_attrs.expand_marked_words(merged_word, &mut expanded1);
        let expanded_merged: HashSet<String> = expanded1
            .into_iter()
            .map(|v| v.canonical_spelling.into_iter().collect())
            .collect();

        spread_attrs.expand_marked_words(spread_word, &mut expanded2);
        let expanded_spread: HashSet<String> = expanded2
            .into_iter()
            .map(|v| v.canonical_spelling.into_iter().collect())
            .collect();

        assert_eq!(
            expanded_merged.into_iter().collect::<HashSet<_>>(),
            expanded_spread.into_iter().collect::<HashSet<_>>()
        );
    }
}
