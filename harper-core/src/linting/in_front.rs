use crate::{
    Lrc, Punctuation, Token,
    patterns::{EitherPattern, Invert, Pattern, SequencePattern},
};

use super::{Lint, LintKind, PatternLinter, Suggestion};

pub struct InFront {
    pattern: Box<dyn Pattern>,
}

impl Default for InFront {
    fn default() -> Self {
        let infront = Lrc::new(SequencePattern::default().t_aco("infront"));

        let not_hyphen = Lrc::new(Invert::new(|t: &Token, _source: &[char]| {
            t.kind.is_hyphen()
        }));

        let with_prev_not_hyphen = SequencePattern::default()
            .then(not_hyphen.clone())
            .then(infront.clone());

        let with_next_not_hyphen = SequencePattern::default()
            .then(infront.clone())
            .then(not_hyphen.clone());

        Self {
            pattern: Box::new(EitherPattern::new(vec![
                // Box::new(infront),
                Box::new(with_prev_not_hyphen),
                // Box::new(with_next_tok),
                Box::new(with_next_not_hyphen),
            ])),
        }
    }
}

impl PatternLinter for InFront {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched_toks: &[Token], source_chars: &[char]) -> Option<Lint> {
        let span = matched_toks.first()?.span;

        let chars = span.get_content(source_chars);

        eprintln!(">>{}<<", chars.iter().collect::<String>());

        // Ignore trademarks etc. like InFront, inFront
        if (chars[0] == 'i' || chars[0] == 'I') && chars[1..] == ['n', 'F', 'r', 'o', 'n', 't'] {
            return None;
        }

        Some(Lint {
            span,
            lint_kind: LintKind::WordChoice,
            suggestions: vec![Suggestion::replace_with_match_case_str(
                "in front",
                span.get_content(source_chars),
            )],
            message: "Use `in front` rather than `of` here.".to_string(),
            priority: 127,
        })
    }

    fn description(&self) -> &str {
        "Corrects `infront`, which should be written as two words."
    }
}

#[cfg(test)]
mod tests {
    use super::InFront;
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    #[test]
    fn corrects_lone_incase() {
        assert_suggestion_result(
            "Button always overlaps (infront) of other views.",
            InFront::default(),
            "Button always overlaps (in front) of other views.",
        );
    }

    #[test]
    fn corrects_infront() {
        assert_suggestion_result(
            "So if i have no variable or a running process id/name which indicates that liveley is infront/fullscreen i can't do anything further via batch and must wait ...",
            InFront::default(),
            "So if i have no variable or a running process id/name which indicates that liveley is in front/fullscreen i can't do anything further via batch and must wait ...",
        );
    }

    #[test]
    fn ignores_pascalcase() {
        assert_lint_count(
            "InFront Labs, LLC has 16 repositories available. Follow their code on GitHub.",
            InFront::default(),
            0,
        );
    }

    #[test]
    fn ignores_camelcase() {
        assert_lint_count(
            "Click the \"toggle\" button to see how wrapping changes when an inFront is added to a letter in a word.",
            InFront::default(),
            0,
        );
    }

    // it's actually the apostrophe that makes this a valid word
    // #[test]
    // fn ignore_with_hyphen_before() {
    //     assert_lint_count(
    //         "GitHub Gist: star and fork yossi-infront's gists by creating an account on GitHub.",
    //         InFront::default(),
    //         0,
    //     );
    // }

    #[test]
    fn correct_with_period_after() {
        assert_suggestion_result(
            "Car with a reversed ramp infront.",
            InFront::default(),
            "Car with a reversed ramp in front.",
        );
    }

    #[test]
    fn ignores_with_hyphen_before() {
        assert_lint_count(
            "Instantly share code, notes, and snippets. @yossi-infront",
            InFront::default(),
            0,
        );
    }

    #[test]
    fn ignores_with_hyphen_after() {
        assert_lint_count(
            "infront-cycle.ipe · infront-cycle.ipe · infront-cycle.svg · infront-cycle.svg · infront-s1s2.ipe · infront-s1s2.ipe · infront-s1s2.svg · infront-s1s2.svg.",
            // "infront-cycle.ipe",
            InFront::default(),
            0,
        );
    }
}
