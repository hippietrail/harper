// based on harper-core/src/patterns/is_not_title_case.rs
use crate::{Dictionary, Token, TokenStringExt, make_canonical_case};

use super::Pattern;

pub struct IsNotCanonicalCase<D: Dictionary> {
    inner: Box<dyn Pattern>,
    dict: D,
}

impl<D: Dictionary> IsNotCanonicalCase<D> {
    pub fn new(inner: Box<dyn Pattern>, dict: D) -> Self {
        println!("s: IsNotCanonicalCase::new()");
        eprintln!("e: IsNotCanonicalCase::new()");

        // let pat = *inner;
        let pinto = (*pat).into();



        Self { inner, dict }
    }
}

impl<D: Dictionary> Pattern for IsNotCanonicalCase<D> {
    fn matches(&self, tokens: &[Token], source: &[char]) -> usize {
        println!("s: IsNotCanonicalCase::matches()");
        eprintln!("e: IsNotCanonicalCase::matches()");

        0
    }
}