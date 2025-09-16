use crate::{Token, TokenStringExt};

use super::Step;

/// A [`Step`] which will match only if the cursor is over the first word-like of a token stream.
/// It will return that token.
pub struct AnchorStart;

impl Step for AnchorStart {
    fn step(&self, tokens: &[Token], cursor: usize, _source: &[char]) -> Option<isize> {
        if tokens.iter_word_like_indices().next() == Some(cursor) {
            Some(0)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::ExprExt;
    use crate::linting::tests::SpanVecExt;
    use crate::{Document, Span};

    use super::AnchorStart;

    #[test]
    fn matches_first_word() {
        let document = Document::new_markdown_default_curated("This is a test.");
        let matches: Vec<_> = AnchorStart.iter_matches_in_doc(&document).collect();

        eprintln!("❤️ {:#?}", matches.to_strings(&document));
        assert_eq!(matches, vec![Span::new(0, 0)])
    }

    #[test]
    fn does_not_match_empty() {
        let document = Document::new_markdown_default_curated("");
        let matches: Vec<_> = AnchorStart.iter_matches_in_doc(&document).collect();

        eprintln!("🍏 {:#?}", matches.to_strings(&document));
        assert_eq!(matches, vec![])
    }

    #[test]
    fn matches_first_word_after_whitespace() {
        let document = Document::new_plain_english_curated(" This is another test.");
        let matches: Vec<_> = AnchorStart.iter_matches_in_doc(&document).collect();

        eprintln!("⭐️ {:#?}", matches);
        eprintln!("⭐️ {:#?}", matches.to_strings(&document));
        assert_eq!(matches, vec![Span::new(1, 1)])
    }

    #[test]
    fn doesnt_matche_whitespace_or_punctuation() {
        let document = Document::new_plain_english_curated(" ~ \t . \n , ; ' @ ! ");
        let matches: Vec<_> = AnchorStart.iter_matches_in_doc(&document).collect();

        eprintln!("🍊 {:#?}", matches.to_strings(&document));
        assert_eq!(matches, vec![])
    }
}
