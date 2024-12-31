use super::PatternLinter;
use super::{Lint, LintKind, Suggestion};
use crate::make_title_case;
use crate::patterns::{EitherPattern, IsNotTitleCase, Pattern, SequencePattern};
use crate::FstDictionary;
use crate::{Token, TokenStringExt};
use std::sync::Arc;

/// A macro that will generate a linter to enforce capitalization of a multi-token proper noun.
macro_rules! create_linter_for {
    ($name:ident, $pattern:expr, $message:literal) => {
        create_linter_for!($name, $pattern, $message, $message);
    };
    ($name:ident, $pattern:expr, $message:literal, $description:literal) => {
        pub struct $name {
            pattern: Box<dyn Pattern>,
            dict: Arc<FstDictionary>,
        }

        impl $name {
            pub fn new() -> Self {
                let dict = FstDictionary::curated();

                Self {
                    pattern: Box::new(IsNotTitleCase::new(Box::new($pattern), dict.clone())),
                    dict,
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

            fn match_to_lint(&self, matched_tokens: &[Token], source: &[char]) -> Lint {
                let proper = make_title_case(matched_tokens, source, &self.dict);

                Lint {
                    span: matched_tokens.span().unwrap(),
                    lint_kind: LintKind::Capitalization,
                    suggestions: vec![Suggestion::ReplaceWith(proper)],
                    message: $message.to_string(),
                    priority: 31,
                }
            }

            fn description(&self) -> &'static str {
                $description
            }
        }
    };
}

create_linter_for!(
    Americas,
    SequencePattern::default()
        .then(Box::new(EitherPattern::new(vec![
            Box::new(SequencePattern::default().then_any_capitalization_of("South")),
            Box::new(SequencePattern::default().then_any_capitalization_of("North"))
        ])))
        .then_whitespace()
        .then_any_capitalization_of("America"),
    "When referring to the continents, make sure to treat them as a proper noun."
);

create_linter_for!(
    ChineseCommunistParty,
    SequencePattern::default()
        .then_any_capitalization_of("Chinese")
        .then_whitespace()
        .then_any_capitalization_of("Communist")
        .then_whitespace()
        .then_any_capitalization_of("Party"),
    "When referring to the political party, make sure to treat them as a proper noun."
);

create_linter_for!(
    UnitedOrganizations,
    SequencePattern::default()
        .then_any_capitalization_of("United")
        .then_whitespace()
        .then(Box::new(EitherPattern::new(vec![
            Box::new(SequencePattern::default().then_any_capitalization_of("Nations")),
            Box::new(SequencePattern::default().then_any_capitalization_of("States")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Kingdom")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Airlines")),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("Arab")
                    .then_whitespace()
                    .then_any_capitalization_of("Emirates")
            )
        ]))),
    "When referring to national or international organizations, make sure to treat them as a proper noun."
);

create_linter_for!(
    Holidays,
    EitherPattern::new(vec![
        Box::new(
            SequencePattern::default()
                .then(Box::new(EitherPattern::new(vec![
                    Box::new(SequencePattern::default().then_any_capitalization_of("Presidents'")),
                    Box::new(SequencePattern::default().then_any_capitalization_of("Valentines")),
                    Box::new(SequencePattern::default().then_any_capitalization_of("Christmas")),
                    Box::new(SequencePattern::default().then_any_capitalization_of("Easter")),
                    Box::new(SequencePattern::default().then_any_capitalization_of("Flag")),
                    Box::new(SequencePattern::default().then_any_capitalization_of("Independence")),
                    Box::new(SequencePattern::default().then_any_capitalization_of("Mothers'")),
                    Box::new(
                        SequencePattern::default()
                            .then_any_capitalization_of("New")
                            .then_any_capitalization_of("Years")
                    ),
                    Box::new(SequencePattern::default().then_any_capitalization_of("Fathers'")),
                    Box::new(SequencePattern::default().then_any_capitalization_of("Columbus")),
                    Box::new(SequencePattern::default().then_any_capitalization_of("Thanksgiving")),
                    Box::new(SequencePattern::default().then_any_capitalization_of("Memorial")),
                    Box::new(SequencePattern::default().then_any_capitalization_of("May")),
                    Box::new(SequencePattern::default().then_any_capitalization_of("Halloween")),
                    Box::new(SequencePattern::default().then_any_capitalization_of("Tax")),
                    Box::new(SequencePattern::default().then_any_capitalization_of("Parents")),
                    Box::new(SequencePattern::default().then_any_capitalization_of("Veterans")),
                    Box::new(SequencePattern::default().then_any_capitalization_of("Armistice")),
                    Box::new(SequencePattern::default().then_any_capitalization_of("Groundhog")),
                    Box::new(
                        SequencePattern::default()
                            .then_any_capitalization_of("National")
                            .then_whitespace()
                            .then_any_capitalization_of("Freedom")
                    ),
                    Box::new(
                        SequencePattern::default()
                            .then_any_capitalization_of("All")
                            .then_whitespace()
                            .then_any_capitalization_of("Saints")
                    ),
                    Box::new(
                        SequencePattern::default()
                            .then_any_capitalization_of("All")
                            .then_whitespace()
                            .then_any_capitalization_of("Souls")
                    )
                ])))
                .then_whitespace()
                .then_any_capitalization_of("Day")
        ),
        Box::new(
            SequencePattern::default()
                .then_any_capitalization_of("Black")
                .then_whitespace()
                .then_any_capitalization_of("Friday")
        ),
        Box::new(
            SequencePattern::default()
                .then_any_capitalization_of("Cyber")
                .then_whitespace()
                .then_any_capitalization_of("Monday")
        )
    ]),
    "When referring to holidays, make sure to treat them as a proper noun."
);

