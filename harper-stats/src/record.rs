use harper_core::{
    linting::{Lint, LintGroupConfig, LintKind},
    Document, FatStringToken,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct Record {
    pub kind: RecordKind,
    /// Recorded as seconds from the Unix Epoch
    pub when: i64,
    pub uuid: Uuid,
}

impl Record {
    /// Record a new instance at the current system time.
    pub fn now(kind: RecordKind) -> Self {
        Self {
            kind,
            when: chrono::Utc::now().timestamp(),
            uuid: Uuid::new_v4(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub enum RecordKind {
    Lint {
        kind: LintKind,
        context: Vec<FatStringToken>,
    },
    LintConfigUpdate(LintGroupConfig),
}

impl RecordKind {
    pub fn from_lint(lint: &Lint, doc: &Document) -> Self {
        Self::Lint {
            kind: lint.lint_kind,
            context: doc
                .fat_tokens_intersecting(lint.span)
                .into_iter()
                .map(|t| t.into())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use harper_core::{
        linting::{LintGroupConfig, LintKind},
        Document,
    };
    use quickcheck::Arbitrary;

    use super::{Record, RecordKind};

    fn arbitrary_lintconfig(g: &mut quickcheck::Gen) -> LintGroupConfig {
        let mut config = LintGroupConfig::default();

        for _ in 0..g.size() {
            config.set_rule_enabled(String::arbitrary(g), bool::arbitrary(g));
        }

        config
    }

    impl Arbitrary for RecordKind {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let lcu = Self::LintConfigUpdate(arbitrary_lintconfig(g));

            let context = Document::new_plain_english_curated(&String::arbitrary(g))
                .fat_string_tokens()
                .collect();

            let kind = *g
                .choose(&[
                    LintKind::Spelling,
                    LintKind::Capitalization,
                    LintKind::Style,
                    LintKind::Formatting,
                    LintKind::Repetition,
                    LintKind::Enhancement,
                    LintKind::Readability,
                    LintKind::WordChoice,
                    LintKind::Miscellaneous,
                ])
                .unwrap();

            g.choose(&[RecordKind::Lint { kind, context }, lcu])
                .unwrap()
                .clone()
        }
    }

    impl Arbitrary for Record {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Record {
                kind: RecordKind::arbitrary(g),
                when: i64::arbitrary(g),
                uuid: uuid::Builder::from_u128(u128::arbitrary(g)).into_uuid(),
            }
        }
    }
}
