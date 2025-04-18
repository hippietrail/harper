use crate::{
    Token,
    linting::{Lint, LintKind, Suggestion},
    patterns::{Pattern, SequencePattern, WordSet},
};

use crate::linting::PatternLinter;

/// See also:
/// harper-core/src/linting/compound_nouns/implied_ownership_compound_nouns.rs
/// harper-core/src/linting/lets_confusion/mod.rs
/// harper-core/src/linting/lets_confusion/let_us_redundancy.rs
/// harper-core/src/linting/pronoun_contraction/should_contract.rs
pub struct NoContractionWithVerb {
    pattern: Box<dyn Pattern>,
}

impl Default for NoContractionWithVerb {
    fn default() -> Self {
        let pattern = SequencePattern::default()
            .then(WordSet::new(&["lets", "let"]))
            .then_whitespace()
            .then(|tok: &Token, source: &[char]| {
                let Some(Some(meta)) = tok.kind.as_word() else {
                    return false;
                };

                if !meta.is_verb() || meta.is_noun() || meta.is_adjective() {
                    return false;
                }

                let tok_chars = tok.span.get_content(source);

                // If it ends with 'ing' and is at least 5 chars long, it could be a gerund or past participle
                // TODO: replace with metadata check when affix system supports verb forms
                if tok_chars.len() < 5 {
                    return true;
                }

                let is_ing_form = tok_chars
                    .iter()
                    .skip(tok_chars.len() - 3)
                    .map(|&c| c.to_ascii_lowercase())
                    .collect::<Vec<_>>()
                    .ends_with(&['i', 'n', 'g']);

                !is_ing_form
            });

        Self {
            pattern: Box::new(pattern),
        }
    }
}

impl PatternLinter for NoContractionWithVerb {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched_tokens: &[Token], source: &[char]) -> Option<Lint> {
        let problem_span = matched_tokens.first()?.span;
        let template = problem_span.get_content(source);

        Some(Lint {
            span: problem_span,
            lint_kind: LintKind::WordChoice,
            suggestions: vec![
                Suggestion::replace_with_match_case_str("let's", template),
                Suggestion::replace_with_match_case_str("let us", template),
            ],
            message: "It seems you forgot to include a subject here.".to_owned(),
            priority: 31,
        })
    }

    fn description(&self) -> &'static str {
        "Make sure you include a subject when giving permission to it."
    }
}

#[cfg(test)]
mod tests {
    use super::NoContractionWithVerb;
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    // Corrections

    #[test]
    fn fix_lets_inspect() {
        assert_suggestion_result(
            "In the end lets inspect with git-blame the results.",
            NoContractionWithVerb::default(),
            "In the end let's inspect with git-blame the results.",
        );
    }

    // False positives where verb is also a noun

    #[test]
    fn dont_flag_let_chance() {
        assert_lint_count("Let chance decide", NoContractionWithVerb::default(), 0);
    }

    #[test]
    fn dont_flag_let_time() {
        assert_lint_count(
            "Let time granularity be parametrized",
            NoContractionWithVerb::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_lets_staff() {
        assert_lint_count(
            "A plugin that backs up player's inventories and lets staff restore them or export it as a shulker.",
            NoContractionWithVerb::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_lets_time() {
        assert_lint_count(
            "This is very different than demo recording, which just simulates a network level connection and lets time move at its own rate.",
            NoContractionWithVerb::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_lets_play() {
        assert_lint_count(
            "Sometimes the umpire lets play continue",
            NoContractionWithVerb::default(),
            0,
        );
    }

    // False positives where verb is a gerund/past participle

    #[test]
    fn dont_flag_let_sleeping() {
        assert_lint_count(
            "Let sleeping logs lie.",
            NoContractionWithVerb::default(),
            0,
        );
    }

    // False positives where verb is also an adjective

    #[test]
    fn dont_flag_let_processed() {
        assert_lint_count(
            "Let processed response be a new structure analogous to server auction response.",
            NoContractionWithVerb::default(),
            0,
        );
    }
}
