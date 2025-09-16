use crate::Token;

use super::Step;
use crate::token_string_ext::TokenStringExt;

/// A [`Step`] which will match only if the cursor is over the last non-whitespace character in stream.
/// It will return that token.
///
/// For example, if you built `SequenceExpr::default().t_aco("word").then(AnchorEnd)` and ran it on `This is a word`, the resulting `Span` would only cover the final word token.
pub struct AnchorEnd;

impl Step for AnchorEnd {
    fn step(&self, tokens: &[Token], cursor: usize, _source: &[char]) -> Option<isize> {
        eprintln!("⁉️ '{}' ⁉️ '{}' ⁉️",
            tokens[0..cursor].span()?.get_content_string(_source),
            tokens[cursor..].span()?.get_content_string(_source)
        );
        // if tokens.len() == cursor {
        //     eprintln!("doc end");
        //     Some(0)
        // } else if tokens
        //     .iter()
        //     .enumerate()
        //     .rev()
        //     .filter(|(_, t)| !t.kind.is_whitespace() && !t.kind.is_chunk_terminator())
        //     .map(|(i, _)| i)
        //     .next()
        //     == Some(cursor)
        // {
        //     eprintln!("chunk end");
        //     Some(0)
        // } else {
        //     eprintln!("none");
        //     None
        // }
        if cursor == tokens.len() {
            // if we're at the very end of the document, we match
            eprintln!("doc end");
            return Some(0);
        } else if let Some(last) = tokens.last()
            && last.kind.is_chunk_terminator()
        {
            // if we're at the last token and it's a chunk terminator, we match
            eprintln!("** end with punctuation **");
            return Some(0);
        }
        for i in (0..cursor).rev() {
            if !tokens[i].kind.is_whitespace() {
                return Some(0);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::ExprExt;
    use crate::linting::tests::SpanVecExt;
    use crate::{Document, Span};

    use super::AnchorEnd;

    #[test]
    fn matches_period() {
        let document = Document::new_markdown_default_curated("This is a test.");
        let matches: Vec<_> = AnchorEnd.iter_matches_in_doc(&document).collect();

        assert_eq!(matches, vec![Span::new(7, 7)])
    }

    #[test]
    fn does_not_match_empty() {
        let document = Document::new_markdown_default_curated("");
        let matches: Vec<_> = AnchorEnd.iter_matches_in_doc(&document).collect();

        assert_eq!(matches, vec![])
    }

    #[test]
    fn space() {
        let document = Document::new_plain_english_curated(" ");
        let matches: Vec<_> = AnchorEnd.iter_matches_in_doc(&document).collect();

        eprintln!("⚖️ {:#?} {:?}", matches, matches.to_strings(&document));
    }

    #[test]
    fn period() {
        let document = Document::new_plain_english_curated(".");
        let matches: Vec<_> = AnchorEnd.iter_matches_in_doc(&document).collect();

        eprintln!("🎹 {:#?} {:?}", matches, matches.to_strings(&document));
    }

    #[test]
    fn period_space() {
        let document = Document::new_plain_english_curated(". ");
        let matches: Vec<_> = AnchorEnd.iter_matches_in_doc(&document).collect();

        eprintln!("🪐 {:#?} {:?}", matches, matches.to_strings(&document));
    }

    #[test]
    fn period_a() {
        let document = Document::new_plain_english_curated(".a");
        let matches: Vec<_> = AnchorEnd.iter_matches_in_doc(&document).collect();

        eprintln!("🥗 {:#?} {:?}", matches, matches.to_strings(&document));
    }

    #[test]
    fn a_dot_b() {
        let document = Document::new_plain_english_curated("a.b");
        let matches: Vec<_> = AnchorEnd.iter_matches_in_doc(&document).collect();

        eprintln!("🐝 {:#?} {:?}", matches, matches.to_strings(&document));
    }

    #[test]
    fn abc() {
        let document = Document::new_plain_english_curated("a. b . c ");
        let matches: Vec<_> = AnchorEnd.iter_matches_in_doc(&document).collect();

        eprintln!("🌊 {:#?} {:?}", matches, matches.to_strings(&document));
    }

    #[test]
    fn this_and_that() {
        let document = Document::new_plain_english_curated("this one, and this one, and that's it.");
        let matches: Vec<_> = AnchorEnd.iter_matches_in_doc(&document).collect();

        eprintln!("🥤 {:#?} {:?}", matches, matches.to_strings(&document));
    }
}
