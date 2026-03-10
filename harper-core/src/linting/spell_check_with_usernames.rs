use super::{Linter, SpellCheck};
use crate::{Document, spell::Dictionary};

pub struct SpellCheckWithUsernames<T: Dictionary> {
    spell_check: SpellCheck<T>,
}

impl<T: Dictionary> SpellCheckWithUsernames<T> {
    pub fn new(dictionary: T, dialect: crate::Dialect) -> Self {
        Self {
            spell_check: SpellCheck::new(dictionary, dialect),
        }
    }
}

impl<T: Dictionary> Linter for SpellCheckWithUsernames<T> {
    fn lint(&mut self, document: &Document) -> Vec<super::Lint> {
        // Get all spelling lints from the underlying SpellCheck
        let all_lints = self.spell_check.lint(document);

        // Filter out lints for words that are usernames (follow @ symbols)
        all_lints
            .into_iter()
            .filter(|lint| {
                // Convert the lint's character span to find which tokens it intersects
                let token_indices = document.token_indices_intersecting(lint.span);

                // If we have tokens intersecting this lint, check the previous token
                if let Some(first_token_idx) = token_indices.first() {
                    // Get the previous token (if it exists)
                    if let Some(prev_token) = document.get_token_offset(*first_token_idx, -1) {
                        // Check if the previous token is exactly '@'
                        return !prev_token.kind.is_at();
                    }
                }

                // No previous @ token found, keep this lint
                true
            })
            .collect()
    }

    fn description(&self) -> &'static str {
        "Looks and provides corrections for misspelled words, ignoring usernames."
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        linting::tests::{assert_lint_count, assert_no_lints},
        spell::FstDictionary,
    };

    use super::SpellCheckWithUsernames;

    #[test]
    fn dont_flag_username_mention_2403() {
        assert_no_lints(
            "@asafm this looks great",
            SpellCheckWithUsernames::new(FstDictionary::curated(), crate::Dialect::American),
        );
    }

    #[test]
    fn flag_username_without_mention_2403() {
        assert_lint_count(
            "asafm this looks great",
            SpellCheckWithUsernames::new(FstDictionary::curated(), crate::Dialect::American),
            1,
        );
    }

    #[test]
    fn dont_flag_email_addresses() {
        assert_no_lints(
            "contact me at asafm@example.com",
            SpellCheckWithUsernames::new(FstDictionary::curated(), crate::Dialect::American),
        );
    }
}
