mod lint_record;

use std::io::{self, Write};

pub use lint_record::LintRecord;

pub struct Stats {
    /// A record of the lints the user has applied.
    lints_applied: Vec<LintRecord>,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            lints_applied: Vec::new(),
        }
    }

    pub fn lint_applied(&mut self, record: LintRecord) {
        self.lints_applied.push(record);
    }

    pub fn write_csv(&self, w: &mut impl Write) -> io::Result<()> {
        let mut writer = csv::WriterBuilder::new().has_headers(false).from_writer(w);

        for record in &self.lints_applied {
            writer.serialize(record)?;
        }

        Ok(())
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}
