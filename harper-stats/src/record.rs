use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

use harper_core::linting::LintKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub kind: RecordKind,
    /// Recorded as seconds from the Unix Epoch
    pub when: u64,
    pub uuid: Uuid,
}

impl Record {
    /// Record a new instance at the current system time.
    pub fn now(kind: RecordKind) -> Result<Self, SystemTimeError> {
        Ok(Self {
            kind,
            when: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            uuid: Uuid::new_v4(),
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RecordKind {
    Lint(LintKind),
}
