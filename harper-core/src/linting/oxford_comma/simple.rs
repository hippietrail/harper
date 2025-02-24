use crate::{
    Lrc, Token, TokenStringExt,
    linting::Linter,
    patterns::{OwnedPatternExt, Pattern, SequencePattern, WordSet},
};

use super::super::{Lint, LintKind, Suggestion};

pub struct Simple {
    pattern: Box<dyn Pattern>,
}

impl Default for Simple {
    fn default() -> Self {
        let item = Lrc::new(
            SequencePattern::default()
                .then_article()
                .then_whitespace()
                .then_noun()
                .or(Box::new(SequencePattern::default().then_noun())),
        );
        let item_chunk = SequencePattern::default()
            .then(item.clone())
            .then_comma()
            .then_whitespace();

        let pattern = SequencePattern::default()
            .then_one_or_more(item_chunk)
            .then(item.clone())
            .then_whitespace()
            .then(WordSet::new(&["and", "or", "nor"]))
            .then_whitespace()
            .then(item.clone());

        Self {
            pattern: Box::new(pattern),
        }
    }
}

impl Simple {
    fn match_to_lint(&self, matched_toks: &[Token], _source: &[char]) -> Option<Lint> {
        let conj_index = matched_toks.last_conjunction_index()?;
        let offender = matched_toks[conj_index - 2];

        Some(Lint {
            span: offender.span,
            lint_kind: LintKind::Style,
            suggestions: vec![Suggestion::InsertAfter(vec![','])],
            message: "An Oxford comma is necessary here.".to_owned(),
            priority: 31,
        })
    }
}

impl Linter for Simple {
    fn lint(&mut self, document: &crate::Document) -> Vec<crate::linting::Lint> {
        let mut lints = Vec::new();
        for sentence in document.iter_sentences() {
            let mut tok_cursor = 0;

            loop {
                if tok_cursor >= sentence.len() {
                    break;
                }

                let match_len = self
                    .pattern
                    .matches(&sentence[tok_cursor..], document.get_source());

                if match_len != 0 {
                    let lint = self.match_to_lint(
                        &sentence[tok_cursor..tok_cursor + match_len],
                        document.get_source(),
                    );

                    lints.extend(lint);
                    tok_cursor += match_len;
                } else {
                    tok_cursor += 1;
                }
            }
        }

        lints
    }

    fn description(&self) -> &str {
        "Covers the simple cases."
    }
}
