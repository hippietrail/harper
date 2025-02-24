use super::merge_linters::merge_linters;

mod general;
mod simple;
use general::General;
use simple::Simple;

merge_linters!(OxfordComma => General, Simple => "The Oxford comma is one of the more controversial rules in common use today. Enabling this lint checks that there is a comma before `and`, `or`, or `nor` when listing out more than two ideas.");

#[cfg(test)]
mod tests {
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    use super::OxfordComma;

    #[test]
    fn fruits() {
        assert_lint_count(
            "An apple, a banana and a pear walk into a bar.",
            OxfordComma::default(),
            1,
        );
    }

    #[test]
    fn people() {
        assert_suggestion_result(
            "Nancy, Steve and Carl are going to the coffee shop.",
            OxfordComma::default(),
            "Nancy, Steve, and Carl are going to the coffee shop.",
        );
    }

    #[test]
    fn places() {
        assert_suggestion_result(
            "I've always wanted to visit Paris, Tokyo and Rome.",
            OxfordComma::default(),
            "I've always wanted to visit Paris, Tokyo, and Rome.",
        );
    }

    #[test]
    fn foods() {
        assert_suggestion_result(
            "My favorite foods are pizza, sushi, tacos and burgers.",
            OxfordComma::default(),
            "My favorite foods are pizza, sushi, tacos, and burgers.",
        );
    }

    #[test]
    fn allows_clean_music() {
        assert_lint_count(
            "I enjoy listening to pop music, rock, hip-hop, electronic dance, and classical music.",
            OxfordComma::default(),
            0,
        );
    }

    #[test]
    fn allows_clean_nations() {
        assert_lint_count(
            "The team consists of players from different countries: France, Germany, Italy, and Spain.",
            OxfordComma::default(),
            0,
        );
    }

    #[test]
    fn or_writing() {
        assert_suggestion_result(
            "Harper can be a lifesaver when writing technical documents, emails or other formal forms of communication.",
            OxfordComma::default(),
            "Harper can be a lifesaver when writing technical documents, emails, or other formal forms of communication.",
        );
    }

    #[test]
    fn sports() {
        assert_suggestion_result(
            "They enjoy playing soccer, basketball or tennis.",
            OxfordComma::default(),
            "They enjoy playing soccer, basketball, or tennis.",
        );
    }

    #[test]
    fn nor_vegetables() {
        assert_suggestion_result(
            "I like carrots, kale nor broccoli.",
            OxfordComma::default(),
            "I like carrots, kale, nor broccoli.",
        );
    }

    #[test]
    fn allow_non_list_transportation() {
        assert_lint_count(
            "In transportation, autonomous vehicles and smart traffic management systems promise to reduce accidents and optimize travel routes.",
            OxfordComma::default(),
            0,
        );
    }
}
