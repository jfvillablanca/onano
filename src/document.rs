use crate::Row;
use std::{
    fs,
    io::{BufRead, Error},
};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
}

impl Document {
    #[allow(clippy::missing_errors_doc)]
    pub fn open(file_name: &str) -> Result<Self, Error> {
        let contents = fs::read(file_name)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value.unwrap_or_default()));
        }

        Ok(Self {
            rows,
            file_name: Some(file_name.to_string()),
        })
    }

    #[must_use]
    pub fn get_row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
