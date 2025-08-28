use crate::{
    CharStringExt, Document, Token, TokenStringExt,
    linting::{Lint, LintKind, Linter, Suggestion},
};

#[derive(Debug, Default)]
pub struct AffectEffect;

impl Linter for AffectEffect {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut output = Vec::new();

        for chunk in document.iter_chunks() {
            // for tok in chunk.iter_words() {
            for wix in chunk.iter_word_indices() {
                let tok = &chunk[wix];

                if !tok.kind.is_verb() && !tok.kind.is_noun() {
                    continue;
                }
                // < len of "seem" or > len of "affect" + "ing"
                if tok.span.len() < 4 || tok.span.len() > 6 + 3 {
                    continue;
                }
                let word = tok.span.get_content(document.get_source());
                if !word.eq_any_ignore_ascii_case_str(&[
                    "affect",
                    "affected",
                    "affects",
                    "affecting",
                    "effect",
                    "effected",
                    "effecting",
                    "effects",
                    "seam",
                    "seamed",
                    "seams",
                    "seaming",
                    "seem",
                    "seemed",
                    "seeming",
                    "seems",
                ]) {
                    continue;
                }

                enum Stem {
                    Æffect,
                    Seæm,
                }

                let (stem, stem_len) = match word.first() {
                    Some(&'a' | &'A' | &'e' | &'E') => (Stem::Æffect, 6),
                    Some(&'s' | &'S') => (Stem::Seæm, 4),
                    _ => continue,
                };

                eprintln!(
                    "❤️ '{}'",
                    tok.span.get_content_string(document.get_source())
                );

                let toks = chunk.widen_slice(wix, 2);

                if let Some(span) = toks.span() {
                    eprintln!("❤️ '{}'", span.get_content_string(document.get_source()));
                }

                let ending = &tok.span.get_content(document.get_source())[stem_len..];

                let with_ending = match stem {
                    Stem::Æffect => vec!['æ', 'f', 'f', 'e', 'c', 't'],
                    Stem::Seæm => vec!['s', 'e', 'æ', 'm'],
                }
                .into_iter()
                .chain(ending.iter().copied())
                .collect::<Vec<_>>();
                let message = format!("Did you mean `{}`?", with_ending.iter().collect::<String>());

                output.push(Lint {
                    span: tok.span,
                    lint_kind: LintKind::Spelling,
                    suggestions: vec![Suggestion::replace_with_match_case(with_ending, word)],
                    message,
                    priority: 63,
                })
            }
        }

        output
    }

    fn description(&self) -> &'static str {
        "Fixes mix-ups between `affect` and `effect`."
    }
}

#[cfg(test)]
mod tests {
    use super::AffectEffect;
    use crate::linting::tests::{assert_lint_count, assert_no_lints};

    // legit affect - verb - lemma

    #[test]
    fn dont_flag_every_code_change_might_affect_anything_else() {
        assert_no_lints("every code change might affect anything else", AffectEffect);
    }

    #[test]
    fn dont_flag_how_these_affect_the_ux() {
        assert_no_lints(
            "probably you’ll never be aware which are your slowest code parts under real-world scenario and how these affect the UX",
            AffectEffect,
        );
    }

    #[test]
    fn dont_flag_null_values_affect_performance() {
        assert_no_lints("How do null values affect performance?", AffectEffect);
    }

    #[test]
    fn dont_flag_only_affect_things_that_use_schannel_ssp() {
        assert_no_lints(
            "Changes made by this category only affect things that use [Schannel SSP]",
            AffectEffect,
        );
    }

    #[test]
    fn dont_flag_that_can_affect_performance() {
        assert_no_lints(
            "you might want to be aware of the options that can affect performance",
            AffectEffect,
        );
    }

    #[test]
    fn dont_flag_can_sometimes_affect_gameplay_too() {
        assert_no_lints(
            "but can sometimes affect gameplay too",
            AffectEffect::default(),
        );
    }

    // legit affected - verb - past participle

    #[test]
    fn dont_flag_is_not_affected() {
        assert_no_lints(
            "Local RDP such as for Hyper-V enhanced session is not affected.",
            AffectEffect,
        );
    }

    #[test]
    fn dont_flag_the_people_who_were_affected_by_this_tragedy() {
        assert_no_lints(
            "your help will be immensely valuable for the people who were affected by this tragedy",
            AffectEffect::default(),
        );
    }

