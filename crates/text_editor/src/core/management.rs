use std::cmp::Ordering;
use data_structures::binary_tree::{errors::InsertError, rope::Rope};


pub struct TextBuffer {
    pub data: Rope,
    state: Vec<usize>
}

impl TextBuffer {
    pub fn new() -> Self {
        Self { data: Rope::new(""), state: vec![0] }
    }

    pub fn from_string(text: &str) -> Self {
        let mut state = vec![0];

        for (i, c) in text.char_indices() {
            if c == '\n' {
                state.push(i + 1);
            }
        }

        Self { data: Rope::new(text), state }
    }

    pub fn line_count(&self) -> usize {
        self.state.len()
    }

    pub fn line_range(&self, line_number: usize) -> (usize, usize) {
        let line_start = self.state[line_number];
        let tree_size = self.data.tree_size().expect("Failed to calculate tree size");

        let line_end = self.state.get(line_number+1).unwrap_or(
            &tree_size
        );

        (line_start, *line_end)
    }

    pub fn insert(mut self, text: &str, index: usize) -> Result<Self, InsertError> {
        let text_len = text.len();

        // This should always find a line and index, if not, something went terribly wrong and we can't recover now.
        let line_no = self.find_line_by_index(index);
        let (line_start_index, line_end_index) = self.line_range(line_no);

        let line_len = line_end_index.checked_sub(line_start_index).unwrap_or(0) ;
        let state_len = self.state.len();
        
        self.data = self.data.insert(text, index)?;

        if text.ends_with("\n") {
            let new_line_length = line_len.checked_sub(index).unwrap_or(0);
            let old_line_length = line_len.checked_sub(new_line_length).unwrap_or(0) + text_len;

            if line_no < state_len {
                // split to two lines
                self.state.insert(line_no+1, new_line_length);
            } else {
                // add new line at the end
                self.state.push(new_line_length);
            }

            self.state[line_no] = old_line_length;
        } else {
            // update state
            for line_idx in line_no+1..self.state.len() {
                self.state[line_idx] += text_len;
            }
        };

        Ok(self)
    }

    fn find_line_by_index(&self, index: usize) -> usize {

        for (i, start_index) in self.state.iter().enumerate() {
            let start = *start_index;

            if start >= index {
                return i;
            }
        };

        self.line_count() - 1
    }

}

pub struct Cursor {
    pub line: usize,
    pub column: usize,
    pub index: usize,
}

impl Cursor {
    pub fn move_by_char(&mut self, c: char) {
        self.index += c.len_utf8();

        if c == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }

    pub fn move_inline_left(&mut self, text_buffer: &TextBuffer) {
        let line_no = text_buffer.find_line_by_index(self.index);//;.expect("Failed to localize line no.");
        let (line_start_index, line_end_index) = text_buffer.line_range(line_no);

        if self.column - 1 >= line_start_index {
            self.column -= 1;
        } else {
            let new_line = line_no.checked_sub(1).unwrap_or(0);
            let (new_start, new_end) = text_buffer.line_range(new_line);

            self.line -= 1;
            self.column = new_end;
        }
    }

    pub fn move_inline_right(&mut self, text_buffer: &TextBuffer) {
        let line_no = text_buffer.find_line_by_index(self.index);//.expect("Failed to localize line no.");
        let (line_start_index, line_end_index) = text_buffer.line_range(line_no);

        if self.column + 1 <= line_end_index {
            self.column += 1;
        } else {
            let new_line = (line_no+1).min(text_buffer.line_count());
            let (new_start, new_end) = text_buffer.line_range(new_line);

            self.line += 1;
            self.column = new_start;
        }
    }

    pub fn move_line_down(&mut self, text_buffer: &TextBuffer) {
        if self.line + 1 > text_buffer.line_count() {
            return;
        }

        self.line += 1;
    }

    pub fn move_line_up(&mut self, text_buffer: &TextBuffer) {
        if self.line > 0 {
            self.line -= 1;
        }
    }
}