create_linter_for!(
    AmazonNames,
    SequencePattern::default()
    .then_any_capitalization_of("Amazon")
    .then_whitespace()
    .then(Box::new(EitherPattern::new(vec![
        Box::new(
            SequencePattern::default()
                .then_any_capitalization_of("Shopping")
        ),
        Box::new(
            SequencePattern::default()
                .then_any_capitalization_of("Web")
                    .then_whitespace()
                .then_any_capitalization_of("Services")
        ),
        Box::new(
            SequencePattern::default()
                .then_any_capitalization_of("Lambda")
        ),
        Box::new(
            SequencePattern::default()
                .then_any_capitalization_of("RDS")
        ),
        Box::new(
            SequencePattern::default()
                .then_any_capitalization_of("DynamoDB")
        ),
        Box::new(
            SequencePattern::default()
                .then_any_capitalization_of("SageMaker")
        ),
        Box::new(
            SequencePattern::default()
                .then_any_capitalization_of("Rekognition")
        ),
        Box::new(
            SequencePattern::default()
                .then_any_capitalization_of("CloudFront")
        ),
        Box::new(
            SequencePattern::default()
                .then_any_capitalization_of("ECS")
        ),
        Box::new(
            SequencePattern::default()
                .then_any_capitalization_of("EKS")
        ),
        Box::new(
            SequencePattern::default()
                .then_any_capitalization_of("CloudWatch")
        ),
        Box::new(
            SequencePattern::default()
                .then_any_capitalization_of("IAM")
        ),
        Box::new(
            SequencePattern::default()
                .then_any_capitalization_of("Prime")
        ),
        Box::new(
            SequencePattern::default()
                .then_any_capitalization_of("Kindle")
        )
    ]))),
    "When referring to the various products of Amazon.com, make sure to treat them as a proper noun."
);

create_linter_for!(
    GoogleNames,
    SequencePattern::default()
        .then_any_capitalization_of("Google")
        .then_whitespace()
        .then(Box::new(EitherPattern::new(vec![
            Box::new(SequencePattern::default().then_any_capitalization_of("Search")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Cloud")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Maps")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Docs")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Sheets")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Slides")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Drive")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Meet")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Gmail")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Calendar")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Chrome")),
            Box::new(SequencePattern::default().then_any_capitalization_of("ChromeOS")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Android")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Play")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Bard")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Gemini")),
            Box::new(SequencePattern::default().then_any_capitalization_of("YouTube")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Photos")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Analytics")),
            Box::new(SequencePattern::default().then_any_capitalization_of("AdSense")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Pixel")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Nest")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Workspace"))
        ]))),
    "When referring to Google products and services, make sure to treat them as proper nouns."
);

create_linter_for!(
    AzureNames,
    SequencePattern::default()
        .then_any_capitalization_of("Azure")
        .then_whitespace()
        .then(Box::new(EitherPattern::new(vec![
            Box::new(SequencePattern::default().then_any_capitalization_of("DevOps")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Functions")),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("Cosmos")
                    .then_whitespace()
                    .then_any_capitalization_of("DB")
            ),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("SQL")
                    .then_whitespace()
                    .then_any_capitalization_of("Database")
            ),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("Kubernetes")
                    .then_whitespace()
                    .then_any_capitalization_of("Service")
            ),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("Virtual")
                    .then_whitespace()
                    .then_any_capitalization_of("Machines")
            ),
            Box::new(SequencePattern::default().then_any_capitalization_of("Monitor")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Storage")),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("Active")
                    .then_whitespace()
                    .then_any_capitalization_of("Directory")
            ),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("App")
                    .then_whitespace()
                    .then_any_capitalization_of("Service")
            ),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("Key")
                    .then_whitespace()
                    .then_any_capitalization_of("Vault")
            ),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("Cognitive")
                    .then_whitespace()
                    .then_any_capitalization_of("Services")
            ),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("Service")
                    .then_whitespace()
                    .then_any_capitalization_of("Bus")
            ),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("Event")
                    .then_whitespace()
                    .then_any_capitalization_of("Hub")
            )
        ]))),
    "When referring to Azure cloud services, make sure to treat them as proper nouns."
);

