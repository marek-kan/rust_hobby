use crate::core::management::{Cursor, TextBuffer, Viewport};
use std::io;
use std::io::Write;

pub use crossterm::{
    Command, cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, Clear, ClearType},
};

pub fn move_cursor(line: usize, column: usize) -> cursor::MoveTo {
    cursor::MoveTo(column as u16, line as u16)
}

pub fn render<W: Write>(
    out: &mut W,
    buf: &TextBuffer,
    cursor: &Cursor,
    viewport: &mut Viewport,
) -> io::Result<()> {
    let text = buf.data.to_string();
    let lines: Vec<&str> = text.split('\n').collect();

    let (_, sc_height) = terminal::size()?; // 1 indexed 
    let screen_rows = sc_height.saturating_sub(1) as usize;
    viewport.adjust_viewport(cursor, screen_rows);

    let end = (viewport.row_offset + screen_rows + 1).min(buf.line_count());

    queue!(out, Clear(ClearType::All))?;

    for (row, line) in lines[viewport.row_offset..end].iter().enumerate() {
        queue!(
            out,
            cursor::MoveTo(0, row as u16),
            Clear(ClearType::CurrentLine),
            style::Print(line)
        )?;
    }

    let viewport_adjusted_line = cursor.line.saturating_sub(viewport.row_offset);

    queue!(out, move_cursor(viewport_adjusted_line, cursor.column))?;

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
