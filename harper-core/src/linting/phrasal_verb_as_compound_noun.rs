use std::sync::Arc;

use super::{Lint, LintKind, Linter, Suggestion};
use crate::{Dictionary, Document, FstDictionary, Span, Token, TokenStringExt};

/// Detect phrasal verbs written as compound nouns.
pub struct PhrasalVerbAsCompoundNoun {
    dict: Arc<FstDictionary>,
}

// Helper functions to get tokens before or after the current one
// After is easy since it already handles going past the end
fn get_token_after(doc: &Document, i: usize, n: usize) -> Option<&Token> {
    doc.get_token(i + n)
}

// Before is trickier since we have to handle going past the start
fn get_token_before(doc: &Document, i: usize, n: usize) -> Option<&Token> {
    (i >= n).then(|| doc.get_token(i - n).unwrap())
}

impl PhrasalVerbAsCompoundNoun {
    pub fn new() -> Self {
        Self {
            dict: FstDictionary::curated(),
        }
    }
}

impl Default for PhrasalVerbAsCompoundNoun {
    fn default() -> Self {
        Self {
            dict: FstDictionary::curated(),
        }
    }
}

impl Linter for PhrasalVerbAsCompoundNoun {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut lints = Vec::new();
        for i in document.iter_noun_indices() {
            // It would be handy if there could be a dict flag for nouns which are compounds of phrasal verbs.
            // Instead let's use a few heuristics.
            let token = document.get_token(i).unwrap();
            // * Can't also be a proper noun or a real verb.
            if token.kind.is_proper_noun() || token.kind.is_verb() {
                continue;
            }
            let nountok_charsl = document.get_span_content(&token.span);
            // * Can't contain a hyphen or space.
            if nountok_charsl.contains(&'-') || nountok_charsl.contains(&' ') {
                // This means Harper's tokenizer's concept of a word is different to what we expect
                // or has changed since this code was written.
                unreachable!();
            }
            // * Can't contain an apostrophe
            if nountok_charsl.contains(&'\'') || nountok_charsl.contains(&'’') {
                continue;
            }
            // * Must end with the same letters as one of the particles used in phrasal verbs.
            let particle_endings: &[&[char]] = &[
                &['a', 'r', 'o', 'u', 'n', 'd'][..],
                &['b', 'a', 'c', 'k'][..],
                &['d', 'o', 'w', 'n'][..],
                &['i', 'n'][..],
                &['o', 'n'][..],
                &['o', 'f', 'f'][..],
                &['o', 'u', 't'][..],
                &['o', 'v', 'e', 'r'][..],
                &['u', 'p'][..],
            ];

            // * Must not be in the set of known false positives.
            if nountok_charsl == &['g', 'a', 'l', 'l', 'o', 'n'] {
                // eprintln!(
                //     "* Rejected '{}' because it's a known false positive",
                //     nountok_charsl.iter().collect::<String>()
                // );
                continue;
            }

            let mut found_particle_len = 0;
            if !particle_endings.iter().any(|ending| {
                let ending_len = ending.len();
                if ending_len <= nountok_charsl.len()
                    && ending
                        .iter()
                        .eq(nountok_charsl[nountok_charsl.len() - ending_len..].iter())
                {
                    found_particle_len = ending_len;
                    true
                } else {
                    false
                }
            }) {
                continue;
            }

            let verb_part = &nountok_charsl[..nountok_charsl.len() - found_particle_len];
            let particle_part = &nountok_charsl[nountok_charsl.len() - found_particle_len..];
            let phrasal_verb: String = verb_part
                .iter()
                .chain(std::iter::once(&' '))
                .chain(particle_part.iter())
                .collect();

            // Check if both things are verbs.
            // So far we only have a small number of phrasal verbs in the dictionary.
            let (verb_part_is_verb, phrasal_verb_is_verb) = (
                self.dict
                    .get_word_metadata(verb_part)
                    .map_or(false, |md| md.verb.is_some()),
                self.dict
                    .get_word_metadata_str(&phrasal_verb)
                    .map_or(false, |md| md.verb.is_some()),
            );

            // If neither is a verb, then it's not a phrasal verb
            if !verb_part_is_verb && !phrasal_verb_is_verb {
                continue;
            }

            // Now we know it matches the pattern of a phrasal verb erroneously written as a compound noun.
            // But we have to check if it's an actual compound noun rather than an error.
            // For that we need some heuristics based on the surrounding context.
            // Let's try to get the word before and the word after.
            // To do that we have to get the tokens immediately before and after, which we expect to be whitespace.
            let maybe_prev_ws = get_token_before(document, i, 1);
            let maybe_prev_tok = maybe_prev_ws.and_then(|prev_ws| {
                (prev_ws.kind.is_whitespace()).then(|| get_token_before(document, i, 2).unwrap())
            });

            let maybe_next_ws = get_token_after(document, i, 1);
            let maybe_next_tok = maybe_next_ws.and_then(|next_ws| {
                (next_ws.kind.is_whitespace()).then(|| get_token_after(document, i, 2).unwrap())
            });

            // If it's in isolation, a compound noun is fine.
            if maybe_prev_tok.is_none() && maybe_next_tok.is_none() {
                continue;
            }

            let message = match (phrasal_verb_is_verb, verb_part_is_verb) {
                (true, _) => Some("This word should be a phrasal verb, not a compound noun."),
                (false, true) => {
                    Some("This word might be a phrasal verb rather than a compound noun.")
                }
                _ => continue,
            };

            if let Some(prev_tok) = maybe_prev_tok {
                if prev_tok.kind.is_adjective() || prev_tok.kind.is_determiner() {
                    // eprintln!(
                    //     "* Rejected '{}' because it follows the {} '{}'",
                    //     nountok_charsl.iter().collect::<String>(),
                    //     if prev_tok.kind.is_adjective() {
                    //         "adjective"
                    //     } else {
                    //         "determiner"
                    //     },
                    //     document.get_span_content(&prev_tok.span).iter().collect::<String>(),
                    // );
                    continue;
                }

                // "dictionary lookup" is not a mistakes but "couples breakup" is.
                if prev_tok.kind.is_noun() && !prev_tok.kind.is_plural_noun() {
                    // eprintln!(
                    //     "* Rejected '{}' because it follows the singular noun '{}",
                    //     nountok_charsl.iter().collect::<String>(),
                    //     document.get_span_content(&prev_tok.span).iter().collect::<String>(),
                    // );
                    continue;
                }
            }

            lints.push(Lint {
                span: Span::new(token.span.start, token.span.end),
                lint_kind: LintKind::WordChoice,
                suggestions: vec![Suggestion::ReplaceWith(phrasal_verb.chars().collect())],
                message: message.unwrap().to_string(),
                priority: 63,
            });
        }

