mod let_us_redundancy;
mod no_contraction_with_verb;

use super::merge_linters::merge_linters;
use let_us_redundancy::LetUsRedundancy;
use no_contraction_with_verb::NoContractionWithVerb;

merge_linters!(LetsConfusion => LetUsRedundancy, NoContractionWithVerb => "TODO");

#[cfg(test)]
mod tests {
    use crate::linting::tests::assert_suggestion_result;

    use super::LetsConfusion;

    #[test]
    fn issue_426_us() {
        assert_suggestion_result("let's us do", LetsConfusion::default(), "let's do");
    }

    #[test]
    fn issue_426_me() {
        assert_suggestion_result("let's me do", LetsConfusion::default(), "let's do");
    }

    #[test]
    fn from_harper_docs() {
        assert_suggestion_result("Often the longest and the shortest words are the most helpful, so lets push them first.", LetsConfusion::default(), "Often the longest and the shortest words are the most helpful, so let's push them first.");
    }

    #[test]
    fn issue_470_missing_apostrophe() {
        assert_suggestion_result("lets play", LetsConfusion::default(), "let's play");
    }

    #[test]
    fn issue_470_missing_subject() {
        assert_suggestion_result("let play", LetsConfusion::default(), "let's play");
    }
}
