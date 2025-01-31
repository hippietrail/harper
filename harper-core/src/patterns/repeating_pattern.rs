use super::Pattern;
use crate::Token;

/// A pattern that will match one or more repetitions of the same pattern.
///
/// Somewhat reminiscent of the `.*` operator in Regex.
pub struct RepeatingPattern {
    inner: Box<dyn Pattern>,
    required_repetitions: usize,
}

impl RepeatingPattern {
    pub fn new(pattern: Box<dyn Pattern>, required_repetitions: usize) -> Self {
        Self {
            inner: pattern,
            required_repetitions,
        }
    }
}

impl Pattern for RepeatingPattern {
    fn matches(&self, tokens: &[Token], source: &[char]) -> usize {
        let mut tok_cursor = 0;
        let mut repetition = 0;

        loop {
            let match_len = self.inner.matches(&tokens[tok_cursor..], source);

            if match_len == 0 {
                if repetition >= self.required_repetitions {
                    return tok_cursor;
                } else {
                    return 0;
                }
            } else {
                tok_cursor += match_len;
                repetition += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RepeatingPattern;
    use crate::patterns::{AnyPattern, Pattern};
    use crate::Document;

    #[test]
    fn matches_anything() {
        let doc = Document::new_plain_english_curated(
            "This matcher will match the entirety of any document!",
        );
        let pat = RepeatingPattern::new(Box::new(AnyPattern), 0);

        assert_eq!(
            pat.matches(doc.get_tokens(), doc.get_source()),
            doc.get_tokens().len()
        )
    }

    #[test]
    fn does_not_match_short() {
        let doc = Document::new_plain_english_curated("No match");
        let pat = RepeatingPattern::new(Box::new(AnyPattern), 4);

        assert_eq!(pat.matches(doc.get_tokens(), doc.get_source()), 0)
    }
}