        lints
    }

    fn description(&self) -> &str {
        "This rule looks for phrasal verbs written as compound nouns."
    }
}

#[cfg(test)]
mod tests {
    use super::PhrasalVerbAsCompoundNoun;
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    #[test]
    fn flag_breakup_and_workout() {
        assert_lint_count(
            "I will never breakup with Gym. We just seem to workout.",
            PhrasalVerbAsCompoundNoun::default(),
            2,
        );
    }

    #[test]
    fn correct_breakup_and_workout() {
        assert_suggestion_result(
            "I will never breakup with Gym. We just seem to workout.",
            PhrasalVerbAsCompoundNoun::default(),
            "I will never break up with Gym. We just seem to work out.",
        );
    }

    #[test]
    fn dont_flag_random_words_that_happen_to_end_like_a_particle() {
        assert_lint_count("I like bacon.", PhrasalVerbAsCompoundNoun::default(), 0);
    }

    #[test]
    fn dont_flag_non_verb_particles() {
        assert_lint_count("non", PhrasalVerbAsCompoundNoun::default(), 0);
    }

    #[test]
    fn correct_after_i() {
        assert_suggestion_result(
            "I backup",
            PhrasalVerbAsCompoundNoun::default(),
            "I back up",
        );
    }

    #[test]
    fn correct_after_we() {
        assert_suggestion_result(
            "we breakup",
            PhrasalVerbAsCompoundNoun::default(),
            "we break up",
        );
    }

    #[test]
    fn dont_flag_checkin() {
        // It's actually not a noun in English.
        assert_lint_count("checkin", PhrasalVerbAsCompoundNoun::default(), 0);
    }

    #[test]
    fn dont_flag_cleanup() {
        assert_lint_count("cleanup", PhrasalVerbAsCompoundNoun::default(), 0);
    }

    #[test]
    fn correct_after_you_lowercase() {
        assert_suggestion_result(
            "you checkout",
            PhrasalVerbAsCompoundNoun::default(),
            "you check out",
        );
    }

    #[test]
    fn correct_after_you_capitalized() {
        assert_suggestion_result(
            "You checkout",
            PhrasalVerbAsCompoundNoun::default(),
            "You check out",
        );
    }

    #[test]
    fn flag_checkout_after_you() {
        assert_lint_count("you checkout", PhrasalVerbAsCompoundNoun::default(), 1);
    }

    #[test]
    fn correct_after_they_lowercase() {
        assert_suggestion_result(
            "they cleanup",
            PhrasalVerbAsCompoundNoun::default(),
            "they clean up",
        );
    }

    #[test]
    fn flag_cleanup_after_they() {
        assert_lint_count("they cleanup", PhrasalVerbAsCompoundNoun::default(), 1);
    }

    #[test]
    fn dont_flag_dictionary_lookup() {
        assert_lint_count("dictionary lookup", PhrasalVerbAsCompoundNoun::default(), 0);
    }

    #[test]
    fn flag_couples_breakup() {
        assert_lint_count("couples breakup", PhrasalVerbAsCompoundNoun::default(), 1);
    }

    #[test]
    fn dont_flag_gallon() {
        assert_lint_count("gallon", PhrasalVerbAsCompoundNoun::default(), 0);
    }

    // Maybe this works by accident because "given" is also an adjective.
    // It should be because "funding" is a noun, but it's a gerund, which makes it also a verb.
    // Still, "given start up" doesn't make sense so maybe this test if fine.
    #[test]
    fn dont_flag_startup_funding() {
        assert_lint_count(
            "Yarvin has actually given startup funding. They hang out and party together",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_huge_markup() {
        assert_lint_count(
            "Sell it back to Russia at a huge markup.",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_another_layoff() {
        assert_lint_count(
            "And now just announced another layoff",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    // "Shakedown" is a compound noun -- it's part of a comma-separated list with another noun "threat"
    // But this is not easy to check for so is not implemented yet.
    // #[test]
    // fn dont_flag_a_threat_or_shakedown() {
    //     assert_lint_count(
    //         "Just a threat or Shakedown.",
    //         PhrasalVerbAsCompoundNoun::default(),
    //         0,
    //     );
    // }

    #[test]
    fn dont_flag_a_flyover() {
        assert_lint_count(
            "if I'm the Brits I'm doing a flyover",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_mafia_style_shakedown() {
        assert_lint_count(
            "Basically it's kind of a mafia style shakedown of Ukraine",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_my_meetup_repository() {
        assert_lint_count(
            "I might have in my Meetup repository",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }
}
