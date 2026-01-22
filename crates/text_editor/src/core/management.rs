use data_structures::binary_tree::{
    errors::{DeleteError, InsertError},
    rope::Rope,
};
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

pub fn save_to_path(path: &str, data: &Rope) -> io::Result<()> {
    let path = Path::new(path);

    let text = data.to_string();
    let lines: Vec<&str> = text.split('\n').collect();

    let mut file = File::create(path)?;

    for line in lines.iter() {
        writeln!(file, "{line}")?;
    }

    file.flush()?;
    Ok(())
}

pub fn open_from_path(path: &str) -> io::Result<TextBuffer> {
    let path = Path::new(path);
    let text = std::fs::read_to_string(path)?;
    Ok(TextBuffer::from_string(text.as_str()))
}

pub struct TextBuffer {
    pub data: Rope,
    pub state: Vec<usize>,
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            data: Rope::new(""),
            state: vec![0],
        }
    }

    pub fn from_string(text: &str) -> Self {
        let mut state = vec![0];

        for (i, c) in text.chars().enumerate() {
            if c == '\n' {
                state.push(i);
            }
        }

        Self {
            data: Rope::new(text),
            state,
        }
    }

    pub fn line_count(&self) -> usize {
        self.state.len()
    }

    pub fn line_range(&self, line_number: usize) -> (usize, usize) {
        let line_start = self.state[line_number];

        if self.line_count() - 1 == line_number {
            (
                line_start,
                self.data
                    .tree_size()
                    .expect("Failed to calculate tree size"),
            )
        } else {
            (line_start, *self.state.get(line_number + 1).unwrap())
        }
    }

    pub fn insert(mut self, text: &str, index: usize) -> Result<Self, InsertError> {
        let text_len = text.chars().count();
        self.data = self.data.insert(text, index)?;

        let state_len = self.line_count() - 1;
        let line_no = self.find_line_by_index(index);

        let (line_start_index, line_end_index) = self.line_range(line_no);
        let line_len = line_end_index.saturating_sub(line_start_index);

        if text.ends_with("\n") {
            if line_no < state_len {
                // split to two lines
                let new_line_length = line_len.saturating_sub(index);
                let old_line_length = line_len.saturating_sub(new_line_length) + text_len;

                self.state.insert(line_no + 1, new_line_length);
                self.state[line_no] = old_line_length;
            } else {
                // add new line at the end
                self.state.push(index);
            }
        } else {
            // update state
            for line_idx in line_no + 1..self.line_count() {
                self.state[line_idx] += text_len;
            }
        };

        Ok(self)
    }

    pub fn delete(mut self, index: usize, backspace: bool) -> Result<Self, DeleteError> {
        if backspace && index == 0 {
            return Ok(self);
        }

        let line_no = self.find_line_by_index(index);
        let (line_start_index, line_end_index) = self.line_range(line_no);
        let line_len_before = line_end_index.saturating_sub(line_start_index);
        let n_lines = self.line_count() - 1;
        let max_index = self.state[n_lines] + line_len_before;

        let del_start = if backspace {
            index.saturating_sub(1)
        } else {
            index
        };

        if index == max_index && !backspace {
            return Ok(self);
        }

        self.data = self.data.delete(del_start, 1)?;

        // update the state
        if line_no < n_lines {
            for i in line_no + 1..n_lines + 1 {
                self.state[i] -= 1;
            }
        }

        if backspace {
            match line_len_before.checked_sub(1) {
                None => {
                    self.state.remove(line_no);
                }
                Some(_) => {
                    if index - 1 == line_start_index && line_no != 0 {
                        // join with line above
                        self.state.remove(line_no);
                    }
                }
            }
        } else if index == line_end_index && line_no + 1 < n_lines {
            // join with line below
            self.state.remove(line_no + 1);
        };

        Ok(self)
    }

    fn find_line_by_index(&self, index: usize) -> usize {
        for (i, start_index) in self.state.iter().enumerate() {
            let start = *start_index;

            if start >= index {
                return i.saturating_sub(1);
            }
        }

        self.line_count() - 1
    }

    pub fn calculate_columns_in_line(&self, line_no: usize) -> usize {
        let (line_start, line_end) = self.line_range(line_no);
        line_end.saturating_sub(line_start + 1)
    }
}
#[derive(Default)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
    pub index: usize,
    desired_column: usize,
}

impl Cursor {
    pub fn new(line: usize, index: usize, column: usize) -> Self {
        Self {
            line,
            column,
            index,
            desired_column: column,
        }
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
            self.line = self.line.saturating_sub(1);
            self.column = self.columns_in_line(text_buffer, self.line);

            if self.line != 0 {
                self.column = self.column.saturating_sub(1);
            }
        }

        self.index = self.index.saturating_sub(1);
        self.desired_column = self.column;
    }

    pub fn move_inline_right(&mut self, text_buffer: &TextBuffer) {
        let mut cols = self.columns_in_line(text_buffer, self.line);

        if self.line != 0 {
            cols = cols.saturating_sub(1);
        }

        if self.column < cols {
            self.column += 1;
            self.index += 1;
        } else {
            self.line = (self.line + 1).min(text_buffer.line_count() - 1);
            let (line_start_index, _) = text_buffer.line_range(self.line);

            self.column = 0;
            self.index = line_start_index + 1;
        }

        self.desired_column = self.column;
    }

    pub fn move_line_down(&mut self, text_buffer: &TextBuffer) {
        if self.line + 1 > text_buffer.line_count() - 1 {
            return;
        }
        self.line += 1;
        let (line_start_index, _) = text_buffer.line_range(self.line);

        let mut cols = self.columns_in_line(text_buffer, self.line);

        if self.line != 0 {
            cols = cols.saturating_sub(1)
        };

        self.column = cols.min(self.desired_column);
        self.index = self.column + line_start_index + 1;
    }

    pub fn move_line_up(&mut self, text_buffer: &TextBuffer) {
        if self.line > 0 {
            self.line -= 1;

            let (line_start_index, _) = text_buffer.line_range(self.line);
            let mut cols = self.columns_in_line(text_buffer, self.line);

            if self.line != 0 {
                cols = cols.saturating_sub(1)
            };

            self.column = cols.min(self.desired_column);

            self.index = line_start_index + self.column;

            if self.line != 0 {
                self.index += 1;
            }
        }
    }

    pub fn move_to_new_row_after_backspace(
        &mut self,
        starting_line: usize,
        starting_column: usize,
        text_buffer: &TextBuffer,
    ) {
        // joining two rows, calculating correct new position
        let line_now = text_buffer.find_line_by_index(self.index);
        let columns_now = text_buffer.calculate_columns_in_line(line_now);

        let column = columns_now.saturating_sub(starting_column);
        let line = starting_line.saturating_sub(1);

        if column > 0 {
            self.move_inline_left(text_buffer);
        };
        
        self.column = column;
        self.line = line;
    }

    fn columns_in_line(&self, text_buffer: &TextBuffer, line_no: usize) -> usize {
        let (line_start_index, line_end_index) = text_buffer.line_range(line_no);

        if line_end_index < line_start_index {
            panic!(
                "{}",
                format!(
                    "start: {}, end: {}, line: {}",
                    line_start_index, line_end_index, line_no
                )
            )
        }

        line_end_index
            .checked_sub(line_start_index)
            .unwrap_or_else(|| {
                panic!(
                    "Couldn't subtract end({}) - start({}) index at line {}",
                    line_end_index, line_start_index, line_no
                )
            })
    }
}
