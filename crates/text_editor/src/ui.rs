use crate::core::management::Cursor;
use data_structures::binary_tree::rope::Rope;
use std::io;
use std::io::Write;

pub use crossterm::{
    Command, cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, Clear, ClearType},
};

pub fn move_cursor(cursor: &Cursor) -> cursor::MoveTo {
    cursor::MoveTo(cursor.column as u16, cursor.line as u16)
}

pub fn render<W: Write>(out: &mut W, rope: &Rope, cursor: &Cursor) -> io::Result<()> {
    let text = rope.to_string();
    let lines: Vec<&str> = text.split('\n').collect();

    queue!(out, Clear(ClearType::All))?;

    for (row, line) in lines.iter().enumerate() {
        queue!(
            out,
            cursor::MoveTo(0, row as u16),
            Clear(ClearType::CurrentLine),
            style::Print(line)
        )?;
    }

    queue!(out, move_cursor(cursor))?;

    out.flush()?;
    Ok(())
}

pub fn prompt_user<W: Write>(out: &mut W, prompt: &str) -> io::Result<()> {
    let lines: Vec<&str> = prompt.split('\n').collect();
    let mut final_row: u16 = 0;

    queue!(out, Clear(ClearType::All))?;
    for (row, line) in lines.iter().enumerate() {
        queue!(
            out,
            cursor::MoveTo(0, row as u16),
            Clear(ClearType::CurrentLine),
            style::Print(line)
        )?;
        final_row = row as u16;
    }

    queue!(out, cursor::MoveTo(0, final_row + 1))?;

    out.flush()?;
    Ok(())
}

pub fn read_line_from_user<W: Write>(stdout: &mut W) -> io::Result<Option<String>> {
    let mut input = String::new();

    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    input.push(c);
                    queue!(stdout, style::Print(c))?;
                    stdout.flush()?;
                }

                KeyCode::Backspace => {
                    if !input.is_empty() {
                        input.pop();
                        queue!(
                            stdout,
                            cursor::MoveLeft(1),
                            style::Print(" "),
                            cursor::MoveLeft(1)
                        )?;
                        stdout.flush()?;
                    }
                }

                KeyCode::Enter => {
                    return Ok(Some(input));
                }

                KeyCode::Esc => {
                    return Ok(None);
                }

                _ => {}
            }
        }
    }
}
