use crate::{
    CharStringExt, IrregularNouns, Lint, Token, TokenStringExt,
    expr::{Expr, FirstMatchOf, FixedPhrase, OwnedExprExt, SequenceExpr, SpelledNumberExpr},
    linting::{
        ExprLinter, LintKind, Suggestion,
        debug::format_lint_match,
        expr_linter::{
            Sentence, find_the_only_token_idx_matching, followed_by_token, followed_by_word,
        },
    },
    regular_nouns,
};

#[derive(PartialEq, Debug)]
enum Mismatch {
    ThereIsWithPluralNoun,
    ThereAreWithSingularNoun,
}
use Mismatch::*;

pub struct IsThereAgreement {
    expr: FirstMatchOf,
}

impl Default for IsThereAgreement {
    fn default() -> Self {
        let sg_verb = SequenceExpr::any_of(vec![
            Box::new(FirstMatchOf::new(vec![
                // Box::new(Word::new("there's")),
                Box::new(FixedPhrase::from_phrase("there is")), // stmt sg pres
            ])),
            Box::new(FixedPhrase::from_phrase("there was")), //    stmt sg past
            Box::new(FixedPhrase::from_phrase("is there")),  //    q.   sg pres
            Box::new(FixedPhrase::from_phrase("was there")), //    q.   sg past
        ]);
        let pl_verb = SequenceExpr::any_of(vec![
            Box::new(FixedPhrase::from_phrase("there are")), //    stmt pl pres
            Box::new(FixedPhrase::from_phrase("there were")), //   stmt pl past
            Box::new(FixedPhrase::from_phrase("are there")), //    q.   pl pres
            Box::new(FixedPhrase::from_phrase("were there")), //   q.   pl past
        ]);

        Self {
            expr: FirstMatchOf::new(vec![
                Box::new(sg_verb.t_ws().then_plural_noun()),
                Box::new(pl_verb.t_ws().then(
                    SequenceExpr::default().then_singular_noun().and_not(
                        // singular nouns that are also something else
                        FirstMatchOf::new(vec![
                            Box::new(|t: &Token, s: &[char]| {
                                t.kind.is_adjective()
                                    || t.span
                                        .get_content(s)
                                        .eq_ignore_ascii_case_chars(&['n', 'o'])
                            }),
                            // "two" etc. are sg. nouns even though they can also be plural quantifiers
                            Box::new(SpelledNumberExpr),
                        ]),
                    ),
                )),
            ]),
        }
    }
}

fn is_singular_noun(token: &Token, src: &[char]) -> bool {
    token.kind.is_singular_noun()
        && !token.kind.is_verb_progressive_form()
        && !token
            .span
            .get_content(src)
            .eq_ignore_ascii_case_chars(&['i', 'n'])
}

impl ExprLinter for IsThereAgreement {
    type Unit = Sentence;

    fn expr(&self) -> &dyn Expr {
        &self.expr
    }

    fn description(&self) -> &str {
        "Checks for `is there` and its variants agreeing with singular vs plural subjects"
    }

