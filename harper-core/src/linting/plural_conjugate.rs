use crate::{
    Token,
    patterns::{OwnedPatternExt, Pattern, SequencePattern},
};

use super::{Lint, LintKind, PatternLinter, Suggestion};

pub struct PluralConjugate {
    pattern: Box<dyn Pattern>,
}

impl Default for PluralConjugate {
    fn default() -> Self {
        let plural_number = SequencePattern::default()
            .then_plural_nominal()
            .then_whitespace()
            .then_exact_word("is");

        let singular_number = SequencePattern::default()
            .then(|tok: &Token, _source: &[char]| {
                tok.kind.is_not_plural_nominal() && tok.kind.is_nominal()
            })
            .then_whitespace()
            .then_exact_word("are");

        let pat = plural_number.or(Box::new(singular_number));

        Self {
            pattern: Box::new(pat),
        }
    }
}

impl PatternLinter for PluralConjugate {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched_tokens: &[Token], _source: &[char]) -> Option<Lint> {
        let should_be_plural = matched_tokens.first()?.kind.is_plural_nominal();

        let sug = if should_be_plural {
            vec!['a', 'r', 'e']
        } else {
            vec!['i', 's']
        };

        Some(Lint {
            span: matched_tokens.last()?.span,
            lint_kind: LintKind::WordChoice,
            suggestions: vec![Suggestion::ReplaceWith(sug)],
            message: "Use the alternative conjugation of this verb to be consistent with the noun's plural nature.".to_owned(),
            priority: 63,
        })
    }

    fn description(&self) -> &'static str {
        "Make sure you use the correct conjugation of the verb \"to be\" in plural contexts."
    }
}

#[cfg(test)]
mod tests {
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    use super::PluralConjugate;

    #[test]
    fn issue_256() {
        assert_suggestion_result(
            "The bananas is tasty",
            PluralConjugate::default(),
            "The bananas are tasty",
        );
    }

    #[test]
    fn plural_students() {
        assert_suggestion_result(
            "The students is doing their homework.",
            PluralConjugate::default(),
            "The students are doing their homework.",
        );
    }

    #[test]
    fn singular_house() {
        assert_suggestion_result(
            "The house are just sitting there.",
            PluralConjugate::default(),
            "The house is just sitting there.",
        );
    }

    #[test]
    fn review_doc_page() {
        assert_lint_count(
            "If you are testing it, try harder.",
            PluralConjugate::default(),
            0,
        );
    }

    #[test]
    fn pronoun_singular_he_test() {
        assert_suggestion_result(
            "If he are testing it.",
            PluralConjugate::default(),
            "If he is testing it.",
        );
    }

    #[test]
    fn pronoun_singular_he_going() {
        assert_suggestion_result(
            "He are going to the store.",
            PluralConjugate::default(),
            "He is going to the store.",
        );
    }

    #[test]
    fn pronoun_singular_she() {
        assert_suggestion_result(
            "She are playing soccer.",
            PluralConjugate::default(),
            "She is playing soccer.",
        );
    }

    #[test]
    fn pronoun_singular_it() {
        assert_suggestion_result(
            "It are on the table.",
            PluralConjugate::default(),
            "It is on the table.",
        );
    }

    #[test]
    fn pronoun_plural_they() {
        assert_suggestion_result(
            "They is arriving soon.",
            PluralConjugate::default(),
            "They are arriving soon.",
        );
    }

    #[test]
    fn pronoun_plural_we() {
        assert_suggestion_result(
            "We is having lunch now.",
            PluralConjugate::default(),
            "We are having lunch now.",
        );
    }

    #[test]
    fn pronoun_plural_you() {
        assert_suggestion_result(
            "You is responsible for this task.",
            PluralConjugate::default(),
            "You are responsible for this task.",
        );
    }

    #[test]
    fn collective_noun_singular() {
        assert_suggestion_result(
            "The committee are meeting today.",
            PluralConjugate::default(),
            "The committee is meeting today.",
        );
    }
}
