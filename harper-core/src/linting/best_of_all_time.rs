use crate::Token;
use crate::expr::{Expr, SequenceExpr};
use crate::patterns::WhitespacePattern;

use super::{ExprLinter, Lint, LintKind, Suggestion};

pub struct BestOfAllTime {
    expr: Box<dyn Expr>,
}

impl Default for BestOfAllTime {
    fn default() -> Self {
        // Best, Biggest
        let inflection_superlative = SequenceExpr::default().then_superlative_adjective();
        // Most interesting
        let most_superlative = SequenceExpr::default()
            .t_aco("most")
            .t_ws()
            .then_positive_adjective();
        // Some resources call this an 'absolute adjective', some consider it a superlative.
        let fave = SequenceExpr::word_set(&["favorite", "favourite"]);

        // We can't use the noun phrase Expr because it allows determiners before the nouns and "best the thing" wouldn't be right
        let expr = SequenceExpr::default()
            .then_any_of(vec![
                Box::new(inflection_superlative),
                Box::new(most_superlative),
                Box::new(fave),
            ])
            .then_one_or_more(SequenceExpr::default().then(WhitespacePattern).then_noun())
            .then_fixed_phrase(" of all times");

        Self {
            expr: Box::new(expr),
        }
    }
}

impl ExprLinter for BestOfAllTime {
    fn expr(&self) -> &dyn Expr {
        self.expr.as_ref()
    }

    fn match_to_lint(&self, toks: &[Token], src: &[char]) -> Option<Lint> {
        let times_span = toks.last()?.span;

        if let Some((_, time_singular)) = times_span.get_content(src).split_last() {
            return Some(Lint {
                span: times_span,
                lint_kind: LintKind::WordChoice,
                suggestions: vec![Suggestion::ReplaceWith(time_singular.to_vec())],
                message: "This expression uses singular `time`".to_string(),
                ..Default::default()
            });
        }

        None
    }

    fn description(&self) -> &'static str {
        "Checks for nonstandard `of all times` in superlatives instead of singular `time`"
    }
}

#[cfg(test)]
mod tests {
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    use super::BestOfAllTime;

    #[test]
    fn dont_flag_list_of_all_times() {
        assert_lint_count(
            "Provides a formatted list of all times that SDO was non-nominal",
            BestOfAllTime::default(),
            0,
        );
    }

    #[test]
    fn fix_after_best() {
        assert_suggestion_result(
            "And also in the best IDE of all times Visual Studio",
            BestOfAllTime::default(),
            "And also in the best IDE of all time Visual Studio",
        );
    }

    #[test]
    fn fix_after_greatest() {
        assert_suggestion_result(
            "This app shows you why Sachin Tendulkar is the greatest cricket of all times, by using interactive stories.",
            BestOfAllTime::default(),
            "This app shows you why Sachin Tendulkar is the greatest cricket of all time, by using interactive stories.",
        );
    }

    #[test]
    fn fix_after_biggest() {
        assert_suggestion_result(
            "THIS IS THE BIGGEST QUESTIONS OF ALL TIMES...",
            BestOfAllTime::default(),
            "THIS IS THE BIGGEST QUESTIONS OF ALL TIME...",
        );
    }

    #[test]
    fn fix_after_most_influential() {
        assert_suggestion_result(
            "It is an open source project that aggregates multiple lists of \"the best/most influential games of all times\"",
            BestOfAllTime::default(),
            "It is an open source project that aggregates multiple lists of \"the best/most influential games of all time\"",
        );
    }

    #[test]
    fn dont_flag_sum_of_all_times() {
        assert_lint_count(
            "The original TotalTime seems not be the sum of all times",
            BestOfAllTime::default(),
            0,
        );
    }

    #[test]
    fn dont_flag_history_stacks_of_all_times() {
        assert_lint_count(
            "Didn't this imply all history stacks of all times, which itself implied all those saved.",
            BestOfAllTime::default(),
            0,
        );
    }

    #[test]
    fn fix_after_favorite() {
        assert_suggestion_result(
            "Red Dead Redemption 2 is my nr 1 favorite game of all times",
            BestOfAllTime::default(),
            "Red Dead Redemption 2 is my nr 1 favorite game of all time",
        );
    }

    #[test]
    fn fix_after_favourite() {
        assert_suggestion_result(
            "Just made this website to show you my favourite movies of all times.",
            BestOfAllTime::default(),
            "Just made this website to show you my favourite movies of all time.",
        );
    }
}
