use super::{Lint, LintKind, Linter, Suggestion};
use crate::{Document, OrdinalSuffix, Span, TokenKind};
use crate::{Number, TokenStringExt};

/// Detect incorrect number suffix (e.g. "2st").
#[derive(Debug, Clone, Copy, Default)]
pub struct CorrectNumberSuffix;

impl Linter for CorrectNumberSuffix {
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
                && let Some(correct_suffix) = OrdinalSuffix::correct_suffix_for(value)
                && suffix != correct_suffix
            {
                output.push(Lint {
                    span: suffix_span,
                    lint_kind: LintKind::Miscellaneous,
                    message: "This number needs a different suffix to sound right.".to_string(),
                    suggestions: vec![Suggestion::ReplaceWith(correct_suffix.to_chars())],
                    ..Default::default()
                })
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
    use super::CorrectNumberSuffix;
    use crate::linting::tests::assert_lint_count;

    #[test]
    fn passes_correct_cases() {
        assert_lint_count("2nd", CorrectNumberSuffix, 0);
        assert_lint_count("101st", CorrectNumberSuffix, 0);
        assert_lint_count("1012th", CorrectNumberSuffix, 0);
    }

    #[test]
    fn detects_incorrect_cases() {
        assert_lint_count("2st", CorrectNumberSuffix, 1);
        assert_lint_count("101nd", CorrectNumberSuffix, 1);
        assert_lint_count("1012rd", CorrectNumberSuffix, 1);
    }
}
