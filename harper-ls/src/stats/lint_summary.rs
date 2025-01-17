use std::collections::HashMap;

use harper_core::linting::LintKind;

pub struct LintSummary {
    counts: HashMap<LintKind, usize>,
}

impl LintSummary {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    /// Increment the count for a particular lint kind.
    pub fn inc(&mut self, kind: LintKind) {
        self.counts
            .entry(kind)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    /// Get the count for a particular lint kind.
    pub fn get(&self, kind: LintKind) -> usize {
        self.counts.get(&kind).copied().unwrap_or(0)
    }
}
