macro_rules! define_preferred_variants {
    (
        $(
            ($snuck:ident, $sneaked:ident)
        ),+
    ) => {
        paste::paste! {
            use crate::{
                CharStringExt, Lint, Token,
                expr::Expr,
                linting::{ExprLinter, LintKind, Suggestion, expr_linter::Chunk},
                patterns::Word,
            };

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum Prefer {
                $(
                    [<$snuck:camel>],
                    [<$sneaked:camel>],
                )*
            }

            fn build_expr(flag: Prefer) -> Word {
                match flag {
                    $(
                        Prefer::[<$snuck:camel>] => Word::new(stringify!($sneaked)),
                        Prefer::[<$sneaked:camel>] => Word::new(stringify!($snuck)),
                    )*
                }
            }

            fn to_lint(toks: &[Token], src: &[char], pref: Prefer) -> Option<Lint> {
                let tokspan = toks.first()?.span;
                let word = tokspan.get_content(src);

                let (target_word, source_word) = match pref {
                    $(
                        Prefer::[<$snuck:camel>] => {
                            if word.eq_str(stringify!($sneaked)) {
                                (stringify!($snuck), stringify!($sneaked))
                            } else {
                                return None;
                            }
                        }
                        Prefer::[<$sneaked:camel>] => {
                            if word.eq_str(stringify!($snuck)) {
                                (stringify!($sneaked), stringify!($snuck))
                            } else {
                                return None;
                            }
                        }
                    )*
                };

                Some(Lint {
                    span: tokspan,
                    lint_kind: LintKind::Usage,
                    suggestions: vec![Suggestion::replace_with_match_case_str(target_word, word)],
                    message: format!("Use `{}` instead of `{}`.", target_word, source_word),
                    ..Default::default()
                })
            }

            macro_rules! impl_expr_linter {
                ($name:ident, $pref:expr, $desc:expr) => {
                    impl Default for $name {
                        fn default() -> Self {
                            Self {
                                expr: build_expr($pref),
                            }
                        }
                    }

                    impl ExprLinter for $name {
                        type Unit = Chunk;

                        fn description(&self) -> &str {
                            $desc
                        }

                        fn match_to_lint(&self, toks: &[Token], src: &[char]) -> Option<Lint> {
                            to_lint(toks, src, $pref)
                        }

                        fn expr(&self) -> &dyn Expr {
                            &self.expr
                        }
                    }
                };
            }

            $(
                pub struct [<Prefer $snuck:camel>] {
                    expr: Word,
                }
                pub struct [<Prefer $sneaked:camel>] {
                    expr: Word,
                }

                impl_expr_linter!(
                    [<Prefer $snuck:camel>],
                    Prefer::[<$snuck:camel>],
                    concat!("Prefer `", stringify!($snuck), "` over `", stringify!($sneaked), "`.")
                );

                impl_expr_linter!(
                    [<Prefer $sneaked:camel>],
                    Prefer::[<$sneaked:camel>],
                    concat!("Prefer `", stringify!($sneaked), "` over `", stringify!($snuck), "`.")
                );
            )*
        }
    }
}

define_preferred_variants!((pleaded, pled), (sneaked, snuck), (gradable, gradeable));

#[cfg(test)]
mod tests {
    use super::{
        PreferGradable, PreferGradeable, PreferPleaded, PreferPled, PreferSneaked, PreferSnuck,
    };
    use crate::linting::tests::{assert_lint_count, assert_no_lints, assert_suggestion_result};

    // Prefer "snuck"

    #[test]
    fn correct_sneaked_to_snuck() {
        assert_suggestion_result(
            "He sneaked in around the back.",
            PreferSnuck::default(),
            "He snuck in around the back.",
        );
    }

    #[test]
    fn correct_sneaked_uppercase() {
        assert_suggestion_result(
            "He Sneaked in around the back.",
            PreferSnuck::default(),
            "He Snuck in around the back.",
        );
    }

