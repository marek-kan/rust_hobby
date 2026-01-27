use std::io;
use text_editor::core::management::{Cursor, TextBuffer, Viewport, open_from_path, save_to_path};
use text_editor::core::ui::{prompt_user, read_line_from_user, render};

use crossterm::event::{KeyEventKind, KeyEventState, KeyModifiers};
pub use crossterm::{
    Command, cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, Clear, ClearType},
};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut buf = TextBuffer::default();
    let mut cursor = Cursor::default();
    let mut viewport = Viewport::default();

    execute!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    loop {
        render(&mut stdout, &buf, &cursor, &mut viewport)?;

        let event = event::read()?;
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key {
                KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                } => {
                    prompt_user(
                        &mut stdout,
                        "**Only backspace allowed**\nPlease, enter save filepath:",
                    )?;

                    if let Some(path) = read_line_from_user(&mut stdout)? {
                        save_to_path(&path, &buf.data)?;

                        terminal::disable_raw_mode()?;
                        execute!(stdout, terminal::LeaveAlternateScreen)?;

                        return Ok(());
                    } else {
                        render(&mut stdout, &buf, &cursor, &mut viewport)?;
                    }
                }
                KeyEvent {
                    code: KeyCode::Char('o'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                } => {
                    prompt_user(
                        &mut stdout,
                        "**Only backspace allowed**\nPlease, enter filepath:",
                    )?;

                    if let Some(path) = read_line_from_user(&mut stdout)? {
                        buf = open_from_path(&path)?;
                        render(&mut stdout, &buf, &cursor, &mut viewport)?;
                        continue;
                    } else {
                        render(&mut stdout, &buf, &cursor, &mut viewport)?;
                    }
                }
                _ => {}
            }

            match key.code {
                KeyCode::Char(c) => {
                    buf = buf.insert(&c.to_string(), cursor.index).unwrap();
                    cursor.move_by_char(c);
                }

                KeyCode::Enter => {
                    buf = buf.insert("\n", cursor.index).unwrap();
                    cursor.move_by_char('\n');
                }

                KeyCode::Backspace => {
                    // bc move_inline left moves eventho at 0 index nothing is erased -> jumps to end of line
                    if cursor.index != 0 {
                        let line_before = cursor.line;
                        let columns_before = buf.calculate_columns_in_line(line_before);

                        buf = buf.delete(cursor.index, true).unwrap();

                        if cursor.column != 0 {
                            cursor.move_inline_left(&buf);
                        } else if columns_before == 0 {
                            // special case for empty line
                            cursor.move_inline_left(&buf);
                        } else {
                            // joining two lines
                            cursor.move_to_new_row_after_backspace(
                                line_before,
                                columns_before,
                                &buf,
                            );
                        }
                    }
                }

                KeyCode::Delete => {
                    buf = buf.delete(cursor.index, false).unwrap();
                }

                KeyCode::Left => cursor.move_inline_left(&buf),
                KeyCode::Right => cursor.move_inline_right(&buf),
                KeyCode::Up => cursor.move_line_up(&buf),
                KeyCode::Down => cursor.move_line_down(&buf),

                KeyCode::Esc => break,

                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, terminal::LeaveAlternateScreen)?;
    Ok(())
}
