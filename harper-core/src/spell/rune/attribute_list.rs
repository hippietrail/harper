use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use smallvec::ToSmallVec;

use super::super::word_map::{WordMap, WordMapEntry};
use super::Error;
use super::affix_replacement::AffixReplacement;
use super::expansion::{Expansion, HumanReadableExpansion};
use super::word_list::MarkedWord;
use crate::{CharString, Span, WordId, WordMetadata};

#[derive(Debug, Clone)]
pub struct AttributeList {
    /// Key = Affix Flag
    affixes: HashMap<char, Expansion>,
}

impl AttributeList {
    fn into_human_readable(self) -> HumanReadableAttributeList {
        HumanReadableAttributeList {
            affixes: self
                .affixes
                .into_iter()
                .map(|(affix, exp)| (affix, exp.into_human_readable()))
                .collect(),
        }
    }

    pub fn parse(source: &str) -> Result<Self, Error> {
        let human_readable: HumanReadableAttributeList =
            serde_json::from_str(source).map_err(|_| Error::MalformedJSON)?;

        human_readable.into_normal()
    }

    /// Expand [`MarkedWord`] into a list of full words, including itself.
    ///
    /// Will append to the given `dest`;
    ///
    /// In the future, I want to make this function cleaner and faster.
    pub fn expand_marked_word(&self, word: MarkedWord, dest: &mut WordMap) {
        dest.reserve(word.attributes.len() + 1);
        let mut gifted_metadata = WordMetadata::default();

        for attr in &word.attributes {
            let Some(expansion) = self.affixes.get(attr) else {
                continue;
            };

            gifted_metadata.append(&expansion.gifts_metadata);
            let mut new_words: HashMap<CharString, WordMetadata> = HashMap::new();

            for replacement in &expansion.replacements {
                if let Some(replaced) =
                    Self::apply_replacement(replacement, &word.letters, expansion.suffix)
                {
                    if let Some(val) = new_words.get_mut(&replaced) {
                        val.append(&expansion.adds_metadata);
                    } else {
                        new_words.insert(replaced, expansion.adds_metadata.clone());
                    }
                }
            }

            if expansion.cross_product {
                let mut opp_attr = Vec::new();

                for attr in &word.attributes {
                    let Some(attr_def) = self.affixes.get(attr) else {
                        continue;
                    };
                    if attr_def.suffix != expansion.suffix {
                        opp_attr.push(*attr);
                    }
                }

                for (new_word, metadata) in new_words {
                    self.expand_marked_word(
                        MarkedWord {
                            letters: new_word.clone(),
                            attributes: opp_attr.clone(),
                        },
                        dest,
                    );
                    let t_metadata = dest.get_metadata_mut_chars(&new_word).unwrap();
                    t_metadata.append(&metadata);
                    t_metadata.derived_from = Some(WordId::from_word_chars(&word.letters))
                }
            } else {
                for (key, mut value) in new_words.into_iter() {
                    value.derived_from = Some(WordId::from_word_chars(&word.letters));

                    if let Some(val) = dest.get_metadata_mut_chars(&key) {
                        val.append(&value);
                    } else {
                        dest.insert(WordMapEntry {
                            canonical_spelling: key,
                            metadata: value,
                        });
                    }
                }
            }
        }

        if let Some(prev_val) = dest.get_with_chars(&word.letters) {
            dest.insert(WordMapEntry {
                metadata: gifted_metadata.or(&prev_val.metadata),
                canonical_spelling: word.letters,
            });
        } else {
            dest.insert(WordMapEntry {
                metadata: gifted_metadata,
                canonical_spelling: word.letters,
            });
        }
    }

    /// Expand an iterator of marked words into strings.
    /// Note that this does __not__ guarantee that produced words will be
    /// unique.
    pub fn expand_marked_words(
        &self,
        words: impl IntoIterator<Item = MarkedWord>,
        dest: &mut WordMap,
    ) {
        for word in words {
            self.expand_marked_word(word, dest);
        }
    }

    fn apply_replacement(
        replacement: &AffixReplacement,
        letters: &[char],
        suffix: bool,
    ) -> Option<CharString> {
        if replacement.condition.len() > letters.len() {
            return None;
        }

        let target_span = if suffix {
            Span::new(letters.len() - replacement.condition.len(), letters.len())
        } else {
            Span::new(0, replacement.condition.len())
        };

        let target_segment = target_span.get_content(letters);

        if replacement.condition.matches(target_segment) {
            let mut replaced_segment = letters.to_smallvec();
            let mut remove: CharString = replacement.remove.to_smallvec();

            if !suffix {
                replaced_segment.reverse();
            } else {
                remove.reverse();
            }

            for c in &remove {
                let last = replaced_segment.last()?;

                if last == c {
                    replaced_segment.pop();
                } else {
                    return None;
                }
            }

            let mut to_add = replacement.add.to_vec();

            if !suffix {
                to_add.reverse()
            }

            replaced_segment.extend(to_add);

            if !suffix {
                replaced_segment.reverse();
            }

            return Some(replaced_segment);
        }

        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanReadableAttributeList {
    affixes: HashMap<char, HumanReadableExpansion>,
}

impl HumanReadableAttributeList {
    pub fn into_normal(self) -> Result<AttributeList, Error> {
        let mut affixes = HashMap::with_capacity(self.affixes.len());

        for (affix, expansion) in self.affixes.into_iter() {
            affixes.insert(affix, expansion.into_normal()?);
        }

        Ok(AttributeList { affixes })
    }
}
