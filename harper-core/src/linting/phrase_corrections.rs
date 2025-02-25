use super::{Lint, LintKind, PatternLinter};
use crate::linting::Suggestion;
use crate::patterns::{ExactPhrase, OwnedPatternExt, Pattern, SimilarToPhrase};
use crate::{Token, TokenStringExt};

macro_rules! create_linter_map_phrase {
    ($name:ident, $pattern:expr, $($correct_form:literal).*, $message:expr, $description:expr) => {
        #[doc = $description]
        pub struct $name {
            pattern: Box<dyn Pattern>,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    pattern: Box::new($pattern),
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl PatternLinter for $name {
            fn pattern(&self) -> &dyn Pattern {
                self.pattern.as_ref()
            }

            fn match_to_lint(&self, matched_tokens: &[Token], source: &[char]) -> Option<Lint> {
                let span = matched_tokens.span()?;
                let matched_text = span.get_content(source);

                Some(Lint {
                    span,
                    lint_kind: LintKind::Miscellaneous,
                    suggestions: vec![$(
                        Suggestion::replace_with_match_case(
                            $correct_form.chars().collect(),
                            matched_text,
                        ),
                    )*],
                    message: $message.to_string(),
                    priority: 31,
                })
            }

            fn description(&self) -> &'static str {
                $description
            }
        }
    };
}

/// Generate a linter that will look for a common phrase and correct mild errors that
/// are still composed of real words.
macro_rules! create_linter_for_phrase {
    ($name:ident, $correct_form:literal, $dist:literal) => {
        create_linter_map_phrase!(
            $name,
            SimilarToPhrase::from_phrase($correct_form, $dist),
            $correct_form,
            concat!("Did you mean the phrase `", $correct_form, "`?"),
            concat!(
                "Looks for slight improper modifications to the phrase `",
                $correct_form,
                "`."
            )
        );
    };
}

create_linter_for_phrase!(TurnItOff, "turn it off", 1);
create_linter_for_phrase!(HumanLife, "human life", 1);
create_linter_for_phrase!(ThatChallenged, "that challenged", 2);
create_linter_for_phrase!(NoLonger, "no longer", 1);
create_linter_for_phrase!(NeedHelp, "need help", 1);
create_linter_for_phrase!(OfCourse, "of course", 1);
create_linter_for_phrase!(AndTheLike, "and the like", 1);
create_linter_for_phrase!(BadRap, "bad rap", 1);
create_linter_for_phrase!(BatedBreath, "bated breath", 1);
create_linter_for_phrase!(BeckAndCall, "beck and call", 1);
create_linter_for_phrase!(HungerPang, "hunger pang", 2);
create_linter_for_phrase!(EnMasse, "en masse", 1);
create_linter_for_phrase!(LetAlone, "let alone", 1);
create_linter_for_phrase!(SneakingSuspicion, "sneaking suspicion", 3);
create_linter_for_phrase!(SpecialAttention, "special attention", 1);
create_linter_for_phrase!(ThanOthers, "than others", 1);
create_linter_for_phrase!(SupposedTo, "supposed to", 1);

create_linter_map_phrase!(LoAndBehold, ExactPhrase::from_phrase("long and behold"), "lo and behold", "Did you mean `lo and behold`?", "Detects the exact phrase `long and behold` and suggests replacing it with the idiomatically correct `lo and behold`");
create_linter_map_phrase!(
    ChangeTack,
    ExactPhrase::from_phrase("change tact"),
    "change tack",
    "Did you mean the sailing idiom?",
    "Locates minor errors in the sailing idiom `change tack`."
);
create_linter_map_phrase!(WantBe, ExactPhrase::from_phrase("want be"),"won't be"."want to be","Did you mean `won't be` or `want to be`?", "Detects incorrect usage of `want be` and suggests `won't be` or `want to be` based on context.");
create_linter_map_phrase!(StateOfTheArt, ExactPhrase::from_phrase("state of art"), "state of the art", "Did you mean `state of the art`?", "Detects incorrect usage of `state of art` and suggests `state of the art` as the correct phrase.");
create_linter_map_phrase!(FastPaste, ExactPhrase::from_phrase("fast paste").or(Box::new(ExactPhrase::from_phrase("fast-paste"))), "fast-paced", "Did you mean `fast-paced`?", "Detects incorrect usage of `fast paste` or `fast-paste` and suggests `fast-paced` as the correct phrase.");

