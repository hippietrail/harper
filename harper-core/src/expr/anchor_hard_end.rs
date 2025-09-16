use crate::Token;
use crate::token_string_ext::TokenStringExt;

use super::Step;

/// A [`Step`] which will match only if the cursor is at the end of a token stream.
pub struct AnchorHardEnd;

impl Step for AnchorHardEnd {
    fn step(&self, tokens: &[Token], cursor: usize, source: &[char]) -> Option<isize> {
        eprintln!("🪜 '{}'", tokens[cursor..].span()?.get_content_string(source));
        if cursor == tokens.len() {
            eprintln!("** c={}, end without punctuation **", cursor);
            Some(0)
        } else if let Some(last) = tokens.last()
            && last.kind.is_chunk_terminator()
        {
            eprintln!("** c={}, end with punctuation **", cursor);
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

    use super::AnchorHardEnd;

    #[test]
    fn matches_period() {
        let document = Document::new_markdown_default_curated("This is a test.");
        let matches: Vec<_> = AnchorHardEnd.iter_matches_in_doc(&document).collect();

        eprintln!("🎂 {:#?}", matches.to_strings(&document));
        // assert_eq!(matches, vec![Span::new(7, 7)])
    }

    #[test]
    fn does_not_match_empty() {
        let document = Document::new_markdown_default_curated("");
        let matches: Vec<_> = AnchorHardEnd.iter_matches_in_doc(&document).collect();

        eprintln!("🌭 {:#?}", matches.to_strings(&document));
        // assert_eq!(matches, vec![])
    }

    #[test]
    fn matches_space() {
        let document = Document::new_plain_english_curated(" ");
        eprintln!("«{}»", document.get_full_string());
        let matches: Vec<_> = AnchorHardEnd.iter_matches_in_doc(&document).collect();

        eprintln!("🐍 {:#?}", matches.to_strings(&document));
    }

    #[test]
    fn matches_comma() {
        let document = Document::new_plain_english_curated(",");
        let matches: Vec<_> = AnchorHardEnd.iter_matches_in_doc(&document).collect();

        eprintln!("🎀 {:#?}", matches.to_strings(&document));
    }

    #[test]
    fn matches_semicolon() {
        let document = Document::new_plain_english_curated(";");
        let matches: Vec<_> = AnchorHardEnd.iter_matches_in_doc(&document).collect();

        eprintln!("🚛 {:#?}", matches.to_strings(&document));
    }

    #[test]
    fn matches_full_stop() {
        let document = Document::new_plain_english_curated(".");
        let matches: Vec<_> = AnchorHardEnd.iter_matches_in_doc(&document).collect();

        eprintln!("🫥 {:#?}", matches.to_strings(&document));
    }
}
