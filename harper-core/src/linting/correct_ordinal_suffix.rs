use super::{Lint, LintKind, Linter, Suggestion};
use crate::{Document, NumberSuffix, Span, TokenKind};
use crate::{Number, TokenStringExt};

/// Detect and warn that the sentence is too long.
#[derive(Debug, Clone, Copy, Default)]
pub struct CorrectOrdinalSuffix;

impl Linter for CorrectOrdinalSuffix {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut output = Vec::new();

        for number_tok in document.iter_numbers() {
            let Some(suffix_span) = Span::new_with_len(number_tok.span.end, 2).pulled_by(2) else {
                continue;
            };

            if let TokenKind::Number(Number {
                value,
                suffix: Some(suffix),
                ..
            }) = number_tok.kind
            {
                // Verify that the suffix is actually attached to the number
                let number_span = number_tok.span;
                let suffix_start = number_span.end;
                let suffix_end = suffix_start + 2;
                
                // Create a span for the suffix
                let suffix_span = Span::new(suffix_start, suffix_end);
                
                // Get the characters at the suffix position
                let suffix_chars = document.get_span_content(&suffix_span);
                
                // Only check if it's exactly 2 characters long and is an ordinal suffix
                if suffix_chars.len() != 2 || NumberSuffix::from_chars(suffix_chars).is_none() {
                    continue;
                }

                eprintln!("suffix span: '{:?}'", suffix_chars);
                eprintln!("value: '{:?}', suffix: '{:?}'", value, suffix);
                if let Some(correct_suffix) = NumberSuffix::correct_suffix_for(value) {
                    if suffix != correct_suffix {
                        output.push(Lint {
                            span: suffix_span,
                            lint_kind: LintKind::Miscellaneous,
                            message: "This number needs a different ordinal suffix to sound right."
                                .to_string(),
                            suggestions: vec![Suggestion::ReplaceWith(correct_suffix.to_chars())],
                            ..Default::default()
                        })
                    }
                }
            }
        }

        output
    }

    fn description(&self) -> &'static str {
        "When making quick edits, it is common for authors to change the value of a number without changing its suffix. This rule looks for these cases, for example: `2st`."
    }
}

#[cfg(test)]
mod tests {
    use super::CorrectOrdinalSuffix;
    use crate::linting::tests::assert_lint_count;

    #[test]
    fn passes_correct_cases() {
        assert_lint_count("2nd", CorrectOrdinalSuffix, 0);
        assert_lint_count("101st", CorrectOrdinalSuffix, 0);
        assert_lint_count("1012th", CorrectOrdinalSuffix, 0);
    }

    #[test]
    fn detects_incorrect_cases() {
        assert_lint_count("2st", CorrectOrdinalSuffix, 1);
        assert_lint_count("101nd", CorrectOrdinalSuffix, 1);
        assert_lint_count("1012rd", CorrectOrdinalSuffix, 1);
    }
}
