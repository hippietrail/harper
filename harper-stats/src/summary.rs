use std::{collections::HashMap, fmt::Display};

use harper_core::linting::LintKind;

pub struct Summary {
    lint_counts: HashMap<LintKind, usize>,
}

impl Summary {
    pub fn new() -> Self {
        Self {
            lint_counts: HashMap::new(),
        }
    }

    /// Increment the count for a particular lint kind.
    pub fn inc_lint_count(&mut self, kind: LintKind) {
        self.lint_counts
            .entry(kind)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    /// Get the count for a particular lint kind.
    pub fn get(&self, kind: LintKind) -> usize {
        self.lint_counts.get(&kind).copied().unwrap_or(0)
    }
}

impl Default for Summary {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (kind, count) in &self.lint_counts {
            writeln!(f, "{kind}\t{count}")?;
        }

        Ok(())
    }
}
