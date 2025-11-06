use crate::expr::{Expr, SequenceExpr, SpaceOrHyphen};
use crate::linting::{ExprLinter, Lint, LintKind, Suggestion};
use crate::patterns::InflectionOfBe;
use crate::token_string_ext::TokenStringExt;
use crate::{CharStringExt, Token};

pub struct BeAbstractNoun {
    expr: Box<dyn Expr>,
}

impl Default for BeAbstractNoun {
    fn default() -> Self {
        Self {
            expr: Box::new(
                SequenceExpr::default()
                    .then(InflectionOfBe::new())
                    .t_ws()
                    .then_optional(
                        SequenceExpr::word_set(&[
                            "extremely",
                            "damn",
                            "fairly",
                            "pretty",
                            "quite",
                            "rather",
                            "really",
                            "so",
                            "too",
                            "totally",
                            "utterly",
                            "very",
                        ])
                        .t_ws(),
                    )
                    .then_word_set(&[
                        // Common even among native speakers
                        "bias",
                        "prejudice",
                        // Common mainly in Singapore and Malaysia
                        "shock",
                        "stress",
                    ])
                    .then_optional(SpaceOrHyphen)
                    .then_optional(SequenceExpr::any_word()),
            ),
        }
    }
}

impl ExprLinter for BeAbstractNoun {
    fn expr(&self) -> &dyn Expr {
        self.expr.as_ref()
    }

    fn match_to_lint(&self, toks: &[Token], src: &[char]) -> Option<Lint> {
        // If there's an intensifier it will be token #2 and the abstract noun will be token #4
        // If there's no intensifier the abstract noun will be token #2
        // An intensifier should be an adverb

        // let has_intensifier = toks.len() >= 5;
        let has_intensifier = toks.get(2)?.kind.is_adverb();
        let abstract_noun_index = has_intensifier as usize * 2 + 2;

        // IF the 2nd last token is a hyphen, the abstract noun is part of a hyphenated compound
        if toks.get(toks.len().saturating_sub(2))?.kind.is_hyphen() {
            return None;
        }

        // Otherwise check for known false positive compounds using a space
        if toks[toks.len().saturating_sub(3)..]
            .span()?
            .get_content(src)
            .eq_any_ignore_ascii_case_str(&[
                "bias corrected",
                "shock sensitive",
                "stress free",
                "stress tested",
                "stress testing",
            ])
        {
            return None;
        }

        let abstract_noun_tok = toks.get(abstract_noun_index)?;
        let abstract_noun = abstract_noun_tok.span.get_content_string(src);

        let adjective: Vec<char> = match abstract_noun.as_str() {
            "bias" => "biased",
            "prejudice" => "prejudiced",
            "shock" => "shocked",
            "stress" => "stressed",
            _ => return None,
        }
        .chars()
        .collect();

        Some(Lint {
            span: abstract_noun_tok.span,
            lint_kind: LintKind::Usage,
            suggestions: vec![Suggestion::replace_with_match_case(
                adjective,
                abstract_noun_tok.span.get_content(src),
            )],
            message: "You can't \"be\" an abstract noun. Use the adjective form.".to_string(),
            ..Default::default()
        })
    }

    fn description(&self) -> &str {
        "Checks for certain abstract nouns being used where they should be adjectives."
    }
}

#[cfg(test)]
mod tests {
    use crate::linting::BeAbstractNoun;
    use crate::linting::tests::{assert_no_lints, assert_suggestion_result};

    #[test]
    fn test_i_am_shock() {
        assert_suggestion_result("I am shock", BeAbstractNoun::default(), "I am shocked");
    }

    #[test]
    fn test_multiple() {
        assert_suggestion_result(
            "I was very stress when I found out he was so prejudice",
            BeAbstractNoun::default(),
            "I was very stressed when I found out he was so prejudiced",
        );
    }

