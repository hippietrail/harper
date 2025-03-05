use std::num::NonZeroUsize;

use lru::LruCache;

use crate::{CharString, Document, Token, TokenStringExt, patterns::Pattern};

use super::{Lint, Linter};

/// A trait that searches for [`Pattern`]s in [`Document`]s.
///
/// Makes use of [`TokenStringExt::iter_chunks`] to avoid matching across sentence or clause
/// boundaries.
#[cfg(not(feature = "concurrent"))]
pub trait PatternLinter {
    /// A simple getter for the pattern to be searched for.
    fn pattern(&self) -> &dyn Pattern;
    /// If any portions of a [`Document`] match [`Self::pattern`], they are passed through [`PatternLinter::match_to_lint`] to be
    /// transformed into a [`Lint`] for editor consumption.
    ///
    /// This function may return `None` to elect _not_ to produce a lint.
    fn match_to_lint(&self, matched_tokens: &[Token], source: &[char]) -> Option<Lint>;
    /// A user-facing description of what kinds of grammatical errors this rule looks for.
    /// It is usually shown in settings menus.
    fn description(&self) -> &str;
}

/// A trait that searches for [`Pattern`]s in [`Document`]s.
///
/// Makes use of [`TokenStringExt::iter_chunks`] to avoid matching across sentence or clause
/// boundaries.
#[cfg(feature = "concurrent")]
pub trait PatternLinter: Send + Sync {
    /// A simple getter for the pattern to be searched for.
    fn pattern(&self) -> &dyn Pattern;
    /// If any portions of a [`Document`] match [`Self::pattern`], they are passed through [`PatternLinter::match_to_lint`] to be
    /// transformed into a [`Lint`] for editor consumption.
    ///
    /// This function may return `None` to elect _not_ to produce a lint.
    fn match_to_lint(&self, matched_tokens: &[Token], source: &[char]) -> Option<Lint>;
    /// A user-facing description of what kinds of grammatical errors this rule looks for.
    /// It is usually shown in settings menus.
    fn description(&self) -> &str;
}

impl<L> Linter for L
where
    L: PatternLinter,
{
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut lints = Vec::new();
        let source = document.get_source();

        for chunk in document.iter_chunks() {
            lints.extend(run_on_chunk(self, chunk, source));
        }

        lints
    }

    fn description(&self) -> &str {
        self.description()
    }
}

type ChunkCache = LruCache<CharString, Vec<Lint>>;

/// A cache that wraps around a [`PatternLinter`], caching
/// results by chunk.
pub struct PatternLinterCache<P: PatternLinter> {
    cache: ChunkCache,
    inner: P,
}

impl<P: PatternLinter> PatternLinterCache<P> {
    /// Add a cache to a given [`PatternLinter`] with a given cache size.
    /// About a 1000 rows is recommended.
    pub fn new(inner: P, cache_size: NonZeroUsize) -> Self {
        Self {
            cache: ChunkCache::new(cache_size),
            inner,
        }
    }
}

impl<P: PatternLinter> Linter for PatternLinterCache<P> {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut lints = Vec::new();
        let source = document.get_source();

        for chunk in document.iter_chunks() {
            lints.extend(run_on_chunk_cached(
                &self.inner,
                chunk,
                source,
                &mut self.cache,
            ));
        }

        lints
    }

    fn description(&self) -> &str {
        self.inner.description()
    }
}

fn run_on_chunk(linter: &impl PatternLinter, chunk: &[Token], source: &[char]) -> Vec<Lint> {
    let mut lints = Vec::new();
    let mut tok_cursor = 0;

    loop {
        if tok_cursor >= chunk.len() {
            break;
        }

        let match_len = linter.pattern().matches(&chunk[tok_cursor..], source);

        if match_len != 0 {
            let lint = linter.match_to_lint(&chunk[tok_cursor..tok_cursor + match_len], source);

            lints.extend(lint);
            tok_cursor += match_len;
        } else {
            tok_cursor += 1;
        }
    }

    lints
}

fn run_on_chunk_cached(
    linter: &impl PatternLinter,
    chunk: &[Token],
    source: &[char],
    cache: &mut ChunkCache,
) -> Vec<Lint> {
    let Some(chunk_span) = chunk.span() else {
        return Vec::new();
    };

    let key = chunk_span.get_content(source);

    if let Some(hit) = cache.get(key) {
        hit.clone()
    } else {
        let lints = run_on_chunk(linter, chunk, source);
        cache.put(key.into(), lints.clone());
        lints
    }
}
