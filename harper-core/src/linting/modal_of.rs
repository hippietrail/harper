use crate::{
    patterns::{OwnedPatternExt, Pattern, SequencePattern, WordSet},
    Lrc, Token, TokenStringExt,
};

use super::{Lint, LintKind, PatternLinter, Suggestion};

pub struct ModalOf {
    pattern: Box<dyn Pattern>,
}

impl Default for ModalOf {
    fn default() -> Self {
        let modals = ["could", "had", "might", "must", "should", "would"];
        let mut words = WordSet::all(&modals);
        modals.iter().for_each(|word| {
            words.add(&format!("{}n't", word));
        });

        let modal_of = Lrc::new(
            SequencePattern::default()
                .then(Box::new(words))
                .then_whitespace()
                .then_exact_word("of"),
        );

        let ws_course = Lrc::new(
            SequencePattern::default()
                .then_whitespace()
                .then_exact_word("course"),
        );

        Self {
            pattern: Box::new(
                SequencePattern::default()
                    .then(Box::new(modal_of.clone()))
                    .then(Box::new(ws_course.clone()))
                    .or(Box::new(modal_of.clone())),
            ),
        }
    }
}

impl PatternLinter for ModalOf {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched_toks: &[Token], source_chars: &[char]) -> Option<Lint> {
        if matched_toks.len() != 3 {
            return None;
        }

        let span_modal_of = matched_toks[0..3].span().unwrap();
        let span_modal = matched_toks[0].span;

        let modal_have = format!("{} have", span_modal.get_content_string(source_chars))
            .chars()
            .collect();
        let modal_ws_of = span_modal_of.get_content(source_chars);

        Some(Lint {
            span: span_modal_of,
            lint_kind: LintKind::WordChoice,
            suggestions: vec![Suggestion::replace_with_match_case(modal_have, modal_ws_of)],
            message: "Use `have` rather than `of` here.".to_string(),
            priority: 126,
        })
    }

    fn description(&self) -> &'static str {
        "Detects `would of`, `could of, `should of`, etc."
    }
}

#[cfg(test)]
mod tests {
    use super::ModalOf;
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    // a lint_count test
    #[test]
    fn test_could_of() {
        assert_lint_count("could of", ModalOf::default(), 1);
    }

    // a suggestion result test
    #[test]
    fn test_could_of_suggestion() {
        assert_suggestion_result("could of", ModalOf::default(), "have");
    }
}