    #[test]
    fn i_was_quite_shock() {
        assert_suggestion_result(
            "It turns out just 54 rules are not redundant. I was quite shock, and then I used another example to test which is here:",
            BeAbstractNoun::default(),
            "It turns out just 54 rules are not redundant. I was quite shocked, and then I used another example to test which is here:",
        );
    }

    #[test]
    fn he_was_very_bias() {
        assert_suggestion_result(
            "One candidate in particular he was very bias towards because he worked with him in the past",
            BeAbstractNoun::default(),
            "One candidate in particular he was very biased towards because he worked with him in the past",
        );
    }

    #[test]
    fn i_am_so_stress() {
        assert_suggestion_result(
            "I have this problem for 3 days. yeah i know its so long and i am so stress about it.",
            BeAbstractNoun::default(),
            "I have this problem for 3 days. yeah i know its so long and i am so stressed about it.",
        );
    }

    #[test]
    fn i_was_extremely_shock() {
        assert_suggestion_result(
            "I was extremely shock, since no one ever in my life insulted me that way",
            BeAbstractNoun::default(),
            "I was extremely shocked, since no one ever in my life insulted me that way",
        );
    }

    #[test]
    fn i_am_too_stress() {
        assert_suggestion_result(
            "I am too stress because I internship at a company",
            BeAbstractNoun::default(),
            "I am too stressed because I internship at a company",
        );
    }

    #[test]
    fn i_am_really_stress() {
        assert_suggestion_result(
            "I have work with a client for nearly a week and I am really stress to cooperate with him.",
            BeAbstractNoun::default(),
            "I have work with a client for nearly a week and I am really stressed to cooperate with him.",
        );
    }

    #[test]
    fn i_am_totally_shock() {
        assert_suggestion_result(
            "I am new here i am totally shock no one is answering me just voting down why?",
            BeAbstractNoun::default(),
            "I am new here i am totally shocked no one is answering me just voting down why?",
        );
    }

    #[test]
    fn dont_flag_hyphenated() {
        assert_no_lints(
            "It is very shock-sensitive and becomes even more unstable over time.",
            BeAbstractNoun::default(),
        );
    }

    #[test]
    fn dont_flag_stress_tested() {
        assert_no_lints(
            "unless they are really stress tested",
            BeAbstractNoun::default(),
        );
    }

    #[test]
    fn dont_flag_shock_sensitive() {
        assert_no_lints(
            "Unfortunately we won't be able to load our dynamite in the cannon, it is too shock sensitive.",
            BeAbstractNoun::default(),
        );
    }

    #[test]
    fn dont_flag_stress_free() {
        assert_no_lints(
            "I am really stress free now with respect to tenant management and maintenance issues.",
            BeAbstractNoun::default(),
        );
    }

    #[test]
    fn dont_flag_bias_corrected() {
        assert_no_lints(
            "From the code (#188) I gather they are bias-corrected estimates.",
            BeAbstractNoun::default(),
        );
    }

    #[test]
    #[ignore = "false positive - maybe we should check for 'it' two tokens before the copula"]
    fn dont_flag_some_of_it_is_bias() {
        assert_no_lints(
            "Some of it is bias you're fine with",
            BeAbstractNoun::default(),
        );
    }

    #[test]
    #[ignore = "false positive we can't handle yet"]
    fn dont_flag_all_it_brings_you_is_stress() {
        assert_no_lints(
            "If all a relationship brings you is stress, it's time to break ties and move on.",
            BeAbstractNoun::default(),
        );
    }

    #[test]
    #[ignore = "false positive we can't handle yet"]
    fn dont_flag_all_around_you_is_prejudice() {
        assert_no_lints(
            "How do you make sense of diversity when all around you is prejudice?",
            BeAbstractNoun::default(),
        );
    }

    #[test]
    #[ignore = "false positive we can't handle yet"]
    fn dont_flag_bias_created_by_lack_of() {
        assert_no_lints(
            "because it is bias created by lack of women in the design",
            BeAbstractNoun::default(),
        );
    }
}
