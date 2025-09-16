use crate::{Token};

use super::Step;

/// A [`Step`] which will match only if the cursor is at the start of a token stream.
pub struct AnchorHardStart;

impl Step for AnchorHardStart {
    fn step(&self, _tokens: &[Token], cursor: usize, _source: &[char]) -> Option<isize> {
        if cursor == 0 {
            Some(0)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::ExprExt;
    use crate::{Document, Span};

    use super::AnchorHardStart;

    #[test]
    fn matches_first_word() {
        let document = Document::new_markdown_default_curated("This is a test.");
        let matches: Vec<_> = AnchorHardStart.iter_matches_in_doc(&document).collect();

        assert_eq!(matches, vec![Span::new(0, 0)])
    }

    #[test]
    fn does_not_match_empty() {
        let document = Document::new_markdown_default_curated("");
        let matches: Vec<_> = AnchorHardStart.iter_matches_in_doc(&document).collect();

        assert_eq!(matches, vec![])
    }

    #[test]
    fn matches_whitespace_before_first_word() {
        let document = Document::new_plain_english_curated(" This is another test.");
        let matches: Vec<_> = AnchorHardStart.iter_matches_in_doc(&document).collect();

        assert_eq!(matches, vec![Span::new(0, 0)])
    }

    #[test]
    fn matches_whitespace() {
        let document = Document::new_plain_english_curated(" ");
        let matches: Vec<_> = AnchorHardStart.iter_matches_in_doc(&document).collect();

        assert_eq!(matches, vec![Span::new(0, 0)])
    }
}
