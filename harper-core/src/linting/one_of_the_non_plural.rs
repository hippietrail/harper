use crate::Token;
use crate::expr::{Expr, SequenceExpr};
use crate::linting::{ExprLinter, Lint, LintKind, Suggestion};
use crate::spell::Dictionary;

pub struct OneOfTheNonPlural<D> {
    expr: Box<dyn Expr>,
    dict: D,
}

impl SequenceExpr {
    pub fn then_our_noun(self) -> Self {
        self.then_kind_except(|k| k.is_noun(), &["as", "behind", "loses", "while"])
    }
}

impl<D> OneOfTheNonPlural<D>
where
    D: Dictionary,
{
    pub fn new(dict: D) -> Self {
        // let then_our_noun = SequenceExpr::default().then_noun();

        let noun_phrase = SequenceExpr::optional(SequenceExpr::default().then_determiner().t_ws())
            // .then_optional(SequenceExpr::default().then_degree_adverb().t_ws())
            .then_optional_degree_adverb_then_space()
            // .then_optional(SequenceExpr::default().then_adjective().t_ws())
            .then_optional_adjective_then_space()
            .then(
                SequenceExpr::default().then_our_noun().then_optional(
                    SequenceExpr::default()
                        .then_one_or_more(SequenceExpr::default().t_ws().then_our_noun()),
                ),
            );

        Self {
            expr: Box::new(SequenceExpr::fixed_phrase("one of the ").then(noun_phrase)),
            dict,
        }
    }
}

impl<D> ExprLinter for OneOfTheNonPlural<D>
where
    D: Dictionary,
{
    fn expr(&self) -> &dyn Expr {
        self.expr.as_ref()
    }

    fn match_to_lint(&self, toks: &[Token], src: &[char]) -> Option<Lint> {
        // Chop off the fixed phrase and keep the noun phrase
        let toks = &toks[6..];

        let last_tok = toks.last().unwrap();

        // If the NP (ends with a) plural then all is fine, nothing to flag.
        if last_tok.kind.is_plural_noun() {
            return None;
        }

        // TODO: If the NP ends with a possessive, ignore it for now.
        // TODO: But it can actually introduce a "sub-NP" starting from the optional adverb step.
        // ... had put on one of the Rabbit’s little white kid gloves while she was talking.
        if last_tok.kind.is_possessive_noun() {
            return None;
        }

        let singular = last_tok.span.get_content_string(src);

        let plural_s = singular.clone() + "s";
        let plural_es = singular + "es";

        let mut suggestions = Vec::new();

        if let Some(metadata) = self.dict.get_word_metadata_str(&plural_s)
            && metadata.is_plural_noun()
        {
            suggestions.push(Suggestion::replace_with_match_case(
                plural_s.chars().collect(),
                last_tok.span.get_content(src),
            ));
        }

        if let Some(metadata) = self.dict.get_word_metadata_str(&plural_es)
            && metadata.is_plural_noun()
        {
            suggestions.push(Suggestion::replace_with_match_case(
                plural_es.chars().collect(),
                last_tok.span.get_content(src),
            ));
        }

        Some(Lint {
            span: last_tok.span,
            lint_kind: LintKind::Agreement,
            suggestions,
            message: "Use plural nouns after 'one of the' (e.g., 'one of the things' not 'one of the thing')".to_string(),
            ..Default::default()
        })
    }

    fn description(&self) -> &'static str {
        "Use plural nouns after 'one of the' (e.g., 'one of the things' not 'one of the thing')"
    }
}

#[cfg(test)]
mod tests {
    use crate::linting::{
        OneOfTheNonPlural,
        tests::{assert_no_lints, assert_suggestion_result, assert_top3_suggestion_result},
    };
    use crate::spell::FstDictionary;

    #[test]
    fn one_of_the_thing() {
        assert_suggestion_result(
            "one of the thing",
            OneOfTheNonPlural::new(FstDictionary::curated()),
            "one of the things",
        );
    }

    #[test]
    fn one_of_the_singular() {
        assert_suggestion_result(
            "... noticed occasional production issue when one of the node loses connection",
            OneOfTheNonPlural::new(FstDictionary::curated()),
            "... noticed occasional production issue when one of the nodes loses connection",
        );
    }

    #[test]
    fn one_of_the_adjective_singular() {
        assert_suggestion_result(
            "one of the neat trick with AVX-512 is that given a mask",
            OneOfTheNonPlural::new(FstDictionary::curated()),
            "one of the neat tricks with AVX-512 is that given a mask",
        );
    }

    #[test]
    fn one_of_the_superlative_adjective_singular() {
        assert_suggestion_result(
            "Footer line shown since one of the latest version #11794",
            OneOfTheNonPlural::new(FstDictionary::curated()),
            "Footer line shown since one of the latest versions #11794",
        );
    }

    #[test]
    fn one_of_the_past_participle_singular() {
        assert_suggestion_result(
            "Sublime Merge hangs if one of the unstaged file is a pretty ...",
            OneOfTheNonPlural::new(FstDictionary::curated()),
            "Sublime Merge hangs if one of the unstaged files is a pretty ...",
        );
    }

    #[test]
    fn one_of_the_proper_noun_singular() {
        assert_top3_suggestion_result(
            "One of the Brave Process is consuming almost 170%",
            OneOfTheNonPlural::new(FstDictionary::curated()),
            "One of the Brave Processes is consuming almost 170%",
        );
    }

    #[test]
    fn one_of_the_most_adjective_singular() {
        assert_suggestion_result(
            "One of the most cumbersome thing to create in markdown is a table.",
            OneOfTheNonPlural::new(FstDictionary::curated()),
            "One of the most cumbersome things to create in markdown is a table.",
        );
    }

    #[test]
    fn dont_flag_one_of_the_singular_plural() {
        assert_no_lints(
            "exactly one of the delimiter lines is not present",
            OneOfTheNonPlural::new(FstDictionary::curated()),
        );
    }

    #[test]
    #[ignore = "A false positive we can't detect yet - maybe the preceding 'the'?"]
    fn dont_flag_the_one_of_the_initialism_singular() {
        assert_no_lints(
            "Directory of file renaming should be the one of the PDF file #3884",
            OneOfTheNonPlural::new(FstDictionary::curated()),
        );
    }
}
