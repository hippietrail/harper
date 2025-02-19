use super::{Lint, LintGroup, LintKind, PatternLinter};
use crate::linting::Suggestion;
use crate::patterns::{ExactPhrase, OwnedPatternExt, Pattern, SimilarToPhrase};
use crate::{Token, TokenStringExt};

struct MapPhraseLinter {
    description: String,
    pattern: Box<dyn Pattern>,
    correct_forms: Vec<String>,
    message: String,
}

impl MapPhraseLinter {
    pub fn new(
        pattern: Box<dyn Pattern>,
        correct_forms: impl IntoIterator<Item = impl ToString>,
        message: impl ToString,
        description: impl ToString,
    ) -> Self {
        Self {
            description: description.to_string(),
            pattern,
            correct_forms: correct_forms.into_iter().map(|f| f.to_string()).collect(),
            message: message.to_string(),
        }
    }

    pub fn new_similar_to_phrase(phrase: &'static str, detectable_distance: u8) -> Self {
        Self::new(
            Box::new(SimilarToPhrase::from_phrase(phrase, detectable_distance)),
            [phrase],
            format!("Did you mean the phrase `{phrase}`?"),
            format!("Looks for slight improper modifications to the phrase `{phrase}`."),
        )
    }
}

impl PatternLinter for MapPhraseLinter {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched_tokens: &[Token], source: &[char]) -> Lint {
        let span = matched_tokens.span().unwrap();
        let matched_text = span.get_content(source);

        Lint {
            span,
            lint_kind: LintKind::Miscellaneous,
            suggestions: self
                .correct_forms
                .iter()
                .map(|correct_form| {
                    Suggestion::replace_with_match_case(
                        correct_form.chars().collect(),
                        matched_text,
                    )
                })
                .collect(),
            message: self.message.to_string(),
            priority: 31,
        }
    }

    fn description(&self) -> &str {
        self.description.as_str()
    }
}

/// Produce a [`LintGroup`] that represents all the linters in this module.
/// Comes pre-configured with the recommended default settings.
pub fn lint_group() -> LintGroup {
    let mut group = LintGroup::default();

    group.add(
        "TurnItOff",
        Box::new(MapPhraseLinter::new_similar_to_phrase("turn it off", 1)),
    );
    group.add(
        "HumanLife",
        Box::new(MapPhraseLinter::new_similar_to_phrase("human life", 1)),
    );
    group.add(
        "ThatChallenged",
        Box::new(MapPhraseLinter::new_similar_to_phrase("that challenged", 2)),
    );
    group.add(
        "NoLonger",
        Box::new(MapPhraseLinter::new_similar_to_phrase("no longer", 1)),
    );
    group.add(
        "NeedHelp",
        Box::new(MapPhraseLinter::new_similar_to_phrase("need help", 1)),
    );
    group.add(
        "OfCourse",
        Box::new(MapPhraseLinter::new_similar_to_phrase("of course", 1)),
    );
    group.add(
        "AndTheLike",
        Box::new(MapPhraseLinter::new_similar_to_phrase("and the like", 1)),
    );
    group.add(
        "BadRap",
        Box::new(MapPhraseLinter::new_similar_to_phrase("bad rap", 1)),
    );
    group.add(
        "BatedBreath",
        Box::new(MapPhraseLinter::new_similar_to_phrase("bated breath", 1)),
    );
    group.add(
        "BeckAndCall",
        Box::new(MapPhraseLinter::new_similar_to_phrase("beck and call", 1)),
    );
    group.add(
        "HungerPang",
        Box::new(MapPhraseLinter::new_similar_to_phrase("hunger pang", 2)),
    );
    group.add(
        "EnMasse",
        Box::new(MapPhraseLinter::new_similar_to_phrase("en masse", 1)),
    );
    group.add(
        "LetAlone",
        Box::new(MapPhraseLinter::new_similar_to_phrase("let alone", 1)),
    );
    group.add(
        "SneakingSuspicion",
        Box::new(MapPhraseLinter::new_similar_to_phrase(
            "sneaking suspicion",
            3,
        )),
    );
    group.add(
        "SpecialAttention",
        Box::new(MapPhraseLinter::new_similar_to_phrase(
            "special attention",
            1,
        )),
    );
    group.add(
        "SupposedTo",
        Box::new(MapPhraseLinter::new_similar_to_phrase("supposed to", 1)),
    );

    group.add(
        "ChangeTack",
        Box::new(MapPhraseLinter::new(
            Box::new(ExactPhrase::from_phrase("change tact")),
            ["change tack"],
            "Did you mean the sailing idiom?",
            "Locates minor errors in the sailing idiom `change tack`.",
        )),
    );
    group.add(
    "WantBe",
    Box::new(MapPhraseLinter::new(
        Box::new(ExactPhrase::from_phrase("want be")),
        ["won't be", "want to be"],
        "Did you mean `won't be` or `want to be`?",
        "Detects incorrect usage of `want be` and suggests `won't be` or `want to be` based on context."
    ))
);

    group.add(
    "StateOfTheArt",
    Box::new(MapPhraseLinter::new(
        Box::new(ExactPhrase::from_phrase("state of art")),
        ["state of the art"],
        "Did you mean `state of the art`?",
        "Detects incorrect usage of `state of art` and suggests `state of the art` as the correct phrase."
    ))
);

    group.add(
    "FastPaste",
    Box::new(MapPhraseLinter::new(
        Box::new(ExactPhrase::from_phrase("fast paste").or(Box::new(ExactPhrase::from_phrase("fast-paste")))),
        ["fast-paced"],
        "Did you mean `fast-paced`?",
        "Detects incorrect usage of `fast paste` or `fast-paste` and suggests `fast-paced` as the correct phrase."
    ))
);

    group.add(
        "FaceFirst",
        Box::new(MapPhraseLinter::new(
            Box::new(ExactPhrase::from_phrase("face first into")),
            ["face-first into"],
            "Should this be `face-first`?",
            "Ensures `face first` is correctly hyphenated as `face-first` when used before `into`.",
        )),
    );

    group.add(
        "EludedTo",
        Box::new(MapPhraseLinter::new(
            Box::new(ExactPhrase::from_phrase("eluded to")),
            ["alluded to"],
            "Did you mean `alluded to`?",
            "Corrects `eluded to` to `alluded to` in contexts referring to indirect references.",
        )),
    );

    group.add(
        "BaitedBreath",
        Box::new(MapPhraseLinter::new(
            Box::new(ExactPhrase::from_phrase("baited breath")),
            ["bated breath"],
            "Did you mean `bated breath`?",
            "Ensures `bated breath` is written correctly, as `baited breath` is incorrect.",
        )),
    );

    group.add(
        "BareInMind",
        Box::new(MapPhraseLinter::new(
            Box::new(ExactPhrase::from_phrase("bare in mind")),
            ["bear in mind"],
            "Did you mean `bear in mind`?",
            "Ensures the phrase `bear in mind` is used correctly instead of `bare in mind`.",
        )),
    );

    group.add(
    "MutePoint",
    Box::new(MapPhraseLinter::new(
        Box::new(ExactPhrase::from_phrase("mute point")),
        ["moot point"],
        "Did you mean `moot point`?",
        "Ensures `moot point` is used instead of `mute point`, as `moot` means debatable or irrelevant."
    ))
        );

    group.set_all_rules_to(Some(true));

    group
}