    fn match_to_lint_with_context(
        &self,
        toks: &[Token],
        src: &[char],
        ctx: Option<(&[Token], &[Token])>,
    ) -> Option<super::Lint> {
        eprintln!("🤢 {}", format_lint_match(toks, ctx, src));

        let there_idx = find_the_only_token_idx_matching(&toks[0..=2], src, |t, s| {
            t.span.get_content(s).eq_ignore_ascii_case_str("there")
        })?;
        // TODO does not handle "there's" case
        let be_idx = 2 - there_idx;
        // TODO does not handle "there's" case
        let noun_idx = 4;

        let _there_tok = &toks[there_idx];
        let be_tok = &toks[be_idx];
        let noun_tok = &toks[noun_idx];

        let be_chars = be_tok.span.get_content(src);
        let noun_chars = noun_tok.span.get_content(src);

        let mismatch = if be_tok
            .span
            .get_content(src)
            .eq_any_ignore_ascii_case_str(&["are", "were"])
        {
            ThereAreWithSingularNoun
        } else {
            ThereIsWithPluralNoun
        };

        // Exit early when the noun is just part of a compound

        // Either a hyphenated compound or part of a path
        if followed_by_token(ctx, |t| t.kind.is_hyphen() || t.kind.is_slash()) {
            return None;
        }

        // Or an open compound
        if followed_by_word(ctx, |second_noun| {
            // let second_noun_str = second_noun.span.get_content(src).iter().collect::<String>();
            match mismatch {
                // there is drugs trade
                ThereIsWithPluralNoun => is_singular_noun(second_noun, src),
                // there are car parks
                ThereAreWithSingularNoun => second_noun.kind.is_plural_noun(),
            }
        }) {
            // eprintln!("🚫 Early return: followed by word (open compound)");
            return None;
        }

        let msg = format!(
            "There should be a {} noun with '{}'",
            if mismatch == ThereAreWithSingularNoun {
                "plural"
            } else {
                "singular"
            },
            toks.span()?.get_content_string(src).to_ascii_lowercase()
        );

        match mismatch {
            ThereAreWithSingularNoun => {
                // there are thing -> there is [a] thing
                //                 -> there are thing+s
                let mut plurals = Vec::new();

                // Check irregular plurals first
                if let Some(plural) =
                    IrregularNouns::curated().get_plural_for_singular_chars(noun_chars)
                {
                    plurals.push(plural.chars().collect::<Vec<char>>());
                }

                // Check regular plurals
                plurals.extend(regular_nouns::get_plurals(noun_chars));

                // print the plurals in one comma-separated line:
                eprintln!(
                    "🗿🗿 {}",
                    plurals
                        .iter()
                        .map(|p| p.iter().collect::<String>())
                        .collect::<Vec<String>>()
                        .join(", ")
                );

                if plurals.is_empty() {
                    return None;
                }

                // Create suggestions for all valid plural forms
                let suggestions = plurals
                    .into_iter()
                    .map(|plural_chars| {
                        Suggestion::replace_with_match_case(plural_chars, noun_chars)
                    })
                    .collect();

                Some(Lint {
                    span: noun_tok.span,
                    lint_kind: LintKind::Agreement,
                    suggestions,
                    message: msg,
                    ..Default::default()
                })
            }
            ThereIsWithPluralNoun => {
                // there is things -> there are things
                //                 -> there is [a] thingXs
                let replacement = if be_chars.len() == 2 { "are" } else { "were" };

                Some(Lint {
                    span: be_tok.span,
                    lint_kind: LintKind::Agreement,
                    suggestions: vec![Suggestion::replace_with_match_case(
                        replacement.chars().collect(),
                        be_chars,
                    )],
                    message: msg,
                    ..Default::default()
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::linting::tests::{
        assert_good_and_bad_suggestions, assert_no_lints, assert_suggestion_result,
    };

    use super::IsThereAgreement;

    // Contrived tests

    #[test]
    fn fix_there_is_plural() {
        assert_good_and_bad_suggestions(
            "Hi， when I make the code, there is errors",
            IsThereAgreement::default(),
            &[
                "Hi， when I make the code, there are errors",
                // "Hi， when I make the code, there is an error",
            ],
            &["Hi， when I make the code, there is a error"],
        );
    }

    #[test]
    fn fix_there_are_singular_good_and_bad() {
        assert_good_and_bad_suggestions(
            "there are person",
            IsThereAgreement::default(),
            &[
                "there are people",
                // "there is a person"
            ],
            &["there is an person"],
        );
    }

    #[test]
    fn fix_there_are_singular() {
        assert_suggestion_result(
            "there are person",
            IsThereAgreement::default(),
            "there are people",
        );
    }

    #[test]
    fn dont_flag_there_are_compound_singular() {
        assert_no_lints("there are config errors", IsThereAgreement::default());
    }

    // Real-world tests

    // there is pl

    #[test]
    fn fix_there_is_errors() {
        assert_suggestion_result(
            "Hi， when I make the code, there is errors",
            IsThereAgreement::default(),
            "Hi， when I make the code, there are errors",
            // "Hi， when I make the code, there is an error",
        );
    }

    #[test]
    fn fix_there_is_warnings() {
        assert_suggestion_result(
            "There is warnings from kotlin and dart, as reference: Elvis operator (?:) always returns the left operand of non-nullable type String.",
            IsThereAgreement::default(),
            "There are warnings from kotlin and dart, as reference: Elvis operator (?:) always returns the left operand of non-nullable type String.",
            // "There is a warning from kotlin and dart, as reference: Elvis operator (?:) always returns the left operand of non-nullable type String.",
        );
    }

    #[test]
    fn fix_there_is_problems() {
        assert_suggestion_result(
            "Problem is that if there is a project that has a csproj file, then there is problems with the history folder.",
            IsThereAgreement::default(),
            "Problem is that if there is a project that has a csproj file, then there are problems with the history folder.",
            // "Problem is that if there is a project that has a csproj file, then there is a problem with the history folder.",
        );
    }

    #[test]
    fn fix_there_is_commands() {
        assert_suggestion_result(
            "Additionally if there is Commands that can be used on multiple Resources at the same time.",
            IsThereAgreement::default(),
            "Additionally if there are Commands that can be used on multiple Resources at the same time.",
            // "Additionally if there is a Command that can be used on multiple Resources at the same time.",
        );
    }

    #[test]
    fn fix_there_is_values() {
        assert_suggestion_result(
            "This mean there would not be a single cache, but as many caches as there is values for the second argument.",
            IsThereAgreement::default(),
            "This mean there would not be a single cache, but as many caches as there are values for the second argument.",
            // ❌ "This mean there would not be a single cache, but as many caches as there is a value for the second argument.",
        );
    }

    #[test]
    fn fix_there_is_strings() {
        assert_suggestion_result(
            "I can image other cases (tools different from SPSS) in which there is strings in both sides of the dictionary",
            IsThereAgreement::default(),
            "I can image other cases (tools different from SPSS) in which there are strings in both sides of the dictionary",
            // "I can image other cases (tools different from SPSS) in which there is a string in both sides of the dictionary",
        );
    }

    #[test]
    fn fix_there_is_things() {
        assert_suggestion_result(
            "even though we can check whether there is things running in Node, we can not do it for Chromium's message loops",
            IsThereAgreement::default(),
            "even though we can check whether there are things running in Node, we can not do it for Chromium's message loops",
        );
    }

    #[test]
    fn fix_there_is_people() {
        assert_suggestion_result(
            "there is people making projects, there is people doing tutorials",
            IsThereAgreement::default(),
            "there are people making projects, there are people doing tutorials",
        );
    }

    #[test]
    fn fix_there_is_instructions() {
        assert_suggestion_result(
            "I am just wondering if there is instructions somewhere for handling deep linking while using Redux.",
            IsThereAgreement::default(),
            "I am just wondering if there are instructions somewhere for handling deep linking while using Redux.",
        );
    }

    #[test]
    fn fix_there_is_packages() {
        assert_suggestion_result(
            "if there is packages that handle such protocols installed",
            IsThereAgreement::default(),
            "if there are packages that handle such protocols installed",
        );
    }

    // there is pl - legit - not error

    #[test]
    fn dont_flag_there_is_people_who_have_done_xyz() {
        assert_no_lints(
            "The main expectation there is people who have a deprecated app installed will then get an error if it's disabled",
            IsThereAgreement::default(),
        );
    }

    #[test]
    fn dont_flag_there_is_packages_part_of_dir() {
        assert_no_lints(
            "For example there is packages/vite/src/node , but no packages/vite/src/deno",
            IsThereAgreement::default(),
        );
    }

    // there are sg

    #[test]
    fn fix_there_are_issue() {
        assert_suggestion_result(
            "Seems like if there are issue with OpenAI, it is still trying to call chat completion and giving type error.",
            IsThereAgreement::default(),
            "Seems like if there are issues with OpenAI, it is still trying to call chat completion and giving type error.",
        );
    }

    //  there are sg-pl compound

    // Wrong advice when there are dependency conflicts
    #[test]
    fn dont_flag_there_are_dep_conflicts() {
        assert_no_lints(
            "Wrong advice when there are dependency conflicts",
            IsThereAgreement::default(),
        );
    }

    // If there are dependency errors they will be immediately logged out to you.
    #[test]
    fn dont_flag_there_are_dep_errs() {
        assert_no_lints(
            "If there are dependency errors they will be immediately logged out to you.",
            IsThereAgreement::default(),
        );
    }

    // New Campaign Properties dialog loses changes if there are definition syntax errors
    #[test]
    fn dont_flag_there_are_def_syntax_errs() {
        assert_no_lints(
            "New Campaign Properties dialog loses changes if there are definition syntax errors",
            IsThereAgreement::default(),
        );
    }

    //  there are sg - legit - not error
    //   clang-format indents class member functions oddly if there are function-like macro invocations

    #[test]
    fn dont_flag_alice_there_were_two() {
        assert_no_lints(
            "This time there were two little shrieks, and more sounds of broken glass.",
            IsThereAgreement::default(),
        );
    }

    #[test]
    fn dont_flag_gatsby_there_were_twinkle_bells() {
        assert_no_lints(
            "When he realized what I was talking about, that there were twinkle-bells of sunshine in the room, he smiled like a weather man",
            IsThereAgreement::default(),
        );
    }

    // there was pl

    // there were sg

    // is there pl
    //  please guys tell me, is there people really making money with bot?
    //  Run-as binary without the suid bit set, is there solutions?

    // are there sg

    // was there pl

    // were there sg
}
