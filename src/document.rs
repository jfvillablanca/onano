use crate::{editor::Position, Row};
use std::{
    fs::{self, File},
    io::{BufRead, Error, Write},
};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
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
            dirty: false,
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

    pub fn insert(&mut self, at: &Position, c: char) {
        let end_of_document = self.len();
        if at.y > end_of_document {
            return;
        }
        self.dirty = true;
        if c == '\n' {
            self.insert_newline(at);
            return;
        }
        if at.y == end_of_document {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
        }
    }

    fn insert_newline(&mut self, at: &Position) {
        let end_of_document = self.len();
        if at.y == end_of_document {
            self.rows.push(Row::default());
            return;
        }
        let new_row = self.rows.get_mut(at.y).unwrap().split(at.x);
        self.rows.insert(at.y + 1, new_row);
    }

    pub fn delete(&mut self, at: &Position) {
        let end_of_document = self.len();
        let end_of_row = self.rows.get_mut(at.y).unwrap().len();
        if at.y >= end_of_document {
            return;
        }
        self.dirty = true;
        if at.x == end_of_row && at.y < end_of_document - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next_row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x);
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }
        Ok(())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}