#[cfg(test)]
mod tests {
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    use super::lint_group;

    #[test]
    fn issue_574() {
        assert_lint_count("run by one", lint_group(), 0);
    }

    #[test]
    fn turn_it_off_clean_lower() {
        assert_lint_count("turn it off", lint_group(), 0);
    }

    #[test]
    fn turn_it_off_clean_upper() {
        assert_lint_count("Turn it off", lint_group(), 0);
    }

    #[test]
    fn of_confusion() {
        assert_suggestion_result("Turn it of", lint_group(), "Turn it off");
    }

    #[test]
    fn i_and_of_confusion() {
        assert_suggestion_result("Turn i of", lint_group(), "Turn it off");
    }

    #[test]
    fn off_course() {
        assert_suggestion_result(
            "Yes, off course we should do that.",
            lint_group(),
            "Yes, of course we should do that.",
        );
    }

    #[test]
    fn o_course() {
        assert_suggestion_result(
            "Yes, o course we should do that.",
            lint_group(),
            "Yes, of course we should do that.",
        );
    }

    #[test]
    fn bad_rep() {
        assert_suggestion_result("bad rep", lint_group(), "bad rap");
    }

    #[test]
    fn baited_breath() {
        assert_suggestion_result("baited breath", lint_group(), "bated breath");
    }

    #[test]
    fn change_tact() {
        assert_suggestion_result("change tact", lint_group(), "change tack");
    }

    #[test]
    fn hunger_pain() {
        assert_suggestion_result("hunger pain", lint_group(), "hunger pang");
    }

    #[test]
    fn in_mass() {
        assert_suggestion_result("in mass", lint_group(), "en masse");
    }

    #[test]
    fn let_along() {
        assert_suggestion_result("let along", lint_group(), "let alone");
    }

    #[test]
    fn sneaky_suspicion() {
        assert_suggestion_result("sneaky suspicion", lint_group(), "sneaking suspicion");
    }

    #[test]
    fn supposed_to() {
        assert_suggestion_result("suppose to", lint_group(), "supposed to");
    }

    #[test]
    fn spacial_attention() {
        assert_suggestion_result("spacial attention", lint_group(), "special attention");
    }

    #[test]
    fn now_on_hold() {
        assert_lint_count("Those are now on hold for month.", lint_group(), 0);
    }
}
