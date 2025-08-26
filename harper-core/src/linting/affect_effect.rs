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

    // legit affects - verb 3rd person singular
    
    #[test]
    fn dont_flag_this_value_directly_affects() {
        assert_no_lints("This value directly affects the execution time of this workflow", AffectEffect);
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
}
