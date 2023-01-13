use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct Row {
    pub string: String,
    pub len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            string: String::from(slice),
            len: 0,
        };
        row.update_len();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push_str(" ");
            } else {
                result.push_str(grapheme);
            }
        }
        result
    }
    pub fn backspace(&mut self, x: usize) {
        self.string.replace_range(x.saturating_sub(1)..x, "");
        self.update_len();
    }

    pub fn insert(&mut self, x: usize, char: char) {
        if char == '\n' || char =='\r' {
            self.string.insert(x, '\n');
            self.string.insert(x, '\r');    
        }
        self.string.insert(x, char);
        self.update_len();
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count();
    }
}
