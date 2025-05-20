use crate::Document;

use super::Lint;
use super::LintGroup;
use super::Linter;
use super::MapPhraseLinter;
use super::merge_linters::merge_linters;

pub struct ConvenienceStore;

impl Default for ConvenienceStore {
    fn default() -> Self {
        Self
    }
}

impl Linter for ConvenienceStore {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut group = LintGroup::default();
        group.add_pattern_linter(
            "ConvenienceStore",
            Box::new(MapPhraseLinter::new_exact_phrase(
                "convenient store",
                ["convenience store"],
                "For a small local store selling everyday items and open for long hours, `convenience store` is the correct term.",
                "Corrects `convenient store` to `convenience store`.",
            )),
        );
        group.add_pattern_linter(
            "ConvenienceStores",
            Box::new(MapPhraseLinter::new_exact_phrase(
                "convenient stores",
                ["convenience stores"],
                "For a small local store selling everyday items and open for long hours, `convenience store` is the correct term.",
                "Corrects `convenient store` to `convenience store`.",
            )),
        );
        group.lint(document)
    }

    fn description(&self) -> &str {
        "Corrects common misspellings of 'convenience store' and 'convenience stores'."
    }
}

merge_linters!(ConvenienceStoreLint => ConvenienceStore => "Corrects common misspellings of 'convenience store' and 'convenience stores'.");

pub fn lint_group(dictionary: crate::FstDictionary) -> LintGroup {
    let mut group = LintGroup::default();
    group.add_pattern_linter(
        "ConvenienceStore",
        Box::new(MapPhraseLinter::new_exact_phrase(
            "convenient store",
            ["convenience store"],
            "For a small local store selling everyday items and open for long hours, `convenience store` is the correct term.",
            "Corrects `convenient store` to `convenience store`.",
        )),
    );
    group.add_pattern_linter(
        "ConvenienceStores",
        Box::new(MapPhraseLinter::new_exact_phrase(
            "convenient stores",
            ["convenience stores"],
            "For a small local store selling everyday items and open for long hours, `convenience store` is the correct term.",
            "Corrects `convenient store` to `convenience store`.",
        )),
    );
    group
}

#[cfg(test)]
mod tests {
    use super::ConvenienceStoreLint;
    use crate::linting::tests::assert_suggestion_result;

    #[test]
    fn correct_convenient_store() {
        assert_suggestion_result(
            "convenient store",
            ConvenienceStoreLint::default(),
            "convenience store",
        );
    }

    #[test]
    fn correct_convenient_stores() {
        assert_suggestion_result(
            "convenient stores",
            ConvenienceStoreLint::default(),
            "convenience stores",
        );
    }
}