    // legit affects - verb - 3rd person singular

    #[test]
    fn dont_flag_this_value_directly_affects() {
        assert_no_lints(
            "This value directly affects the execution time of this workflow",
            AffectEffect,
        );
    }

    // legit affecting - verb - present participle

    #[test]
    fn dont_flag_vulnerabilities_affecting_microsoft_products() {
        assert_no_lints(
            "The Microsoft Security Response Center (MSRC) investigates all reports of security vulnerabilities affecting Microsoft products and services",
            AffectEffect,
        );
    }

    // legit effect - noun - plural

    #[test]
    fn dont_flag_avoid_effects_outside_of_functions() {
        assert_no_lints("Avoid effects outside of functions.", AffectEffect);
    }

    #[test]
    fn dont_flag_code_with_effects_like_network_or_db_calls() {
        assert_no_lints(
            "Avoid putting code with effects like network or DB calls outside of functions.",
            AffectEffect,
        );
    }

    #[test]
    fn dont_flag_not_have_any_effect_on() {
        assert_no_lints("They do not have any effect on security.", AffectEffect);
    }

    #[test]
    fn dont_flag_wont_have_any_effect_on_you() {
        assert_no_lints(
            "seeing the source code won't have any effect on you because you aren't able to understand nor verify it.",
            AffectEffect,
        );
    }

    #[test]
    fn dont_flag_will_have_no_effect() {
        assert_no_lints(
            "Setting the variables after sourcing the script will have no effect",
            AffectEffect,
        );
    }

    #[test]
    fn dont_flag_if_a_step_causes_an_effect_to_be_executed() {
        assert_no_lints(
            "if a step causes an effect to be executed",
            AffectEffect::default(),
        );
    }

    #[test]
    fn dont_flag_an_effect_that_reaches_out() {
        assert_no_lints(
            "Currently our reducer is using an effect that reaches out into the real world to hit an API server",
            AffectEffect::default(),
        );
    }

    #[test]
    fn dont_flag_take_effect() {
        assert_no_lints(
            "please redeploy the project for the changes to take effect.",
            AffectEffect::default(),
        );
    }

    // legit effects - noun - plural

    #[test]
    fn dont_flag_side_effects() {
        assert_no_lints(
            "side effects influence your application",
            AffectEffect::default(),
        );
    }

    #[test]
    fn dont_flag_run_the_reducer_and_effects() {
        assert_no_lints(
            "so that the store can run the reducer and effects, and you can observe state changes in the store",
            AffectEffect::default(),
        );
    }

    #[test]
    fn dont_flag_any_effects_that_should() {
        assert_no_lints(
            "The reducer is also responsible for returning any effects that should be",
            AffectEffect::default(),
        );
    }

    #[test]
    fn and_what_effects_need_to_be_executed() {
        assert_no_lints(
            "current state to the next state, and what effects need to be executed",
            AffectEffect::default(),
        );
    }

    #[test]
    fn dont_flag_execute_effects() {
        assert_no_lints(
            "execute effects, and they can return `.none`",
            AffectEffect::default(),
        );
    }

    #[test]
    fn dont_flag_including_the_effects_without() {
        assert_no_lints(
            "And we can immediately test this logic, including the effects, without",
            AffectEffect::default(),
        );
    }

    #[test]
    fn dont_flag_specialized_effects_and_transformations() {
        assert_no_lints(
            "Create specialized effects and transformations for video generation",
            AffectEffect::default(),
        );
    }

    // legit seem - verb - lemma

    #[test]
    fn dont_flag_something_doesnt_seem_right() {
        assert_no_lints(
            "If you find something which doesn't make sense, or something doesn't seem right, please make a pull request",
            AffectEffect,
        );
    }

    // legit seems - verb - 3rd person singular

    #[test]
    fn dont_flag_it_seems_to_be_able_to() {
        assert_no_lints(
            "it seems to be able to handle a bit of general tasks such as",
            AffectEffect,
        );
    }

    #[test]
    fn dont_flag_what_seems_like_a_bug_might_be() {
        assert_no_lints(
            "What seems like a bug might be intended behaviour.",
            AffectEffect::default(),
        );
    }

    //////////////////////////////
}
