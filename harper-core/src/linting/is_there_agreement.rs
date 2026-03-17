use crate::{
    CharStringExt, Lint, TokenStringExt,
    expr::{Expr, FirstMatchOf, FixedPhrase, OwnedExprExt, SequenceExpr, SpelledNumberExpr},
    linting::{
        ExprLinter, LintKind, Suggestion,
        debug::format_lint_match,
        expr_linter::{Sentence, find_the_only_token_idx_matching, followed_by_word},
    },
};

pub struct IsThereAgreement {
    expr: Box<dyn Expr>,
}

impl Default for IsThereAgreement {
    fn default() -> Self {
        let is_was = SequenceExpr::any_of(vec![
            Box::new(FixedPhrase::from_phrase("there is")),
            Box::new(FixedPhrase::from_phrase("there was")),
            Box::new(FixedPhrase::from_phrase("is there")),
            Box::new(FixedPhrase::from_phrase("was there")),
        ]);
        let are_were = SequenceExpr::any_of(vec![
            Box::new(FixedPhrase::from_phrase("there are")),
            Box::new(FixedPhrase::from_phrase("there were")),
            Box::new(FixedPhrase::from_phrase("are there")),
            Box::new(FixedPhrase::from_phrase("were there")),
        ]);

        Self {
            expr: Box::new(FirstMatchOf::new(vec![
                Box::new(is_was.t_ws().then_plural_noun()),
                Box::new(
                    are_were
                        .t_ws()
                        .then(SequenceExpr::default().then_singular_noun().and_not(
                            FirstMatchOf::new(vec![
                                Box::new(|t: &crate::Token, s: &[char]| {
                                    t.kind.is_adjective()
                                        || t.span
                                            .get_content(s)
                                            .eq_ignore_ascii_case_chars(&['n', 'o'])
                                }),
                                Box::new(SpelledNumberExpr),
                            ]),
                        )),
                ),
            ])),
        }
    }
}

impl ExprLinter for IsThereAgreement {
    type Unit = Sentence;

    fn expr(&self) -> &dyn Expr {
        self.expr.as_ref()
    }

    fn description(&self) -> &str {
        "Checks for `is there` and its variants agreeing with singular vs plural subjects"
    }

    fn match_to_lint_with_context(
        &self,
        toks: &[crate::Token],
        src: &[char],
        ctx: Option<(&[crate::Token], &[crate::Token])>,
    ) -> Option<super::Lint> {
        eprintln!("🤢 {}", format_lint_match(toks, ctx, src));

        let theres_idx = find_the_only_token_idx_matching(&toks[0..=2], src, |t, s| {
            t.span
                .get_content(s)
                .eq_any_ignore_ascii_case_str(&["there's", "there's"])
        })?;
        let be_tok_idx = 2 - theres_idx;
        let _theres_tok = &toks[theres_idx];
        let be_tok = &toks[be_tok_idx];
        let be_chars = be_tok.span.get_content(src);

        let is_plural_be = be_tok
            .span
            .get_content(src)
            .eq_any_ignore_ascii_case_str(&["are", "were"]);

        // Statements are "there is ..." and questions are "is there ..."
        let _is_question = theres_idx == 2;

        // If the verb is plural that means we got flagged because the following noun is singular
        // But that could be the first part of a compound plural noun like "config errors"
        if is_plural_be && followed_by_word(ctx, |t| t.kind.is_plural_noun()) {
            // This is a compound plural noun, so we don't need to flag it
            return None;
        }

        eprintln!("🤢🤢");

        let msg = format!(
            "There should be a {} noun with '{}'",
            if is_plural_be { "plural" } else { "singular" },
            toks.span()?.get_content_string(src).to_ascii_lowercase()
        );

        if !is_plural_be {
            // "be" is singular and noun is plural - singular noun might need a/an etc. so changing "be" to plural is easier
            // is -> are; was -> were
            let replacement = if be_chars.starts_with_ignore_ascii_case_chars(&['i']) {
                "are"
            } else {
                "were"
            };

            eprintln!("🍎 {}", replacement);
        
            return Some(Lint {
                span: be_tok.span,
                lint_kind: LintKind::Agreement,
                suggestions: vec![Suggestion::replace_with_match_case(
                    replacement
                    .chars()
                    .collect(),
                    be_chars,
                )],
                message: msg,
                ..Default::default()
            });
        };
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::linting::tests::{assert_no_lints, assert_suggestion_result};

    use super::IsThereAgreement;

    #[test]
    fn fix_there_is_plural() {
        assert_suggestion_result(
            "Hi， when I make the code, there is errors",
            IsThereAgreement::default(),
            "Hi， when I make the code, there are errors",
        );
    }

    #[test]
    fn fix_there_are_singular() {
        assert_suggestion_result(
            "there are person",
            IsThereAgreement::default(),
            "There are people",
        );
    }

    #[test]
    fn dont_flag_there_are_compound_singular() {
        assert_no_lints("there are config errors", IsThereAgreement::default());
    }

    // there was pl
    // there were sg
    // is there pl
    // are there sg
    // was there pl
    // were there sg
}
