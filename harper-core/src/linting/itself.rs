use crate::{
    patterns::{Pattern, SequencePattern},
    Token, TokenStringExt,
};

use super::{Lint, LintKind, PatternLinter, Suggestion};

pub struct Itself {
    pattern: Box<dyn Pattern>,
}

impl Default for Itself {
    fn default() -> Self {
        let pattern = SequencePattern::default()
            .t_aco("it")
            .then_whitespace()
            .t_aco("self");

        Self {
            pattern: Box::new(pattern),
        }
    }
}

impl PatternLinter for Itself {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched_tokens: &[Token], source: &[char]) -> Lint {
        let span = matched_tokens.span().unwrap();
        let orig_chars = span.get_content(source);

        Lint {
            span,
            lint_kind: LintKind::WordChoice,
            suggestions: vec![Suggestion::replace_with_match_case(
                vec!['i', 't', 's', 'e', 'l', 'f'],
                orig_chars,
            )],
            message: "`Itself` is commonly mistaken for `it self`.".to_owned(),
            ..Default::default()
        }
    }

    fn description(&self) -> &'static str {
        "Lint for condensing `it self` to `itself`."
    }
}

#[cfg(test)]
mod tests {
    use super::Itself;
    use crate::linting::tests::assert_suggestion_result;

    #[test]
    fn it_self() {
        let test_sentence = "The project, it self, was quite challenging.";
        let expected = "The project, itself, was quite challenging.";
        assert_suggestion_result(test_sentence, Itself::default(), expected);
    }
}
