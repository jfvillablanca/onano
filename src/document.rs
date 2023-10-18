use crate::Row;
use std::{
    fs,
    io::{BufRead, Error},
};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    #[allow(clippy::missing_errors_doc)]
    pub fn open(filename: &str) -> Result<Self, Error> {
        let contents = fs::read(filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value.unwrap_or_default()));
        }

        Ok(Self { rows })
    }

    #[must_use]
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
