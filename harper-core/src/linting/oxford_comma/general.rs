use crate::{
    Document, Token, TokenStringExt,
    patterns::{NounPhrase, Pattern, SequencePattern, WordSet},
};

use super::super::{Lint, LintKind, Linter, Suggestion};

pub struct General {
    pattern: SequencePattern,
}

impl General {
    pub fn new() -> Self {
        let pattern = SequencePattern::default()
            .then_one_or_more(
                SequencePattern::default()
                    .then(NounPhrase)
                    .then_comma()
                    .then_whitespace(),
            )
            .then(NounPhrase)
            .then_whitespace()
            .then(WordSet::new(&["and", "or", "nor"]))
            .then_whitespace()
            .then(NounPhrase);

        Self { pattern }
    }

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

impl Default for General {
    fn default() -> Self {
        Self::new()
    }
}

impl Linter for General {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut lints = Vec::new();

        for sentence in document.iter_sentences() {
            let mut tok_cursor = 0;

            if let Some(first) = sentence
                .first()
                .and_then(|t| t.kind.as_word().cloned())
                .flatten()
            {
                if first.preposition {
                    tok_cursor = sentence
                        .iter()
                        .position(|t| t.kind.is_comma())
                        .unwrap_or(sentence.iter().len())
                }
            }

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
        "Covers the general cases of missing Oxford commas."
    }
}
