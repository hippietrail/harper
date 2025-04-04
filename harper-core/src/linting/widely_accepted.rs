use crate::{
    Token,
    linting::{Lint, LintKind, PatternLinter, Suggestion},
    patterns::{AnyCapitalization, Pattern, SequencePattern, WordSet},
};

pub struct WidelyAccepted {
    pattern: SequencePattern,
}

impl Default for WidelyAccepted {
    fn default() -> Self {
        let pattern = SequencePattern::default()
            .then(AnyCapitalization::new("wide".chars().collect()))
            .then_whitespace()
            .then(WordSet::new(&["accepted", "acceptable", "used"]));

        Self { pattern }
    }
}

impl PatternLinter for WidelyAccepted {
    fn pattern(&self) -> &dyn Pattern {
        &self.pattern
    }

    fn match_to_lint(&self, matched_tokens: &[Token], _source: &[char]) -> Option<Lint> {
        // We only need to replace the `wide` token with `widely`.
        let wide_token = matched_tokens.first()?;

        Some(Lint {
            span: wide_token.span,
            lint_kind: LintKind::Miscellaneous,
            message: "Use the adverb `widely` in this context. For example, `widely accepted` or `widely used` is standard usage."
                .to_owned(),
            suggestions: vec![Suggestion::ReplaceWith("widely".chars().collect())],
            priority: 31,
        })
    }

    fn description(&self) -> &str {
        "Flags `wide accepted`, `wide acceptable`, or `wide used` and recommends switching `wide` to the adverb `widely`."
    }
}
