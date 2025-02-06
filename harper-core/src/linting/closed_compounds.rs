use crate::{
    patterns::{ExactPhrase, Pattern},
    Token, TokenStringExt,
};

use super::{Lint, LintKind, PatternLinter, Suggestion};

macro_rules! create_closed_compound_linter {
    ($name:ident, $phrase:literal, $correct:expr) => {
        pub struct $name {
            pattern: Box<dyn Pattern>,
        }

        impl Default for $name {
            fn default() -> Self {
                let pattern = ExactPhrase::from_phrase($phrase);

                Self {
                    pattern: Box::new(pattern),
                }
            }
        }

        impl PatternLinter for $name {
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
                        $correct.chars().collect(),
                        orig_chars,
                    )],
                    message: format!("Did you mean the closed compound `{}`?", $correct),
                    ..Default::default()
                }
            }

            fn description(&self) -> &'static str {
                concat!(
                    "Looks for incorrect spacing inside the closed compound `",
                    $correct,
                    "`."
                )
            }
        }
    };
}

create_closed_compound_linter!(Itself, "it self", "itself");
create_closed_compound_linter!(Tonight, "to night", "tonight");
create_closed_compound_linter!(Myself, "my self", "myself");
create_closed_compound_linter!(Therefore, "there fore", "therefore");
create_closed_compound_linter!(Misunderstand, "miss understand", "misunderstand");
create_closed_compound_linter!(Misunderstood, "miss understood", "misunderstood");
create_closed_compound_linter!(Misuse, "miss use", "misuse");
create_closed_compound_linter!(Misused, "miss used", "misused");
create_closed_compound_linter!(Postpone, "post pone", "postpone");
create_closed_compound_linter!(Worldwide, "world wide", "worldwide");
create_closed_compound_linter!(Overall, "over all", "overall");
create_closed_compound_linter!(Likewise, "like wise", "likewise");
create_closed_compound_linter!(However, "how ever", "however");
create_closed_compound_linter!(Altogether, "all together", "altogether");
create_closed_compound_linter!(Asleep, "a sleep", "asleep");
create_closed_compound_linter!(Upset, "up set", "upset");
create_closed_compound_linter!(Thumbnail, "thumb nail", "thumbnail");
create_closed_compound_linter!(Intact, "in tact", "intact");
create_closed_compound_linter!(Somehow, "some how", "somehow");
create_closed_compound_linter!(Bathroom, "bath room", "bathroom");
create_closed_compound_linter!(Facewash, "face wash", "facewash");
create_closed_compound_linter!(Handheld, "hand held", "handheld");
create_closed_compound_linter!(Playground, "play ground", "playground");
create_closed_compound_linter!(Runway, "run way", "runway");
create_closed_compound_linter!(Northeast, "north east", "northeast");
create_closed_compound_linter!(Northeastern, "north eastern", "northeastern");
create_closed_compound_linter!(Upholstery, "up holstery", "upholstery");
create_closed_compound_linter!(Proofread, "proof read", "proofread");

#[cfg(test)]
mod tests {
    use super::{
        Altogether, Asleep, However, Itself, Likewise, Misunderstood, Misuse, Misused, Myself,
        Overall, Therefore, Tonight, Worldwide,
    };
    use crate::linting::tests::assert_suggestion_result;

    #[test]
    fn it_self() {
        let test_sentence = "The project, it self, was quite challenging.";
        let expected = "The project, itself, was quite challenging.";
        assert_suggestion_result(test_sentence, Itself::default(), expected);
    }

    #[test]
    fn to_night() {
        let test_sentence = "She spent the night to night.";
        let expected = "She spent the night tonight.";
        assert_suggestion_result(test_sentence, Tonight::default(), expected);
    }

    #[test]
    fn my_self() {
        let test_sentence = "He treated my self with respect.";
        let expected = "He treated myself with respect.";
        assert_suggestion_result(test_sentence, Myself::default(), expected);
    }

    #[test]
    fn there_fore() {
        let test_sentence = "This is the reason; there fore, this is true.";
        let expected = "This is the reason; therefore, this is true.";
        assert_suggestion_result(test_sentence, Therefore::default(), expected);
    }

    #[test]
    fn mis_understood() {
        let test_sentence = "She miss understood the instructions.";
        let expected = "She misunderstood the instructions.";
        assert_suggestion_result(test_sentence, Misunderstood::default(), expected);
    }

    #[test]
    fn mis_use() {
        let test_sentence = "He tends to miss use the tool.";
        let expected = "He tends to misuse the tool.";
        assert_suggestion_result(test_sentence, Misuse::default(), expected);
    }

    #[test]
    fn mis_used() {
        let test_sentence = "The software was miss used.";
        let expected = "The software was misused.";
        assert_suggestion_result(test_sentence, Misused::default(), expected);
    }

    #[test]
    fn world_wide() {
        let test_sentence = "The world wide impact was significant.";
        let expected = "The worldwide impact was significant.";
        assert_suggestion_result(test_sentence, Worldwide::default(), expected);
    }

    #[test]
    fn over_all() {
        let test_sentence = "The over all performance was good.";
        let expected = "The overall performance was good.";
        assert_suggestion_result(test_sentence, Overall::default(), expected);
    }

    #[test]
    fn like_wise() {
        let test_sentence = "He acted, like wise, without hesitation.";
        let expected = "He acted, likewise, without hesitation.";
        assert_suggestion_result(test_sentence, Likewise::default(), expected);
    }

    #[test]
    fn how_ever() {
        let test_sentence = "This is true, how ever, details matter.";
        let expected = "This is true, however, details matter.";
        assert_suggestion_result(test_sentence, However::default(), expected);
    }

    #[test]
    fn all_together() {
        let test_sentence = "They did it all together.";
        let expected = "They did it altogether.";
        assert_suggestion_result(test_sentence, Altogether::default(), expected);
    }

    #[test]
    fn a_sleep() {
        let test_sentence = "She fell a sleep.";
        let expected = "She fell asleep.";
        assert_suggestion_result(test_sentence, Asleep::default(), expected);
    }
}
