use std::sync::Arc;

use super::{Lint, LintKind, Linter, Suggestion};
use crate::spell::{Dictionary, FstDictionary};
use crate::{CharStringExt, Document, Span, TokenStringExt};

/// Detect phrasal verbs written as compound nouns.
pub struct PhrasalVerbAsCompoundNoun {
    dict: Arc<FstDictionary>,
}

enum Confidence {
    DefinitelyVerb,
    PossiblyVerb,
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
            // Instead, let's use a few heuristics.
            let token = document.get_token(i).unwrap();
            // * Can't also be a proper noun or a real verb.
            if token.kind.is_proper_noun() || token.kind.is_verb() {
                continue;
            }
            let nountok_charsl = document.get_span_content(&token.span);
            // * Can't contain space, hyphen or apostrophe
            if nountok_charsl.contains(&' ')
                || nountok_charsl.contains(&'-')
                || nountok_charsl.contains(&'\'')
                || nountok_charsl.contains(&'’')
            {
                continue;
            }

            let nountok_lower = nountok_charsl.to_lower();
            let nountok_lower = nountok_lower.as_ref();

            // * Must not be in the set of known false positives.
            if nountok_lower == ['g', 'a', 'l', 'l', 'o', 'n']
                || nountok_lower == ['d', 'r', 'a', 'g', 'o', 'n']
            {
                continue;
            }

            // * Must end with the same letters as one of the particles used in phrasal verbs.
            let particle_endings: &[&[char]] = &[
                &['a', 'r', 'o', 'u', 'n', 'd'],
                &['b', 'a', 'c', 'k'],
                &['d', 'o', 'w', 'n'],
                &['i', 'n'],
                &['o', 'n'],
                &['o', 'f', 'f'],
                &['o', 'u', 't'],
                &['o', 'v', 'e', 'r'],
                &['u', 'p'],
            ];

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
                    .is_some_and(|md| md.verb.is_some()),
                self.dict
                    .get_word_metadata_str(&phrasal_verb)
                    .is_some_and(|md| md.verb.is_some()),
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
            let maybe_prev_tok = document.get_next_word_from_offset(i, -1);
            let maybe_next_tok = document.get_next_word_from_offset(i, 1);

            // If it's in isolation, a compound noun is fine.
            if maybe_prev_tok.is_none() && maybe_next_tok.is_none() {
                continue;
            }

            let confidence = match (phrasal_verb_is_verb, verb_part_is_verb) {
                (true, _) => Confidence::DefinitelyVerb,
                (false, true) => Confidence::PossiblyVerb,
                _ => continue,
            };

            if let Some(prev_tok) = maybe_prev_tok {
                if prev_tok.kind.is_adjective() || prev_tok.kind.is_determiner() {
                    continue;
                }

                // "dictionary lookup" is not a mistake but "couples breakup" is.
                // But "settings plugin" is not.
                if prev_tok.kind.is_noun() && !prev_tok.kind.is_plural_noun()
                    || prev_tok
                        .span
                        .get_content(document.get_source())
                        .eq_ignore_ascii_case_str("settings")
                {
                    continue;
                }

                if is_part_of_noun_list(document, i) {
                    continue;
                }

                // If the previous word is (only) a preposition, this word is surely a noun
                if prev_tok.kind.is_preposition()
                    && !prev_tok
                        .span
                        .get_content(document.get_source())
                        .eq_ignore_ascii_case_str("to")
                {
                    continue;
                }

                // If the previous word is OOV, those are most commonly nouns
                if prev_tok.kind.is_oov() {
                    continue;
                }
            }

            // If the compound noun is followed by another noun, check for larger compound nouns.
            if let Some(next_tok) = maybe_next_tok.filter(|tok| tok.kind.is_noun())
                && match nountok_lower {
                    ['b', 'a', 'c', 'k', 'u', 'p'] => {
                        &["file", "images", "location", "snapshots"][..]
                    }
                    ['c', 'a', 'l', 'l', 'b', 'a', 'c', 'k'] => &["function"][..],
                    ['l', 'a', 'y', 'o', 'u', 't'] => &["estimation"][..],
                    ['p', 'l', 'a', 'y', 'b', 'a', 'c', 'k'] => &["latency"][..],
                    ['r', 'o', 'l', 'l', 'o', 'u', 't'] => &["status"][..],
                    ['w', 'o', 'r', 'k', 'o', 'u', 't'] => &["constraints", "preference"][..],
                    _ => &[],
                }
                .contains(
                    &next_tok
                        .span
                        .get_content_string(document.get_source())
                        .to_lowercase()
                        .as_ref(),
                )
            {
                continue;
            }

            let message = match confidence {
                Confidence::DefinitelyVerb => {
                    "This word should be a phrasal verb, not a compound noun."
                }
                Confidence::PossiblyVerb => {
                    "This word might be a phrasal verb rather than a compound noun."
                }
            };

