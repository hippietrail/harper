use serde::{Deserialize, Serialize};

use crate::Span;

use super::{LintKind, Suggestion};

/// An error found in text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lint {
    /// The location in the source text the error lies.
    /// Important for automatic lint resolution through [`Self::suggestions`].
    pub span: Span,
    /// The general category the lint belongs to.
    /// Mostly used for UI elements in integrations.
    pub lint_kind: LintKind,
    /// A list of zero or more suggested edits that would resolve the underlying problem.
    /// See [`Suggestion`].
    pub suggestions: Vec<Suggestion>,
    /// A message to be displayed to the user describing the specific error found.
    ///
    /// You may use the [`format`] macro to generate more complex messages.
    pub message: String,
    /// A numerical value for the importance of a lint.
    /// Lower = more important.
    pub priority: u8,
}

impl Default for Lint {
    fn default() -> Self {
        Self {
            span: Default::default(),
            lint_kind: Default::default(),
            suggestions: Default::default(),
            message: Default::default(),
            priority: 127,
        }
    }
}
