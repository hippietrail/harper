mod prefer_sneaked;
mod prefer_snuck;

use super::merge_linters::merge_linters;

use prefer_sneaked::PreferSneaked;
use prefer_snuck::PreferSnuck;

merge_linters! {
    SneakedSnuck =>
        PreferSneaked,
        PreferSnuck
        => "Enforces `sneaked` v `snuck` preferences."
}

#[cfg(test)]
mod tests {
    use crate::linting::SneakedSnuck;
    use crate::linting::tests::assert_suggestion_result;

    #[test]
    fn correct_snuck_to_sneaked() {
        assert_suggestion_result(
            "He snuck in around the back.",
            SneakedSnuck::default(),
            "He sneaked in around the back.",
        );
    }

    #[test]
    fn correct_sneaked_to_snuck() {
        assert_suggestion_result(
            "He sneaked in around the back.",
            SneakedSnuck::default(),
            "He snuck in around the back.",
        );
    }
}