    #[test]
    fn correct_sneaked_allcaps() {
        assert_suggestion_result(
            "He SNEAKED in around the back.",
            PreferSnuck::default(),
            "He SNUCK in around the back.",
        );
    }

    #[test]
    fn correct_to_snuck_from_github() {
        assert_suggestion_result(
            "recycled transitions, lingering inflation, copula swaps that sneaked through",
            PreferSnuck::default(),
            "recycled transitions, lingering inflation, copula swaps that snuck through",
        );
    }

    #[test]
    fn dont_flag_snuck_when_it_is_preferred() {
        assert_no_lints(
            "I'm not sure exactly when this fix snuck in",
            PreferSnuck::default(),
        );
    }

    // Prefer "sneaked"

    #[test]
    fn correct_snuck_to_sneaked() {
        assert_suggestion_result(
            "He snuck in around the back.",
            PreferSneaked::default(),
            "He sneaked in around the back.",
        );
    }

    #[test]
    fn correct_snuck_uppercase() {
        assert_suggestion_result(
            "He Snuck in around the back.",
            PreferSneaked::default(),
            "He Sneaked in around the back.",
        );
    }

    #[test]
    fn correct_snuck_allcaps() {
        assert_suggestion_result(
            "He SNUCK in around the back.",
            PreferSneaked::default(),
            "He SNEAKED in around the back.",
        );
    }

    #[test]
    fn correct_to_sneaked_from_github() {
        assert_suggestion_result(
            "A few unhandled Errors snuck their way into the code base over time f.ex. here:",
            PreferSneaked::default(),
            "A few unhandled Errors sneaked their way into the code base over time f.ex. here:",
        );
    }

    #[test]
    fn dont_flag_sneaked_when_it_is_preferred() {
        assert_no_lints(
            "Something related to recent experiments of WASM support sneaked into the main branch.",
            PreferSneaked::default(),
        );
    }

    // OneOfMany / Mutual exclusivity

    #[test]
    fn by_default_one_must_be_enabled_and_one_must_be_disabled_snuck() {
        use crate::Dialect;
        use crate::spell::FstDictionary;

        assert_lint_count(
            "I say 'snuck' but you say 'sneaked'.",
            crate::linting::LintGroup::new_curated(FstDictionary::curated(), Dialect::American),
            1,
        );
    }

    // Prefer "pled"

    #[test]
    fn correct_pleaded_to_pled() {
        assert_suggestion_result(
            "Reality Winner pleaded guilty in June to a single count of transmitting national security information.",
            PreferPled::default(),
            "Reality Winner pled guilty in June to a single count of transmitting national security information.",
        );
    }

    // Prefer "pleaded"

    #[test]
    fn correct_pled_to_pleaded() {
        assert_suggestion_result(
            "2005 Samsung pled guilty in connection with the cartel and paid a fine.",
            PreferPleaded::default(),
            "2005 Samsung pleaded guilty in connection with the cartel and paid a fine.",
        );
    }

    // OneOfMany / Mutual exclusivity

    #[test]
    fn by_default_one_must_be_enabled_and_one_must_be_disabled_pled() {
        use crate::Dialect;
        use crate::spell::FstDictionary;

        let mut lg =
            crate::linting::LintGroup::new_curated(FstDictionary::curated(), Dialect::American);
        lg.config.unset_rule_enabled("SpellCheck");

        assert_lint_count("I say 'pled' but you say 'pleaded'.", lg, 1);
    }

    // Others

    #[test]
    fn gradeable_to_gradable() {
        assert_suggestion_result(
            "Gradeable status should be part of the Gradeable model",
            PreferGradable::default(),
            "Gradable status should be part of the Gradable model",
        );
    }

    #[test]
    fn gradable_to_gradeable() {
        assert_suggestion_result(
            "A research project on the structure of gradable adjectives based on corpus analysis",
            PreferGradeable::default(),
            "A research project on the structure of gradeable adjectives based on corpus analysis",
        );
    }
}
