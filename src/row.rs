use std::cmp;

pub struct Row {
    string: String,
}

impl From<String> for Row {
    fn from(string: String) -> Self {
        Self { string }
    }
}

impl Row {
    #[must_use]
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        self.string.get(start..end).unwrap_or_default().to_string()
    }
}