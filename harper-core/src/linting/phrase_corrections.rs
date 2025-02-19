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


// "a","news" => "some news", False positives:
// a news <nominal phrase>
create_linter_map_phrase!(ANews, ExactPhrase::from_phrase("a news"), "some news", "`News` is not plural.", "Addresses `news` used as a plural.");
// "are","too","many","news" => "is too much news"
create_linter_map_phrase!(AreTooManyNews, ExactPhrase::from_phrase("are too many news"), "is too much news", "`News` is not plural.", "Addresses `news` used as a plural.");
// "each","news" => "each piece of news", False positives:
// each news <nominal phrase>
create_linter_map_phrase!(EachNews, ExactPhrase::from_phrase("each news"), "each piece of news", "`News` is not plural.", "Addresses `news` used as a plural.");

// "every","news" => "every piece of news", False positives:
// every news <nominal phrase>
create_linter_map_phrase!(EveryNews, ExactPhrase::from_phrase("every news"), "every piece of news", "`News` is not plural.", "Addresses `news` used as a plural.");

// "how","many","news" => "how much news", False positives:
// I want to see how many news headlines are related to ...
// How many news categories are there ...
create_linter_map_phrase!(HowManyNews, ExactPhrase::from_phrase("how many news"), "how much news", "`News` is not plural.", "Addresses `news` used as a plural.");
// "how","many","news","are" => "how much news is",
create_linter_map_phrase!(HowManyNewsAre, ExactPhrase::from_phrase("how many news are"), "how much news is", "`News` is not plural.", "Addresses `news` used as a plural.");
// "how","many","news","were" => "how much news was",
create_linter_map_phrase!(HowManyNewsWere, ExactPhrase::from_phrase("how many news were"), "how much news was", "`News` is not plural.", "Addresses `news` used as a plural.");
// "many","news" => "a lot of news"
// because they may have no or many news listing blocks
// Many news recommendation models
// Since we have that many news sources at our disposal
// features we may find in many news apps.
create_linter_map_phrase!(ManyNews, ExactPhrase::from_phrase("many news"), "a lot of news", "`News` is not plural.",
    "Addresses `news` used as a plural."
);
// "many","news","are" => "a lot of news is",
create_linter_map_phrase!(ManyNewsAre, ExactPhrase::from_phrase("many news are"), "much news is", "`News` is not plural.", "Addresses `news` used as a plural.");
// "many","news","were" => "a lot of news was",
create_linter_map_phrase!(ManyNewsWere, ExactPhrase::from_phrase("many news were"), "a lot of news was", "`News` is not plural.", "Addresses `news` used as a plural.");
// "news","are" => "news is", False positives:
// the search and facet blocks for news are shown
create_linter_map_phrase!(NewsAre, ExactPhrase::from_phrase("news are"), "news is", "`News` is not plural.",
    "Addresses `news` used as a plural."
);
// "news","were" => "news was", False positives:
// the characteristics of fake news and real news were very similar
// The URLs of news were scraped with Scripts\Scrape_URL.pyy
create_linter_map_phrase!(NewsWere, ExactPhrase::from_phrase("news were"), "news was", "`News` is not plural.",
    "Addresses `news` used as a plural."
);
// "too","many","news" => "too much news",
// Flooded with too many news articles
// Generating too many news messages at once ...
// create_linter_map_phrase!(TooManyNews, ExactPhrase::from_phrase("too many news"), "too much news", "`News` is not plural.", "Addresses `news` used as a plural.");





#[cfg(test)]
mod tests {
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    use super::{
        ANews,
        AreTooManyNews,
        BadRap, BatedBreath, ChangeTack,
        EachNews, EveryNews,
        EnMasse,
        HowManyNews,
        HowManyNewsAre, HowManyNewsWere,
        HungerPang, LetAlone, LoAndBehold,
        ManyNews,
        ManyNewsAre, ManyNewsWere,
        NewsAre, NewsWere,
        OfCourse,
        SneakingSuspicion, SpecialAttention, SupposedTo, ThanOthers,
        // TooManyNews,
        TurnItOff,
    };




    // Mostly participants said that they need a trigger to read a news such as a notification or some trending update
    #[test]
    fn a_news() {
        assert_suggestion_result("Mostly participants said that they need a trigger to read a news such as a notification or some trending update", ANews::default(), "Mostly participants said that they need a trigger to read some news such as a notification or some trending update");
    }
    #[test]
    fn are_too_many_news() {
        assert_suggestion_result("When there are too many news loaded (~10000) any changes of their visibility take too long", AreTooManyNews::default(), "When there is too much news loaded (~10000) any changes of their visibility take too long");
    }

