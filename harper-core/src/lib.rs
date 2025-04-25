#![doc = include_str!("../README.md")]
#![allow(dead_code)]

mod char_ext;
mod char_string;
mod currency;
mod document;
mod edit_distance;
mod fat_token;
mod ignored_lints;
pub mod language_detection;
mod lexing;
pub mod linting;
mod mask;
mod number;
pub mod parsers;
pub mod patterns;
mod punctuation;
mod span;
pub mod spell;
mod sync;
mod title_case;
mod token;
mod token_kind;
mod token_string_ext;
mod vec_ext;
mod word_metadata;

use std::cmp::Ordering;
use std::collections::VecDeque;

pub use char_string::{CharString, CharStringExt};
pub use currency::Currency;
pub use document::Document;
pub use fat_token::{FatStringToken, FatToken};
pub use ignored_lints::IgnoredLints;
use linting::Lint;
pub use mask::{Mask, Masker};
pub use number::{Number, NumberSuffix};
pub use punctuation::{Punctuation, Quote};
pub use span::Span;
pub use spell::{Dictionary, FstDictionary, MergedDictionary, MutableDictionary, WordId};
pub use sync::{LSend, Lrc};
pub use title_case::{make_title_case, make_title_case_str};
pub use token::Token;
pub use token_kind::TokenKind;
pub use token_string_ext::TokenStringExt;
pub use vec_ext::VecExt;
pub use word_metadata::{
    AdverbData, ConjunctionData, Dialect, NounData, PronounData, Tense, VerbData, WordMetadata,
};

/// A utility function that removes overlapping lints in a vector,
/// keeping the more important ones.
///
/// Note: this function will change the ordering of the lints.
pub fn remove_overlaps(lints: &mut Vec<Lint>) {
    if lints.len() < 2 {
        return;
    }

    let mut remove_indices = VecDeque::new();
    lints.sort_by_key(|l| (l.span.start, !0 - l.span.end));

    let mut cur = 0;

    for (i, lint) in lints.iter().enumerate() {
        if lint.span.start < cur {
            remove_indices.push_back(i);
            continue;
        }
        cur = lint.span.end;
    }

    lints.remove_indices(remove_indices);
}

pub fn cmp_strsl_charsl(a_utf8: &str, b_utf32: &[char]) -> Option<Ordering> {
    cmp_charsl_strsl(b_utf32, a_utf8).map(Ordering::reverse)
}

pub fn cmp_charsl_strsl(a_utf32: &[char], b_utf8: &str) -> Option<Ordering> {
    // we want a pointer/index for each
    let (mut a_i_utf32, mut b_i_utf8) = (0, 0);
    while a_i_utf32 < a_utf32.len() && b_i_utf8 < b_utf8.len() {
        // for the UTF-8 side we need to decode the bytes. let's do it manually by examining the bits ourselves
        let a_ch = a_utf32[a_i_utf32];
        let a_u32 = a_ch as u32;
        let mut b_u32: u32 = 0; // we'll accumulate the utf-8 codepoint here
        // we know how many bytes will be in the UTF-8 by the number of leading 1s
        let mut b_num_bytes = 0;
        let b_byte = b_utf8.as_bytes()[b_i_utf8];
        b_i_utf8 += 1;
        if b_byte & 0b1000_0000 == 0b0000_0000 {
            eprintln!("1 byte");
            b_num_bytes = 1;
            b_u32 = b_byte as u32;
            return Some(a_u32.cmp(&b_u32));
        } else if b_byte & 0b1110_0000 == 0b1100_0000 {
            eprintln!("2 bytes");
            b_num_bytes = 2;
            let b2 = b_utf8.as_bytes()[b_i_utf8];
            b_i_utf8 += 1;
            let xxxxx = b_byte & 0b0001_1111;
            let yyyyyy = b2 & 0b0011_1111;
            b_u32 = ((xxxxx as u32) << 6) | (yyyyyy as u32);
            return Some(a_u32.cmp(&b_u32));
        } else if b_byte & 0b1111_0000 == 0b1110_0000 {
            eprintln!("3 bytes");
            b_num_bytes = 3;
            // todo!();
        } else if b_byte & 0b1111_1000 == 0b1111_0000 {
            eprintln!("4 bytes");
            b_num_bytes = 4;
            todo!();
        } else {
            return None;
        }

        // if num_bytes is 1 and c32 is in ascii range < 128 we can compare this codepoint directly
        // if b_num_bytes == 1 && a_u32 <= 127 {
        return Some(a_u32.cmp(&b_u32));
        // }
        // todo!();
    }
    None
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::{
        Dialect, Document, FstDictionary, cmp_charsl_strsl, cmp_strsl_charsl,
        linting::{LintGroup, Linter},
        remove_overlaps,
    };

    // #[test]
    // fn keeps_space_lint() {
    //     let doc = Document::new_plain_english_curated("Ths  tet");

    //     let mut linter = LintGroup::new_curated(FstDictionary::curated(), Dialect::American);

    //     let mut lints = linter.lint(&doc);

    //     dbg!(&lints);
    //     remove_overlaps(&mut lints);
    //     dbg!(&lints);

    //     assert_eq!(lints.len(), 3);
    // }

    #[test]
    fn cmp_1ascii_chars_str_same() {
        assert_eq!(cmp_charsl_strsl(&['a'][..], "a"), Some(Ordering::Equal));
    }

    #[test]
    fn cmp_1ascii_str_chars_same() {
        assert_eq!(cmp_strsl_charsl("a", &['a'][..]), Some(Ordering::Equal));
    }

    #[test]
    fn cmp_1ascii_chars_str_less() {
        assert_eq!(cmp_charsl_strsl(&['a'][..], "b"), Some(Ordering::Less));
    }

    #[test]
    fn cmp_1ascii_str_chars_less() {
        assert_eq!(cmp_strsl_charsl("a", &['b'][..]), Some(Ordering::Less));
    }

    #[test]
    fn cmp_1ascii_chars_str_greater() {
        assert_eq!(cmp_charsl_strsl(&['b'][..], "a"), Some(Ordering::Greater));
    }

    #[test]
    fn cmp_1ascii_str_chars_greater() {
        assert_eq!(cmp_strsl_charsl("b", &['a'][..]), Some(Ordering::Greater));
    }

    #[test]
    fn cmp_latin1_chars_str_same() {
        assert_eq!(cmp_charsl_strsl(&['é'][..], "é"), Some(Ordering::Equal));
    }

    #[test]
    fn cmp_latin1_str_chars_same() {
        assert_eq!(cmp_strsl_charsl("ñ", &['ñ'][..]), Some(Ordering::Equal));
    }
}
