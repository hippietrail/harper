use crate::{
    Lrc, Token,
    patterns::{EitherPattern, Pattern, SequencePattern},
};

use super::{Lint, LintKind, PatternLinter, Suggestion};

pub struct InFront {
    pattern: Box<dyn Pattern>,
}

impl Default for InFront {
    fn default() -> Self {
        let infront = Lrc::new(SequencePattern::default().t_aco("infront"));

        let with_prev = SequencePattern::default()
            .then_anything()
            .then(infront.clone());

        let with_next = SequencePattern::default()
            .then(infront.clone())
            .then_anything();

        let with_prev_and_next = SequencePattern::default()
            .then_anything()
            .then(infront.clone())
            .then_anything();

        Self {
            pattern: Box::new(EitherPattern::new(vec![
                Box::new(with_prev_and_next),
                Box::new(with_prev),
                Box::new(with_next),
                Box::new(infront),
            ])),
        }
    }
}

impl PatternLinter for InFront {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched_toks: &[Token], source_chars: &[char]) -> Option<Lint> {
        let infront_idx = get_infront_idx(matched_toks, source_chars);
        let infront_span = matched_toks[infront_idx].span;
        let infront_chars = infront_span.get_content(source_chars);

        // Ignore if there's a hyphen immediately on either side
        if (0..matched_toks.len())
            .filter(|&i| i != infront_idx)
            .any(|i| matched_toks[i].kind.is_hyphen())
        {
            return None;
        }

        // Ignore trademarks etc. like InFront, inFront
        if (infront_chars[0] == 'i' || infront_chars[0] == 'I')
            && infront_chars[1..] == ['n', 'F', 'r', 'o', 'n', 't']
        {
            return None;
        }

        let template = infront_span.get_content(source_chars);

        Some(Lint {
            span: infront_span,
            lint_kind: LintKind::WordChoice,
            suggestions: vec![Suggestion::replace_with_match_case_str(
                "in front", template,
            )],
            message: "`In front` should be written as two words.".to_string(),
            priority: 127,
        })
    }

    fn description(&self) -> &str {
        "Corrects `infront`, which should be written as two words."
    }
}

fn get_infront_idx(toks: &[Token], src: &[char]) -> usize {
    const LEN: usize = "infront".len();

    if toks.len() != 1 && toks.len() != 2 && toks.len() != 3 {
        unreachable!();
    }

    // we matched some lettercase variant of `infront` on its own
    if toks.len() == 1 {
        return 0;
    }

    // we matched `infront` with a non-hyphen token on one side
    if toks.len() == 3 {
        return 1;
    }

    // we matched a non-hyphen token on one side of `infront`
    let len0 = toks[0].span.len();
    let len1 = toks[1].span.len();

    if len0 == LEN && len1 != LEN {
        return 0;
    } else if len1 == LEN && len0 != LEN {
        return 1;
    }

    // both tokens the right length, check kind
    if toks[0].kind.is_word() && !toks[1].kind.is_word() {
        return 0;
    } else if !toks[0].kind.is_word() && toks[1].kind.is_word() {
        return 1;
    }

    // both tokens: words the right length, check the chars
    !toks[0]
        .span
        .get_content(src)
        .iter()
        .collect::<String>()
        .eq_ignore_ascii_case("infront") as usize
}

#[cfg(test)]
mod tests {
    use super::InFront;
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    #[test]
    fn corrects_lone_infront() {
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

    #[test]
    fn correct_with_period_after() {
        assert_suggestion_result(
            "Car with a reversed ramp infront.",
            InFront::default(),
            "Car with a reversed ramp in front.",
        );
    }

    #[test]
    fn ignore_hyphen_before() {
        assert_lint_count("-infront", InFront::default(), 0);
    }

    #[test]
    fn ignore_hyphen_after() {
        assert_lint_count("infront-", InFront::default(), 0);
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
            InFront::default(),
            0,
        );
    }

    #[test]
    fn even_repeated_infront_works() {
        assert_suggestion_result("infront infront", InFront::default(), "in front in front");
    }
}
