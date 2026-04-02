use crate::{LSend, Token};

use super::SingleTokenPattern;

/// A generic single token pattern that accepts a closure for custom matching logic.
pub struct CustomToken<F>
where
    F: LSend + Fn(&Token, &[char]) -> bool,
{
    matcher: F,
}

impl<F> CustomToken<F>
where
    F: LSend + Fn(&Token, &[char]) -> bool,
{
    /// Creates a new token pattern with the provided matching closure.
    pub fn new(matcher: F) -> Self {
        Self { matcher }
    }
}

impl<F> SingleTokenPattern for CustomToken<F>
where
    F: LSend + Fn(&Token, &[char]) -> bool,
{
    fn matches_token(&self, token: &Token, source: &[char]) -> bool {
        (self.matcher)(token, source)
    }
}
