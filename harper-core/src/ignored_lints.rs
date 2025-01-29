use std::hash::{DefaultHasher, Hash, Hasher};

use hashbrown::HashSet;
use serde::{Deserialize, Serialize};

use crate::{linting::Lint, Document};

/// A structure that keeps track of lints that have been ignored by users.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IgnoredLints {
    context_hashes: HashSet<u64>,
}

impl IgnoredLints {
    pub fn new() -> Self {
        Self::default()
    }

    /// Move entries from another instance to this one.
    pub fn append(&mut self, other: Self) {
        self.context_hashes.extend(other.context_hashes)
    }

    fn hash_lint_context(&self, lint: &Lint, document: &Document) -> u64 {
        let problem_tokens = document.token_indices_intersecting(lint.span);
        let prequel_tokens = lint
            .span
            .with_len(2)
            .pulled_by(2)
            .map(|v| document.token_indices_intersecting(v))
            .unwrap_or_default();
        let sequel_tokens = document.token_indices_intersecting(lint.span.with_len(2).pushed_by(2));

        let mut hasher = DefaultHasher::default();

        problem_tokens
            .into_iter()
            .chain(prequel_tokens)
            .chain(sequel_tokens)
            .flat_map(|idx| document.get_token(idx))
            .for_each(|tok| tok.kind.hash(&mut hasher));

        let lint_hash = lint.spanless_hash();
        lint_hash.hash(&mut hasher);

        hasher.finish()
    }

    /// Add a lint to the list.
    pub fn ignore_lint(&mut self, lint: &Lint, document: &Document) {
        let context_hash = self.hash_lint_context(lint, document);

        self.context_hashes.insert(context_hash);
    }

    pub fn is_ignored(&self, lint: &Lint, document: &Document) -> bool {
        let hash = self.hash_lint_context(lint, document);

        self.context_hashes.contains(&hash)
    }

    /// Remove ignored Lints from a [`Vec`].
    pub fn remove_ignored(&self, lints: &mut Vec<Lint>, document: &Document) {
        lints.retain(|lint| !self.is_ignored(lint, document));
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    use super::IgnoredLints;
    use crate::{
        linting::{LintGroup, LintGroupConfig, Linter},
        Document, FstDictionary,
    };

    #[quickcheck]
    fn can_ignore_all(text: String) -> bool {
        let document = Document::new_markdown_default_curated(&text);

        let mut lints =
            LintGroup::new(LintGroupConfig::default(), FstDictionary::curated()).lint(&document);

        let mut ignored = IgnoredLints::new();

        for lint in &lints {
            ignored.ignore_lint(lint, &document);
        }

        ignored.remove_ignored(&mut lints, &document);
        lints.is_empty()
    }

    #[quickcheck]
    fn can_ignore_first(text: String) -> TestResult {
        let document = Document::new_markdown_default_curated(&text);

        let mut lints =
            LintGroup::new(LintGroupConfig::default(), FstDictionary::curated()).lint(&document);

        let Some(first) = lints.first().cloned() else {
            return TestResult::discard();
        };

        let mut ignored = IgnoredLints::new();
        ignored.ignore_lint(&first, &document);

        ignored.remove_ignored(&mut lints, &document);

        TestResult::from_bool(!lints.contains(&first))
    }
}
