use data_structures::binary_tree::rope::Rope;

pub(crate) struct TextBuffer {
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

pub(crate) struct Cursor {
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) index: usize,
}

impl Cursor {
    pub(crate) fn move_inline_left(&mut self) {
        self.line -= 1;
    }

    pub(crate) fn move__inline_right(&mut self) {
        self.line += 1;
    }

    pub(crate) fn move_line_down(&mut self) {
        self.line -= 1;
    }

    pub(crate) fn move__line_up(&mut self) {
        self.line += 1;
    }
}
