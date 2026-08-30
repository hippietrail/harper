use crate::{
    CharStringExt, Lint, Token,
    char_ext::CharExt,
    expr::{Expr, FirstMatchOf, SequenceExpr},
    linting::{
        ExprLinter, LintKind, Suggestion,
        debug::format_lint_match,
        expr_linter::{Chunk, find_the_only_token_matching},
    },
};

// Note: MUST be lowercased! also any conjunction??
const NOT_USUALLY_BEFORE_AND: &[&str] = &[
    "and", "as", "by", /* "for", */ "is", "of", "on", "to", "was", "with",
];

// Note: MUST be lowercased! also any Pronoun, Determiner, Preposition, Conjunction, Adverb??
const NOT_USUALLY_AFTER_AN: &[&str] = &[
    "a",     // det.indef
    "his",   // det.poss
    "its",   // det.poss
    "that",  // det/pron.rel
    "the",   // det.def
    "their", // det.poss,
    "he",    // pron.subj
    "i",     // pron subj.
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

        // if ch is 'an' we already have the next word in toks, so get the previous word from ctx
        // if ch is 'and' we already have the prev word in toks, so get the next word from ctx
        match ch.len() {
            2 => {
                // got 'an' - change to 'and'?

                let maybe_prev = if let Some((before, _)) = ctx
                    && let [ws, word, ..] = before
                    && ws.kind.is_whitespace()
                {
                    Some(word)
                } else {
                    None
                };

                let next_tok = &toks[2];
                let next_ch = next_tok.get_ch(src);

                // don't change it to 'and' if the previous word is one that doesn't usually come before 'and'
                if maybe_prev.is_some_and(|t| {
                    t.get_ch(src)
                        .eq_any_ignore_ascii_case_str(NOT_USUALLY_BEFORE_AND)
                }) {
                    return None;
                }
            }
            3 => {
                // got 'and' - change to 'an'?

                let prev_tok = &toks[0];
                let prev_ch = prev_tok.get_ch(src);

                let maybe_next = if let Some((_, after)) = ctx
                    && let [ws, word, ..] = after
                    && ws.kind.is_whitespace()
                {
                    Some(word)
                } else {
                    None
                };

                // "an" only comes before a vowel sound, not a consonant sound
                if maybe_next.is_some_and(|w| w.get_ch(src).first().is_some_and(|c| !c.is_vowel()))
                {
                    eprintln!("Skipping 'an' because next word starts with a consonant sound");
                    return None;
                }

                // don't change it to 'an' if the next word is one that doesn't usually come after 'an'
                if maybe_next.is_some_and(|t| {
                    t.get_ch(src)
                        .eq_any_ignore_ascii_case_str(NOT_USUALLY_AFTER_AN)
                }) {
                    eprintln!("Skipping 'an' because next word is not usually after 'an'");
                    return None;
                }

                // don't change it to 'an' if the previous word is 'on' and the next word is 'off'
                if prev_ch.eq_ch(&['o', 'n'])
                    && maybe_next.is_some_and(|w| w.get_ch(src).eq_ch(&['o', 'f', 'f']))
                {
                    eprintln!("Skipping 'an' because previous word is 'on' and next word is 'off'");
                    return None;
                }
            }
            _ => return None,
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
