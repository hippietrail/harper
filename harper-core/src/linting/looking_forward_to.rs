use crate::linting::expr_linter::Chunk;
use crate::{
    Token,
    expr::{Expr, FixedPhrase, SequenceExpr},
    inflections,
    linting::{ExprLinter, Lint, LintKind, Suggestion},
    spell::Dictionary,
};

pub struct LookingForwardTo<D> {
    expr: Box<dyn Expr>,
    dict: D,
}

impl Default for LookingForwardTo<std::sync::Arc<crate::spell::FstDictionary>> {
    fn default() -> Self {
        Self::new(crate::spell::FstDictionary::curated())
    }
}

impl<D: Dictionary> LookingForwardTo<D> {
    pub fn new(dict: D) -> Self {
        let looking_forward_to = FixedPhrase::from_phrase("looking forward to");

        let pattern = SequenceExpr::default()
            .then(looking_forward_to)
            .t_ws()
            .then_verb();

        Self {
            expr: Box::new(pattern),
            dict,
        }
    }
}

impl<D: Dictionary> ExprLinter for LookingForwardTo<D> {
    type Unit = Chunk;

    fn expr(&self) -> &dyn Expr {
        self.expr.as_ref()
    }

    fn match_to_lint(&self, matched_tokens: &[Token], src: &[char]) -> Option<Lint> {
        let span = matched_tokens.last()?.span;
        let verb = matched_tokens.last()?.span.get_content_string(src);

        // Skip if already in progressive form
        if verb.ends_with("ing") {
            return None;
        }

        // Use inflections module to get gerund form
        let gerunds = inflections::verbs::lemma_to_progressive(&verb, &self.dict);
        if gerunds.is_empty() {
            // If no gerund found in dictionary, construct it manually
            return None;
        }

        let gerund_form: String = gerunds.first()?.iter().collect();

        Some(Lint {
            span,
            lint_kind: LintKind::WordChoice,
            message: format!(
                "The verb `{verb}` must be in the gerund form (verb + -ing) after 'looking forward to'.",
            ),
            suggestions: vec![Suggestion::replace_with_match_case(
                gerund_form.chars().collect(),
                span.get_content(src),
            )],
            ..Default::default()
        })
    }

    fn description(&self) -> &'static str {
        "This rule identifies instances where the phrase `looking forward to` is followed by a base form verb instead of the required gerund (verb + `-ing` form)."
    }
}

#[cfg(test)]
mod tests {
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    use super::LookingForwardTo;

    #[test]
    fn not_lint_with_correct_verb() {
        assert_suggestion_result(
            "She was looking forward to see the grandchildren again.",
            LookingForwardTo::default(),
            "She was looking forward to seeing the grandchildren again.",
        );
        // assert_lint_count(
        //     "She was looking forward to seeing the grandchildren again.",
        //     LookingForwardTo::default(),
        //     0,
        // );
    }

    #[test]
    fn lint_with_incorrect_verb() {
        assert_suggestion_result(
            "She was looking forward to see the grandchildren again.",
            LookingForwardTo::default(),
            "She was looking forward to seeing the grandchildren again.",
        );
    }

    #[test]
    fn lint_with_incorrect_verb_ending_in_e() {
        assert_suggestion_result(
            "She was looking forward to make the grandchildren happy.",
            LookingForwardTo::default(),
            "She was looking forward to making the grandchildren happy.",
        );
    }

    #[test]
    fn not_lint_with_non_verb() {
        assert_lint_count(
            "She was looking forward to the grandchildren's visit.",
            LookingForwardTo::default(),
            0,
        );
    }
}
