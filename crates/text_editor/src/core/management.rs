use std::cmp::Ordering;
use data_structures::binary_tree::{errors::InsertError, rope::Rope};


pub struct TextBuffer {
    pub data: Rope,
    pub state: Vec<usize>
}

impl TextBuffer {
    pub fn new() -> Self {
        Self { data: Rope::new(""), state: vec![0] }
    }

    pub fn from_string(text: &str) -> Self {
        let mut state = vec![0];

        for (i, c) in text.chars().enumerate() {
            if c == '\n' {
                state.push(i);
            }
        }

        Self { data: Rope::new(text), state }
    }

    pub fn line_count(&self) -> usize {
        self.state.len()
    }

    pub fn line_range(&self, line_number: usize) -> (usize, usize) {
        let line_start = self.state[line_number];

        if self.line_count() - 1 == line_number {
           return (line_start, self.data.tree_size().expect("Failed to calculate tree size"));
        } else {
            return (line_start, *self.state.get(line_number+1).unwrap());
        }
    }

    pub fn insert(mut self, text: &str, index: usize) -> Result<Self, InsertError> {
        let text_len = text.chars().count();
        self.data = self.data.insert(text, index)?;

        let state_len = self.line_count() - 1;
        let line_no = self.find_line_by_index(index);

        let (line_start_index, line_end_index) = self.line_range(line_no);
        let line_len = line_end_index.checked_sub(line_start_index).unwrap();

        if text.ends_with("\n") {

            if line_no < state_len {
                // split to two lines
                let new_line_length = line_len.checked_sub(index).unwrap();
                let old_line_length = line_len.checked_sub(new_line_length).unwrap() + text_len;
                
                self.state.insert(line_no+1, new_line_length);
                self.state[line_no] = old_line_length;

            } else {
                // add new line at the end
                self.state.push(index);
            }

        } else {
            // update state
            for line_idx in line_no+1..self.line_count() {
                self.state[line_idx] += text_len;
            }
        };

        Ok(self)
    }

    fn find_line_by_index(&self, index: usize) -> usize {

        for (i, start_index) in self.state.iter().enumerate() {
            let start = *start_index;

            if start >= index {
                return i.checked_sub(1).unwrap_or(0);
            }
        };

        self.line_count() - 1
    }

}

pub struct Cursor {
    pub line: usize,
    pub column: usize,
    pub index: usize,
    desired_column: usize
}

impl Cursor {
    pub fn new(line: usize, index: usize, column: usize) -> Self {
        Self { line, column, index, desired_column: column }
    }

    pub fn default() -> Self {
        Self { line: 0, column: 0, index: 0, desired_column: 0 }
    }


    pub fn move_by_char(&mut self, c: char) {
        self.index += 1;

        if c == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        self.desired_column = self.column;
    }

    pub fn move_inline_left(&mut self, text_buffer: &TextBuffer) {

        if self.column != 0 {
            self.column -= 1;

        } else {
            self.line = self.line.checked_sub(1).unwrap_or(0);
            self.column = self.columns_in_line(text_buffer, self.line);
        }

        self.index = self.index.checked_sub(1).unwrap_or(0) ;
        self.desired_column = self.column;
    }

    pub fn move_inline_right(&mut self, text_buffer: &TextBuffer) {
        let cols = self.columns_in_line(text_buffer, self.line).checked_sub(1).unwrap_or(0);

        if self.column < cols {
            self.column += 1;
            self.index += 1;
        } else {
            self.line = (self.line+1).min(text_buffer.line_count()-1);
            let (line_start_index, _) = text_buffer.line_range(self.line);

            self.column = 0;
            self.index = line_start_index+1;
        }
        
        self.desired_column = self.column;
    }

    pub fn move_line_down(&mut self, text_buffer: &TextBuffer) {
        if self.line + 1 > text_buffer.line_count() - 1 {
            return;
        }
        self.line += 1;
        let (line_start_index, _) = text_buffer.line_range(self.line);

        let cols = self.columns_in_line(text_buffer, self.line);

        self.column = cols.min(self.desired_column);
        self.index = self.column + line_start_index;
        // if self.line != 0 {
        //     self.index -= 1;
        // }
    }

    pub fn move_line_up(&mut self, text_buffer: &TextBuffer) {
        if self.line > 0 {
            self.line -= 1;

            let (line_start_index, _) = text_buffer.line_range(self.line);
            let cols = self.columns_in_line(text_buffer, self.line);
            self.column = cols.min(self.desired_column);

            self.index = line_start_index + self.column;

            if self.line != 0 {
                self.index += 1;
            }
        }
    }

    fn columns_in_line(&self, text_buffer: &TextBuffer, line_no: usize) -> usize {
        let (line_start_index, line_end_index) = text_buffer.line_range(line_no);

        if line_end_index < line_start_index {
            panic!(
                "{}",
                format!("start: {}, end: {}, line: {}", line_start_index, line_end_index, line_no)
            )
        }

        line_end_index.checked_sub(line_start_index).expect(
            format!("Couldn't subtract end({}) - start({}) index at line {}", line_end_index, line_start_index, line_no).as_str()
        )
    }
}
