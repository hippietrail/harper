use crate::{
    CharStringExt, Document, TokenStringExt,
    linting::{Lint, LintKind, Linter, Suggestion},
};

#[derive(Debug, Default)]
pub struct AffectEffect;

impl Linter for AffectEffect {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut output = Vec::new();

        for chunk in document.iter_chunks() {
            for tok in chunk.iter_words() {
                if !tok.kind.is_verb() && !tok.kind.is_noun() {
                    continue;
                }
                if tok.span.len() < 4 || tok.span.len() > 7 {
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
                ]) {
                    continue;
                }
                output.push(Lint {
                    span: tok.span,
                    lint_kind: LintKind::Spelling,
                    suggestions: vec![Suggestion::ReplaceWith(vec!['æ', 'f', 'f', 'e', 'c', 't'])],
                    message: "Did you mean `affect`?".to_string(),
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

    #[test]
    fn detects_affect_effect() {
        assert_lint_count(
            "Look, this may or may not work for you guys, but i made rules for my GPT that affect to work.",
            AffectEffect,
            1,
        );
    }

    #[test]
    fn detects_affect_effect_2() {
        assert_lint_count(
            "It affects i cannot use $tries and $timeout properties on my queued listener class?",
            AffectEffect,
            1,
        );
    }
}