    #[test]
    fn each_news() {
        assert_suggestion_result("each news is a separate document", EachNews::default(), "each piece of news is a separate document");        
    }
    // Because every news are bias in some way.
    #[test]
    fn every_news() {
        assert_suggestion_result("Because every news are bias in some way.", EveryNews::default(), "Because every piece of news is bias in some way.");
    }

    #[test]
    fn how_many_news() {
        assert_suggestion_result("you can specify how many news you want to get one time", HowManyNews::default(), "you can specify how much news you want to get one time");
    }
    #[test]
    fn how_many_news_are() {
        assert_suggestion_result("I would like to a number field so that I may choose exactly how many news are shown.", HowManyNewsAre::default(), "I would like to a number field so that I may choose exactly how much news is shown.");
    }    #[test]
    fn how_many_news_were() {
        assert_suggestion_result("How many news were published on the website?", HowManyNewsWere::default(), "How much news was published on the website?");
    }
    #[test]
    fn many_news() {
        assert_suggestion_result("An application has companies and news. Each company has many news.", ManyNews::default(), "An application has companies and news. Each company has a lot of news.");
    }
    #[test]
    fn many_news_are() {
        assert_suggestion_result("So many news are going on these days every time we turn on our TVs", ManyNewsAre::default(), "So much news is going on these days every time we turn on our TVs");
    }
    #[test]
    fn many_news_were() {
        assert_suggestion_result("During the last 3 years, many news were released in our official SAP Group", ManyNewsWere::default(), "During the last 3 years, a lot of news was released in our official SAP Group");
    }
    #[test]
    fn news_are() {
        assert_suggestion_result("The news are made via issues - one per announcement",
            NewsAre::default(), "The news is made via issues - one per announcement");
    }
    #[test]
    fn news_were() {
        assert_suggestion_result("The news were collected from **January to July of 2018**",
            NewsWere::default(), "The news was collected from **January to July of 2018**");
    }






    #[test]
    fn issue_574() {
        assert_lint_count("run by one", TurnItOff::default(), 0);
    }

    #[test]
    fn turn_it_off_clean_lower() {
        assert_lint_count("turn it off", TurnItOff::default(), 0);
    }

    #[test]
    fn turn_it_off_clean_upper() {
        assert_lint_count("Turn it off", TurnItOff::default(), 0);
    }

    #[test]
    fn of_confusion() {
        assert_suggestion_result("Turn it of", TurnItOff::default(), "Turn it off");
    }

    #[test]
    fn i_and_of_confusion() {
        assert_suggestion_result("Turn i of", TurnItOff::default(), "Turn it off");
    }

    #[test]
    fn off_course() {
        assert_suggestion_result(
            "Yes, off course we should do that.",
            OfCourse::default(),
            "Yes, of course we should do that.",
        );
    }

    #[test]
    fn o_course() {
        assert_suggestion_result(
            "Yes, o course we should do that.",
            OfCourse::default(),
            "Yes, of course we should do that.",
        );
    }

    #[test]
    fn bad_rep() {
        assert_suggestion_result("bad rep", BadRap::default(), "bad rap");
    }

    #[test]
    fn baited_breath() {
        assert_suggestion_result("baited breath", BatedBreath::default(), "bated breath");
    }

    #[test]
    fn change_tact() {
        assert_suggestion_result("change tact", ChangeTack::default(), "change tack");
    }

    #[test]
    fn hunger_pain() {
        assert_suggestion_result("hunger pain", HungerPang::default(), "hunger pang");
    }

    #[test]
    fn in_mass() {
        assert_suggestion_result("in mass", EnMasse::default(), "en masse");
    }

    #[test]
    fn let_along() {
        assert_suggestion_result("let along", LetAlone::default(), "let alone");
    }

    #[test]
    fn long_and_behold() {
        assert_suggestion_result("long and behold", LoAndBehold::default(), "lo and behold");
    }

    #[test]
    fn sneaky_suspicion() {
        assert_suggestion_result(
            "sneaky suspicion",
            SneakingSuspicion::default(),
            "sneaking suspicion",
        );
    }

    #[test]
    fn supposed_to() {
        assert_suggestion_result("suppose to", SupposedTo::default(), "supposed to");
    }

    #[test]
    fn spacial_attention() {
        assert_suggestion_result(
            "spacial attention",
            SpecialAttention::default(),
            "special attention",
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
}
