use crate::{
    Token,
    char_string::CharStringExt,
    expr::{Expr, FixedPhrase, SequenceExpr},
    linting::{ExprLinter, Lint, LintKind, Suggestion},
    linting::expr_linter::find_the_only_token_matching,
    patterns::WordSet,
};

/// Corrects simple past tense `went` to past participle `gone` after the auxiliary verb `have`.
pub struct HaveGone {
    expr: Box<dyn Expr>,
}

impl Default for HaveGone {
    fn default() -> Self {
        Self {
            expr: Box::new(
                SequenceExpr::default()
                    .then(WordSet::new(&["have", "had", "has", "having"]))
                    .t_ws()
                    .then(FixedPhrase::from_phrase("went")),
            ),
        }
    }
}

impl ExprLinter for HaveGone {
    fn expr(&self) -> &dyn Expr {
        self.expr.as_ref()
    }

    fn match_to_lint(&self, toks: &[Token], src: &[char]) -> Option<Lint> {
        let went_tok = find_the_only_token_matching(toks, src, |tok, src| {
            tok.span.get_content(src).eq_ignore_ascii_case_chars(&['w', 'e', 'n', 't'])
        })?;

        let suggestions = vec![Suggestion::replace_with_match_case(
            "gone".chars().collect(),
            went_tok.span.get_content(src),
        )];

        let message =
            "Use the past participle `gone` instead of `went` in the \"perfect\" compound tenses."
                .to_owned();

        Some(Lint {
            span: went_tok.span,
            lint_kind: LintKind::Grammar,
            suggestions,
            message,
            ..Default::default()
        })
    }

    fn description(&self) -> &str {
        "Corrects `went` to `gone` in British and Australian English."
    }
}

#[cfg(test)]
mod tests {
    use super::HaveGone;
    use crate::linting::tests::assert_suggestion_result;

    #[test]
    fn correct_have_went() {
        assert_suggestion_result(
            "I have went into the btle.py file and added a print statement in _connect()",
            HaveGone::default(),
            "I have gone into the btle.py file and added a print statement in _connect()",
        );
    }

    #[test]
    fn correct_had_went() {
        assert_suggestion_result(
            "Not sure if TroLoos had went from Tasmota->minimal->Tasmota, or directly Minimal->Tasmota, but going ESPHome->Minimal->Tasmota is not possible",
            HaveGone::default(),
            "Not sure if TroLoos had gone from Tasmota->minimal->Tasmota, or directly Minimal->Tasmota, but going ESPHome->Minimal->Tasmota is not possible",
        );
    }

    #[test]
    fn correct_having_went() {
        assert_suggestion_result(
            "Having went through the setup guidelines and picking react starter, running npm run watch results in an error",
            HaveGone::default(),
            "Having gone through the setup guidelines and picking react starter, running npm run watch results in an error",
        );
    }

    #[test]
    fn correct_has_went() {
        assert_suggestion_result(
            "I would like to report that the package request which you are loading has went into maintenance mode.",
            HaveGone::default(),
            "I would like to report that the package request which you are loading has gone into maintenance mode.",
        );
    }
}
