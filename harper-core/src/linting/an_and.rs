use crate::{
    CharStringExt, Lint, Token,
    expr::{Expr, FirstMatchOf, SequenceExpr},
    linting::{
        ExprLinter, LintKind, Suggestion,
        debug::format_lint_match,
        expr_linter::{Chunk, find_the_only_token_matching, followed_by_word},
    },
};

// also any conjunction??
const NOT_USUALLY_BEFORE_AND: &[&str] = &[
    "and", "as", "by", "for", "is", "of", "on", "to", "was", "with",
];

// also any Pronoun, Determiner, Preposition, Conjunction, Adverb??
const NOT_USUALLY_AFTER_AN: &[&str] = &[
    "a",     // det.indef
    "his",   // det.poss
    "its",   // det.poss
    "that",  // det/pron.rel
    "the",   // det.def
    "their", // det.poss,
    "he",    // pron.subj
    "I",     // pron subj.
    "it",    // pron.subj+obj
    "you",   // pron.subj+obj
    "in",    // prep
    "to",    // prep
    "if",    // special
    "so",    // special
    "then",  // special
    "when",  // special
];

pub struct AnAnd {
    expr: FirstMatchOf,
}

impl Default for AnAnd {
    fn default() -> Self {
        Self {
            expr: FirstMatchOf::new([
                Box::new(SequenceExpr::aco("an").t_ws().t_set(NOT_USUALLY_AFTER_AN)),
                Box::new(
                    SequenceExpr::word_set(NOT_USUALLY_BEFORE_AND)
                        .t_ws()
                        .t_aco("and"),
                ),
            ]),
        }
    }
}

impl ExprLinter for AnAnd {
    type Unit = Chunk;

    fn match_to_lint_with_context(
        &self,
        toks: &[Token],
        src: &[char],
        ctx: Option<(&[Token], &[Token])>,
    ) -> Option<Lint> {
        eprintln!("🚨 {}", format_lint_match(toks, ctx, src));
        let span = find_the_only_token_matching(toks, src, |t: &Token, s: &[char]| {
            t.get_ch(s)
                .eq_any_ignore_ascii_case_chars(&[&['a', 'n'], &['a', 'n', 'd']])
        })?
        .span;

        let ch = span.get_content(src);

        if ch.eq_ch(&['a', 'n', 'd']) {
            if toks[0].get_ch(src).eq_ch(&['o', 'n'])
                && followed_by_word(ctx, |next| next.get_ch(src).eq_ch(&['o', 'f', 'f']))
            {
                return None;
            }
        }

        let mut the_word: Vec<char> = ch.to_vec();

        if the_word.len() == 3 {
            the_word.pop();
        } else {
            // make an 'n' the same case as the 'n' (not the 'a' as the first letter of a word may be capitalized)
            let d_to_n_delta = 'n' as u8 - 'd' as u8;
            let n = the_word[1] as u8;
            let d: char = (n - d_to_n_delta) as char;
            the_word.push(d);
        }

        let suggestions = vec![Suggestion::ReplaceWith(the_word)];

        Some(Lint {
            span,
            lint_kind: LintKind::Typo,
            suggestions,
            message: "Is this an `an` vs. `and` typo?".to_owned(),
            ..Default::default()
        })
    }

    fn expr(&self) -> &dyn Expr {
        &self.expr
    }

    fn description(&self) -> &str {
        "Detects typos mixing up `an` with `and`."
    }
}

#[cfg(test)]
mod tests {
    use crate::linting::tests::{assert_no_lints, assert_suggestion_result};

    use super::AnAnd;

    // Contrived examples

    #[test]
    fn fix_he_an_i() {
        assert_suggestion_result(
            "he an I are friends",
            AnAnd::default(),
            "he and I are friends",
        );
    }

    #[test]
    fn fix_and_ox() {
        assert_suggestion_result(
            "as hungry as and ox",
            AnAnd::default(),
            "as hungry as an ox",
        );
    }

    // Real-world examples

    #[test]
    fn dont_flag_on_and_off() {
        assert_no_lints(
            "Features that can be turned on and off in the conversation or in \"settings\"",
            AnAnd::default(),
        );
    }
}