            lints.push(Lint {
                span: Span::new(token.span.start, token.span.end),
                lint_kind: LintKind::WordChoice,
                suggestions: vec![Suggestion::ReplaceWith(phrasal_verb.chars().collect())],
                message: message.to_string(),
                priority: 63,
            });
        }

        lints
    }

    fn description(&self) -> &str {
        "This rule looks for phrasal verbs written as compound nouns."
    }
}

/// Checks if the current token is part of a list of nouns
fn is_part_of_noun_list(document: &Document, current_index: usize) -> bool {
    // Check for a conjunction before the current word (-1 is whitespace, -2 is the conjunction)
    if !matches!(
        document.get_next_word_from_offset(current_index, -1),
        Some(tok) if tok.kind.is_conjunction()
    ) {
        return false;
    }

    // Check the token sequence before the conjunction
    match document.get_token_offset(current_index, -3) {
        // A comma without the space, assume we're in a list of nouns.
        Some(tok) if tok.kind.is_comma() => true,

        // Whitespace. If the token before that is a noun or a comma, assume we're in a list of nouns.
        Some(ws) if ws.kind.is_whitespace() => {
            document
                .get_token_offset(current_index, -4)
                // `noun and` or `, and`
                .is_some_and(|tok| tok.kind.is_noun() || tok.kind.is_comma())
        }

        _ => false,
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

    #[test]
    #[ignore = "\"Shakedown\" is a compound noun -- it's part of a comma-separated list with another noun \"threat\"\nBut this is not easy to check for so is not implemented yet."]
    fn dont_flag_a_threat_or_shakedown() {
        assert_lint_count(
            "Just a threat or Shakedown.",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

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

    #[test]
    fn ignore_multi_word() {
        assert_lint_count(
            "I like this add-on!",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_list_of_nouns_1298() {
        assert_lint_count(
            "A printable format and layout.",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_oov_nvim_plugin_1280() {
        assert_lint_count(
            "This is the nvim plugin for you.",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn flag_title_case() {
        assert_lint_count(
            "I Will Never Breakup With Gym. We Just Seem To Workout.",
            PhrasalVerbAsCompoundNoun::default(),
            2,
        );
    }

    #[test]
    fn dont_flag_all_caps() {
        assert_lint_count(
            "I WILL NEVER BREAKUP WITH GYM. WE JUST SEEM TO WORKOUT.",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn false_positive_issue_1495() {
        assert_lint_count(
            "Color schemes are available by using the Style Settings plugin.",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_thanks_a_lot_linter_description() {
        assert_lint_count(
            "Thanks a lot` is the fixed, widely accepted form, while variants like `thanks lot` or `thanks alot` are non-standard and can jar readers.",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_backup_location() {
        assert_lint_count(
            "Backup location: `%APPDATA%\\Cursor\\User\\globalStorage\\backups`",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_backup_plan() {
        assert_lint_count(
            "Every backup plan is unique, based on your risk assessment.",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_backup_program() {
        assert_lint_count(
            "restic is a backup program that is fast, efficient and secure",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_backup_solution_or_backup_problems() {
        assert_lint_count(
            "NPBackup is a multiparadigm backup solution which tries to solve two major backup problems",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_backup_utilities_backup_system_or_backup_snapshots() {
        assert_lint_count(
            "GitHub Enterprise Server Backup Utilities is a backup system you install on a separate host, which takes backup snapshots",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_backup_images() {
        assert_lint_count(
            "This App creates and stores backup images of your Nextcloud.",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn fix_backup_individual_apps() {
        assert_suggestion_result(
            "It requires root and allows you to backup individual apps and their data.",
            PhrasalVerbAsCompoundNoun::default(),
            "It requires root and allows you to back up individual apps and their data.",
        );
    }

    #[test]
    fn dont_flag_backup_strategy() {
        assert_lint_count(
            "This is for you if you want to quickly set up a backup strategy without much fuss.",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_helm_backup_plugin() {
        assert_lint_count(
            "Helm Backup Plugin.",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_callback_function() {
        assert_lint_count(
            "By the time the `setTimeout` callback function was invoked",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_playback_latency() {
        assert_lint_count(
            "Low-Latency HLS is a recently standardized variant of the protocol that allows to greatly reduce playback latency.",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_workout_constraints() {
        assert_lint_count(
            "Workout constraints",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_workout_preference() {
        assert_lint_count(
            "Workout preference",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_rollout_status() {
        assert_lint_count(
            "Rollout Status of Latest Image Release",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn font_flag_with_plugin() {
        assert_lint_count(
            "**Xcode** (8.0+, otherwise [with plugin](https://github.com/robertvojta/LigatureXcodePlugin))",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        )
    }

    #[test]
    fn dont_flag_and_layout_of_data() {
        assert_lint_count(
            "shape, memory space, and layout of data, while performing the complicated indexing for the user",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_in_noun_list_without_space_after_comma() {
        assert_lint_count(
            "shape, memory space,and layout of data",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_layout_estimation() {
        assert_lint_count(
            "Layout estimation focuses on predicting architectural elements, i.e., walls, doors, and windows, within an indoor scene.",
            PhrasalVerbAsCompoundNoun::default(),
            0,
        );
    }
}
