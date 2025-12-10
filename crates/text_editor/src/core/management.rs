use data_structures::binary_tree::rope::Rope;

pub struct TextBuffer {
    data: Rope,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            data: Rope::new(""),
        }
    }

    pub fn from_string(text: &str) -> Self {
        Self {
            data: Rope::new(text),
        }
    }
}

pub struct Cursor {
    pub line: usize,
    pub column: usize,
    pub index: usize,
}

impl Cursor {
    pub fn move_inline_left(&mut self) {
        self.index -= 1;
    }

    pub fn move_inline_right(&mut self) {
        self.index += 1;
    }

    pub fn move_line_down(&mut self) {
        self.column += 1;
    }

    pub fn move_line_up(&mut self) {
        self.column -= 1;
    }
}