create_linter_for!(
    MicrosoftNames,
    SequencePattern::default()
        .then_any_capitalization_of("Microsoft")
        .then_whitespace()
        .then(Box::new(EitherPattern::new(vec![
            Box::new(SequencePattern::default().then_any_capitalization_of("Windows")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Office")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Teams")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Excel")),
            Box::new(SequencePattern::default().then_any_capitalization_of("PowerPoint")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Word")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Outlook")),
            Box::new(SequencePattern::default().then_any_capitalization_of("OneDrive")),
            Box::new(SequencePattern::default().then_any_capitalization_of("SharePoint")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Xbox")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Surface")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Edge")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Bing")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Dynamics")),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("Visual")
                    .then_whitespace()
                    .then_any_capitalization_of("Studio")
            )
        ]))),
    "When referring to Microsoft products and services, make sure to treat them as proper nouns."
);

create_linter_for!(
    AppleNames,
    SequencePattern::default()
        .then_any_capitalization_of("Apple")
        .then_whitespace()
        .then(Box::new(EitherPattern::new(vec![
            Box::new(SequencePattern::default().then_any_capitalization_of("iPhone")),
            Box::new(SequencePattern::default().then_any_capitalization_of("iPad")),
            Box::new(SequencePattern::default().then_any_capitalization_of("MacBook")),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("MacBook")
                    .then_whitespace()
                    .then_any_capitalization_of("Pro")
            ),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("MacBook")
                    .then_whitespace()
                    .then_any_capitalization_of("Air")
            ),
            Box::new(SequencePattern::default().then_any_capitalization_of("iMac")),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("Mac")
                    .then_whitespace()
                    .then_any_capitalization_of("Pro")
            ),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("Mac")
                    .then_whitespace()
                    .then_any_capitalization_of("Mini")
            ),
            Box::new(SequencePattern::default().then_any_capitalization_of("AirPods")),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("AirPods")
                    .then_whitespace()
                    .then_any_capitalization_of("Pro")
            ),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("AirPods")
                    .then_whitespace()
                    .then_any_capitalization_of("Max")
            ),
            Box::new(SequencePattern::default().then_any_capitalization_of("Watch")),
            Box::new(SequencePattern::default().then_any_capitalization_of("TV")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Music")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Arcade")),
            Box::new(SequencePattern::default().then_any_capitalization_of("iCloud")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Safari")),
            Box::new(SequencePattern::default().then_any_capitalization_of("HomeKit")),
            Box::new(SequencePattern::default().then_any_capitalization_of("CarPlay")),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("Vision")
                    .then_whitespace()
                    .then_any_capitalization_of("Pro")
            )
        ]))),
    "When referring to Apple products and services, make sure to treat them as proper nouns."
);

create_linter_for!(
    MetaNames,
    SequencePattern::default()
        .then_any_capitalization_of("Meta")
        .then_whitespace()
        .then(Box::new(EitherPattern::new(vec![
            Box::new(SequencePattern::default().then_any_capitalization_of("Oculus")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Portals")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Quest")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Gaming")),
            Box::new(SequencePattern::default().then_any_capitalization_of("Horizon")),
            Box::new(
                SequencePattern::default()
                    .then_any_capitalization_of("Reality")
                    .then_whitespace()
                    .then_any_capitalization_of("Labs")
            ),
        ]))),
    "When referring to Meta products and services, make sure to treat them as proper nouns."
);

#[cfg(test)]
mod tests {
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    use super::{Americas, MetaNames, MicrosoftNames, UnitedOrganizations};

    #[test]
    fn americas_lowercase() {
        assert_suggestion_result("south america", Americas::default(), "South America");
        assert_suggestion_result("north america", Americas::default(), "North America");
    }

    #[test]
    fn americas_uppercase() {
        assert_suggestion_result("SOUTH AMERICA", Americas::default(), "South America");
        assert_suggestion_result("NORTH AMERICA", Americas::default(), "North America");
    }

    #[test]
    fn americas_allow_correct() {
        assert_lint_count("South America", Americas::default(), 0);
        assert_lint_count("North America", Americas::default(), 0);
    }

    #[test]
    fn united_nations_uppercase() {
        assert_suggestion_result(
            "UNITED NATIONS",
            UnitedOrganizations::default(),
            "United Nations",
        );
    }

    #[test]
    fn united_arab_emirates_lowercase() {
        assert_suggestion_result(
            "UNITED ARAB EMIRATES",
            UnitedOrganizations::default(),
            "United Arab Emirates",
        );
    }

    #[test]
    fn united_nations_allow_correct() {
        assert_lint_count("United Nations", UnitedOrganizations::default(), 0);
    }

    #[test]
    fn meta_allow_correct() {
        assert_lint_count("Meta Quest", MetaNames::default(), 0);
    }

    #[test]
    fn microsoft_lowercase() {
        assert_suggestion_result(
            "microsoft visual studio",
            MicrosoftNames::default(),
            "Microsoft Visual Studio",
        );
    }

    #[test]
    fn microsoft_first_word_is_correct() {
        assert_suggestion_result(
            "Microsoft visual studio",
            MicrosoftNames::default(),
            "Microsoft Visual Studio",
        );
    }
}
