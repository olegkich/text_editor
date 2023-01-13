use crate::Row;
use std::fs;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
        })
    }

    // needs improvement, i was lazy
    pub fn write(&self) -> Option<Result<(), std::io::Error>> {
        let mut contents_to_write = String::new();

        for row in &self.rows {
            let str = &row.string;

            contents_to_write.push_str(str)
        }

        match &self.file_name {
            Some(path) => {
                match fs::write(path, contents_to_write) {
                    Ok(_) => return Some(Ok(())),
                    Err(x) => return Some(Err(x)),
                };
            }
            None => return None,
        }
    }
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn row_mut(&mut self, index: usize) -> Option<&mut Row> {
        self.rows.get_mut(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
