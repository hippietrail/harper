use std::fmt::Display;

use is_macro::Is;
use serde::{Deserialize, Serialize};

use crate::Span;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lint {
    pub span: Span,
    pub lint_kind: LintKind,
    pub suggestions: Vec<Suggestion>,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Is, Default)]
pub enum LintKind {
    Spelling,
    Capitalization,
    Style,
    Formatting,
    Repetition,
    Enhancement,
    Readability,
    WordChoice,
    #[default]
    Miscellaneous,
}

impl Display for LintKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LintKind::Spelling => "Spelling",
            LintKind::Capitalization => "Capitalization",
            LintKind::Formatting => "Formatting",
            LintKind::Repetition => "Repetition",
            LintKind::Readability => "Readability",
            LintKind::Miscellaneous => "Miscellaneous",
            LintKind::Enhancement => "Enhancement",
            LintKind::WordChoice => "Word Choice",
            LintKind::Style => "Style",
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Is)]
pub enum Suggestion {
    ReplaceWith(Vec<char>),
    /// Insert the provided characters _after_ the offending text.
    InsertAfter(Vec<char>),
    Remove,
}

impl Suggestion {
    /// Apply a suggestion to a given text.
    pub fn apply(&self, span: Span, source: &mut Vec<char>) {
        match self {
            Self::ReplaceWith(chars) => {
                // Avoid allocation if possible
                if chars.len() == span.len() {
                    for (index, c) in chars.iter().enumerate() {
                        source[index + span.start] = *c
                    }
                } else {
                    let popped = source.split_off(span.start);

                    source.extend(chars);
                    source.extend(popped.into_iter().skip(span.len()));
                }
            }
            Self::Remove => {
                for i in span.end..source.len() {
                    source[i - span.len()] = source[i];
                }

                source.truncate(source.len() - span.len());
            }
            Self::InsertAfter(chars) => {
                let popped = source.split_off(span.end);
                source.extend(chars);
                source.extend(popped);
            }
        }
    }
}

impl Display for Suggestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suggestion::ReplaceWith(with) => {
                write!(f, "Replace with: “{}”", with.iter().collect::<String>())
            }
            Suggestion::InsertAfter(with) => {
                write!(f, "Insert “{}”", with.iter().collect::<String>())
            }
            Suggestion::Remove => write!(f, "Remove error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Span;

    use super::Suggestion;

    #[test]
    fn insert_comma_after() {
        let source = "This is a test";
        let mut source_chars = source.chars().collect();
        let sug = Suggestion::InsertAfter(vec![',']);
        sug.apply(Span::new(0, 4), &mut source_chars);

        assert_eq!(source_chars, "This, is a test".chars().collect::<Vec<_>>());
    }
}
