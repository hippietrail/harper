use super::{LintGroup, MapPhraseLinter};

/// Produce a [`LintGroup`] that looks for errors in common phrases.
/// Comes pre-configured with the recommended default settings.
pub fn lint_group() -> LintGroup {
    let mut group = LintGroup::default();

    macro_rules! add_exact_mappings {
        ($group:expr, {
            $($name:expr => ($input:expr, $corrections:expr, $hint:expr, $description:expr)),+ $(,)?
        }) => {
            $(
                $group.add_pattern_linter(
                    $name,
                    Box::new(
                        MapPhraseLinter::new_exact_phrases(
                            $input,
                            $corrections,
                            $hint,
                            $description
                        ),
                    ),
                );
            )+
        };
    }

    add_exact_mappings!(group, {
        // Avoid suggestions resulting in "a entire ...."
        "AWholeEntire" => (
            ["a whole entire"],
            ["a whole", "an entire"],
            "Avoid redundancy. Use either `whole` or `entire` for referring to the complete amount or extent.",
            "Corrects the redundancy in `whole entire` to `whole` or `entire`."
        ),
        "WholeEntire" => (
            ["whole entire"],
            ["whole", "entire"],
            "Avoid redundancy. Use either `whole` or `entire` for referring to the complete amount or extent.",
            "Corrects the redundancy in `whole entire` to `whole` or `entire`."
        ),
        "EachAndEveryOne" => (
            ["each and everyone"],
            ["each and every one"],
            "Use `every one` for individual members of a group.",
            "Corrects `each and everyone` to `each and every one`."
        ),
        "EveryOneOf" => (
            ["everyone of"],
            ["every one of"],
            "Use `every one` for individual members of a group.",
            "Corrects `everyone of` to `every one of`."
        ),
    });

    group.set_all_rules_to(Some(true));

    group
}

#[cfg(test)]
mod tests {
    use crate::linting::tests::{
        assert_lint_count, assert_second_suggestion_result, assert_suggestion_result,
    };

    use super::lint_group;

    #[test]
    fn detect_atomic_whole_entire() {
        assert_suggestion_result("whole entire", lint_group(), "whole");
    }

    #[test]
    fn correct_atomic_a_whole_entire_to_a_whole() {
        assert_suggestion_result("a whole entire", lint_group(), "a whole");
    }

    #[test]
    fn correct_a_whole_entire_surrounded_by_text() {
        assert_suggestion_result("A B C D a whole entire W X Y Z", lint_group(), "A B C D a whole W X Y Z");
    }

    #[test]
    fn correct_a_whole_entire_nums() {
        assert_suggestion_result("0123456789 a whole entire 9876543210", lint_group(), "0123456789 a whole 9876543210");
    }

    #[test]
    fn correct_a_whole_entire_other() {
        assert_suggestion_result("a whole entire other", lint_group(), "a whole other");
    }

    #[test]
    fn correct_atomic_a_whole_entire_to_an_entire() {
        assert_second_suggestion_result("a whole entire", lint_group(), "an entire");
    }

    #[test]
    fn correct_real_world_whole_entire() {
        assert_suggestion_result(
            "[FR] support use system dns in whole entire app",
            lint_group(),
            "[FR] support use system dns in whole app",
        );
    }

    // TODO: something goes wrong when both WholeEntire and AWholeEntire are enabled
    // TODO: result is `Start mapping a wholeanet using NASA’s MOLA.`
    #[test]
    fn correct_real_world_a_whole_entire() {
        assert_suggestion_result(
            "Start mapping a whole entire new planet using NASA’s MOLA.",
            lint_group(),
            "Start mapping a whole new planet using NASA’s MOLA.",
        );
    }

    #[test]
    fn detect_each_and_everyone() {
        assert_suggestion_result("each and everyone", lint_group(), "each and every one");
    }

    #[test]
    fn detect_each_and_everyone_real_world() {
        assert_suggestion_result(
            "I have modified each and everyone of them to keep only the best of the best!",
            lint_group(),
            "I have modified each and every one of them to keep only the best of the best!",
        );
    }

    #[test]
    fn detect_everyone_of() {
        assert_suggestion_result("everyone of", lint_group(), "every one of");
    }

    #[test]
    fn detect_everyone_of_real_world() {
        assert_suggestion_result(
            "Just chiming in to say I also get this on everyone of my builds and was about to file an issue ...",
            lint_group(),
            "Just chiming in to say I also get this on every one of my builds and was about to file an issue ...",
        );
    }
}
