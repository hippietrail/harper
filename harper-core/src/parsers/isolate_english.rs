use crate::{Dictionary, language_detection::is_likely_english};

use super::{Parser, Token, TokenStringExt};

/// A parser that wraps another, using heuristics to quickly redact paragraphs of a document that aren't
/// intended to be English text.
pub struct IsolateEnglish<D: Dictionary> {
    inner: Box<dyn Parser>,
    dict: D,
}

impl<D: Dictionary> IsolateEnglish<D> {
    pub fn new(inner: Box<dyn Parser>, dictionary: D) -> Self {
        Self {
            inner,
            dict: dictionary,
        }
    }
}

impl<D: Dictionary> Parser for IsolateEnglish<D> {
    fn parse(&self, source: &[char]) -> Vec<Token> {
        let tokens = self.inner.parse(source);

        let mut english_tokens: Vec<Token> = Vec::with_capacity(tokens.len());

        for chunk in tokens.iter_chunks() {
            if chunk.len() < 5 || is_likely_english(chunk, source, &self.dict) {
                english_tokens.extend_from_slice(chunk);
            }
        }

        english_tokens
    }
}

#[cfg(test)]
mod tests {
    use crate::{Document, FstDictionary, TokenStringExt, parsers::PlainEnglish};

    use super::IsolateEnglish;

    /// Assert that the provided text contains _no_ chunks of valid English
    fn assert_no_english(text: &str) {
        let dict = FstDictionary::curated();

        let document = Document::new(
            text,
            &IsolateEnglish::new(Box::new(PlainEnglish), dict.clone()),
            &dict,
        );

        assert_eq!(document.iter_words().count(), 0);
        assert_eq!(document.iter_punctuations().count(), 0);
    }

    /// Assert that, once stripped of non-English chunks, the resulting document looks like another
    /// piece of text.
    fn assert_stripped_english(source: &str, target: &str) {
        let dict = FstDictionary::curated();

        let document = Document::new(
            source,
            &IsolateEnglish::new(Box::new(PlainEnglish), dict.clone()),
            &dict,
        );

        assert_eq!(document.to_string(), target);
    }

    #[test]
    fn mixed_spanish_english_breakfast() {
        assert_no_english(
            "En la mañana, como a dish de los huevos, un poquito of tocino, y a lot of leche.",
        );
    }

    #[test]
    fn mixed_spanish_english_politics() {
        assert_no_english(
            "No estoy of acuerdo con the politics de Los estados unidos ahora; pienso que we need mas diversidad in el gobierno.",
        );
    }

    #[test]
    fn english_no_edit_motto() {
        assert_stripped_english(
            "I have a simple motto in life: ",
            "I have a simple motto in life: ",
        );
    }

    #[test]
    fn chunked_trad_chinese_english() {
        assert_stripped_english(
            "I have a simple motto in life: 如果你渴了，就喝水。",
            "I have a simple motto in life:",
        );
    }

    #[test]
    fn chunked_trad_polish_english() {
        assert_stripped_english(
            "I have a simple motto in life: jeśli jesteś spragniony, napij się wody.",
            "I have a simple motto in life:",
        );
    }
}
