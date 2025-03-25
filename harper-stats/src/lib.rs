mod record;
mod summary;

use std::io::{self, Read, Write};

pub use record::Record;
pub use record::RecordKind;
pub use summary::Summary;

/// A collection of logged statistics for the various Harper frontends.
pub struct Stats {
    pub records: Vec<Record>,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    /// Count the number of each kind of lint applied.
    pub fn summarize(&self) -> Summary {
        let mut summary = Summary::new();

        for record in &self.records {
            match record.kind {
                RecordKind::Lint(lint_kind) => summary.inc_lint_count(lint_kind),
            }
        }

        summary
    }

    pub fn write_csv(&self, w: &mut impl Write) -> io::Result<()> {
        let mut writer = csv::WriterBuilder::new().has_headers(false).from_writer(w);

        for record in &self.records {
            writer.serialize(record)?;
        }

        Ok(())
    }

    pub fn read_csv(r: &mut impl Read) -> io::Result<Self> {
        let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(r);
        let mut records = Vec::new();

        for result in reader.deserialize() {
            let record: Record = result?;

            records.push(record);
        }

        Ok(Self { records })
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}