create_linter_map_phrase!(
    FaceFirst,
    ExactPhrase::from_phrase("face first into"),
    "Should this be `face-first`?",
    "face-first into",
    "Ensures `face first` is correctly hyphenated as `face-first` when used before `into`."
);

create_linter_map_phrase!(
    EludedTo,
    ExactPhrase::from_phrase("eluded to"),
    "alluded to",
    "Did you mean `alluded to`?",
    "Corrects `eluded to` to `alluded to` in contexts referring to indirect references."
);

create_linter_map_phrase!(
    BaitedBreath,
    ExactPhrase::from_phrase("baited breath"),
    "bated breath",
    "Did you mean `bated breath`?",
    "Ensures `bated breath` is written correctly, as `baited breath` is incorrect."
);

create_linter_map_phrase!(
    BareInMind,
    ExactPhrase::from_phrase("bare in mind"),
    "bear in mind",
    "Did you mean `bear in mind`?",
    "Ensures the phrase `bear in mind` is used correctly instead of `bare in mind`."
);

create_linter_map_phrase!(MutePoint, ExactPhrase::from_phrase("mute point"),
    "moot point",
    "Did you mean `moot point`?",
    "Ensures `moot point` is used instead of `mute point`, as `moot` means debatable or irrelevant.");

create_linter_map_phrase!(
    FarToMany,
    ExactPhrase::from_phrase("far to many"),
    "far too many",
    "Did you mean `far too many`?",
    "Ensures `too many` is used instead of `to many`."
);
create_linter_map_phrase!(
    FarToMuch,
    ExactPhrase::from_phrase("far to much"),
    "far too much",
    "Did you mean `far too much`?",
    "Ensures `too much` is used instead of `to much`."
);
create_linter_map_phrase!(
    WayToMany,
    ExactPhrase::from_phrase("way to many"),
    "way too many",
    "Did you mean `way too many`?",
    "Ensures `way too many` is used instead of `to many`."
);
create_linter_map_phrase!(
    WayToMuch,
    ExactPhrase::from_phrase("way to much"),
    "way too much",
    "Did you mean `way too much`?",
    "Ensures `way too much` is used instead of `to much`."
);

#[cfg(test)]
mod tests {
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    use super::{
        BadRap, BatedBreath, ChangeTack, EnMasse, FarToMany, FarToMuch, HungerPang, LetAlone,
        LoAndBehold, OfCourse, SneakingSuspicion, SpecialAttention, SupposedTo, ThanOthers,
        TurnItOff, WayToMany, WayToMuch,
    };

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

    #[test]
    fn operative_system() {
        assert_suggestion_result(
            "COS is a operative system made with the COSMOS Kernel and written in C#, COS its literally the same than MS-DOS but written in C# and open-source.",
            lint_group(),
            "COS is a operating system made with the COSMOS Kernel and written in C#, COS its literally the same than MS-DOS but written in C# and open-source.",
        );
    }

    #[test]
    fn than_others() {
        assert_suggestion_result("Then others", ThanOthers::default(), "Than others");
    }

    #[test]
    fn now_on_hold() {
        assert_lint_count(
            "Those are now on hold for month.",
            LoAndBehold::default(),
            0,
        );
    }

    #[test]
    fn far_to_many() {
        assert_suggestion_result(
            "darknet detecting far to many objects and in random locations",
            FarToMany::default(),
            "darknet detecting far too many objects and in random locations",
        );
    }

    #[test]
    fn far_to_much() {
        assert_suggestion_result(
            "requires far to much day today support and troubleshooting",
            FarToMuch::default(),
            "requires far too much day today support and troubleshooting",
        );
    }

    #[test]
    fn way_to_many() {
        assert_suggestion_result(
            "Way to many TEMP files",
            lint_group(),
            "Way too many TEMP files",
        );
    }

    #[test]
    fn way_to_much() {
        assert_suggestion_result(
            "proper java development has way to much overhead",
            WayToMuch::default(),
            "proper java development has way too much overhead",
        );
    }

    #[test]
    fn operative_system() {
        assert_suggestion_result(
            "COS is a operative system made with the COSMOS Kernel and written in C#, COS its literally the same than MS-DOS but written in C# and open-source.",
            lint_group(),
            "COS is a operating system made with the COSMOS Kernel and written in C#, COS its literally the same than MS-DOS but written in C# and open-source.",
        );
    }

    #[test]
    fn operative_systems() {
        assert_suggestion_result(
            "My dotfiles for my operative systems and other configurations.",
            lint_group(),
            "My dotfiles for my operating systems and other configurations.",
        );
    }
}
