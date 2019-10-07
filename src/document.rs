use crate::Row;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn open() -> Self {
        let mut rows = Vec::new();
        rows.push(Row::from("Hello, World!"));
        Self { rows }
    }
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
}
