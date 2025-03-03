// based on harper-core/src/title_case.rs
use crate::Lrc;
use crate::Token;

use crate::{CharStringExt, Dictionary, Document, parsers::Parser};

pub fn make_canonical_case_str(source: &str, parser: &impl Parser, dict: &impl Dictionary) -> String {
    let source: Vec<char> = source.chars().collect();

    make_canonical_case_chars(Lrc::new(source), parser, dict).to_string()
}

pub fn make_canonical_case_chars(
    source: Lrc<Vec<char>>,
    parser: &impl Parser,
    dict: &impl Dictionary,
) -> Vec<char> {
    let document = Document::new_from_vec(source.clone(), parser, dict);

    make_canonical_case(document.get_tokens(), source.as_slice(), dict)
}

pub fn make_canonical_case(toks: &[Token], source: &[char], dict: &impl Dictionary) -> Vec<char> {
    if toks.is_empty() {
        return Vec::new();
    }

    // just return an empty vec for now
    Vec::new()
}