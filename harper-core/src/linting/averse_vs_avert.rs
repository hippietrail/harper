use crate::{
    CharStringExt, Lint, Token,
    expr::{AnchorStart, Expr, FirstMatchOf, SequenceExpr},
    linting::{
        ExprLinter, LintKind, Suggestion,
        debug::format_lint_match,
        expr_linter::{Chunk, find_the_only_token_matching},
    },
    patterns::{ContractionOfBe, InflectionOfBe},
};

const AVERSE: &[&str] = &["averse", "aversed", "averses", "aversing"];

pub struct AverseVsAvert {
    expr: FirstMatchOf,
}

impl Default for AverseVsAvert {
    fn default() -> Self {
        Self {
            expr: FirstMatchOf::new([
                Box::new(
                    SequenceExpr::default()
                        .then_any_of([
                            Box::new(
                                SequenceExpr::any_of([
                                    Box::new(ContractionOfBe::new()) as Box<dyn Expr>,
                                    Box::new(InflectionOfBe::new()),
                                    // Box::new(SequenceExpr::default().then_degree_adverb()),
                                    // Box::new(SequenceExpr::word_set(&["more", "very", "quite", "rather", "too", "so"])),
                                    Box::new(SequenceExpr::default().then_quantifier()),
                                ])
                                .t_ws(),
                            ) as Box<dyn Expr>,
                            Box::new(AnchorStart),
                        ])
                        .then_any_of([
                            Box::new(SequenceExpr::word_seq(&["avert", "to"])) as Box<dyn Expr>,
                            Box::new(
                                SequenceExpr::aco("risk")
                                    .t_ws_h()
                                    .t_set(&["avert", "averted"]),
                            ),
                        ]),
                ),
                Box::new(
                    SequenceExpr::word_set(AVERSE)
                        .t_ws()
                        .then_possessive_determiner()
                        .t_ws()
                        .t_aco("eyes"),
                ),
            ]),
        }
    }
}

impl ExprLinter for AverseVsAvert {
    type Unit = Chunk;

    fn match_to_lint_with_context(
        &self,
        toks: &[Token],
        src: &[char],
        ctx: Option<(&[Token], &[Token])>,
    ) -> Option<Lint> {
        eprintln!("🚨 {}", format_lint_match(toks, ctx, src));

        let tok = find_the_only_token_matching(toks, src, |t, s| {
            let ch = t.get_ch(s);
            ch.eq_any_ignore_ascii_case_str(AVERSE)
                || ch.eq_any_ignore_ascii_case_str(&["avert", "averted"])
        })?;

        let ch = tok.get_ch(src);

        let correction = match (ch.get(4)?, ch.last()?) {
            ('s', 'e') => "avert",
            ('s', 'd') => "averted",
            ('s', 's') => "averts",
            ('s', 'g') => "averting",
            ('t', 't') => "averse",
            ('t', 'd') => "aversed",
            _ => return None,
        };

        eprintln!("✨ correction: {}", correction);

        let suggestions = vec![Suggestion::replace_with_match_case_str(correction, ch)];
        let message = "Did you mean `averse` (strongly opposed to) rather than `avert` (to prevent or turn away)?".to_owned();

        Some(Lint {
            span: tok.span,
            lint_kind: LintKind::Malapropism,
            suggestions,
            message,
            ..Default::default()
        })
    }

    fn expr(&self) -> &dyn Expr {
        &self.expr
    }

    fn description(&self) -> &str {
        "Detects `avert` when `averse` is intended."
    }
}

#[cfg(test)]
mod tests {
    use crate::linting::tests::{assert_no_lints, assert_suggestion_result};

    use super::AverseVsAvert;

    // be avert to -> be averse to
    // very avert to -> very averse to
    // Avert to the spotlight.

    #[test]
    fn be_avert_to_kill() {
        assert_suggestion_result(
            "It's like how in our world most humans will be avert to kill or torture creatures of the in-group",
            AverseVsAvert::default(),
            "It's like how in our world most humans will be averse to kill or torture creatures of the in-group",
        );
    }

    #[test]
    fn avert_to_at_start() {
        assert_suggestion_result(
            "Avert to the spotlight.",
            AverseVsAvert::default(),
            "Averse to the spotlight.",
        );
    }

    // risk-avert/averted -> risk-averse

    #[test]
    fn more_risk_avert() {
        assert_suggestion_result(
            "to be cautious and like you said becoming more risk avert",
            AverseVsAvert::default(),
            "to be cautious and like you said becoming more risk averse",
        );
    }

    #[test]
    fn being_risk_avert() {
        assert_suggestion_result(
            "the Bank’s attempt to quickly respond to the country’s immediate transport challenges, and not being risk avert, is commendable",
            AverseVsAvert::default(),
            "the Bank’s attempt to quickly respond to the country’s immediate transport challenges, and not being risk averse, is commendable",
        );
    }

    #[test]
    fn are_risk_avert() {
        assert_suggestion_result(
            "households and communities are risk avert and face significant barriers to adopting livelihood strategies",
            AverseVsAvert::default(),
            "households and communities are risk averse and face significant barriers to adopting livelihood strategies",
        );
    }

    #[test]
    fn dont_flag_not_at_start_or_after_be() {
        assert_no_lints(
            "Status: passed; one critical risk averted (per design-review F-2)",
            AverseVsAvert::default(),
        );
    }

    // averse one's eyes -> avert one's eyes

    #[test]
    fn averse_his_eyes() {
        assert_suggestion_result(
            "His claim that he would never averse his eyes from slaughter has proved, sadly, quite hollow.",
            AverseVsAvert::default(),
            "His claim that he would never avert his eyes from slaughter has proved, sadly, quite hollow.",
        );
    }

    // Not handled but found in the wild

    // Malloy plants a sustained kiss on Howard's shocked lips in a scene which
    //   might have caused a 1950s audience to avert to their eyes.
}
