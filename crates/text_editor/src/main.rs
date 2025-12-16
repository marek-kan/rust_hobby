use data_structures::binary_tree::rope::Rope;
use std::error::Error;
use std::io::Write;
use std::os::unix::thread;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;
use std::{fs, io};
use text_editor::core::actions::Actions;
use text_editor::core::management::Cursor;

use crossterm::event::KeyEventKind;
pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, Clear, ClearType},
    Command
};


fn move_cursor(cursor: &Cursor) -> cursor::MoveTo {
    cursor::MoveTo(cursor.column as u16, cursor.line as u16)
}

fn debug_state(label: &str, cursor: &Cursor, rope: &Rope) {
    let text = rope.to_string();
    println!("--- {label} ---");
    println!("cursor = ({}, {})", cursor.line, cursor.column);
    println!("cursor.index() = {}", cursor.index());
    println!("rope.len() = {}", text.len());
    println!("rope = {:?}", text);
    println!();
}

fn render<W: Write>(out: &mut W, rope: &Rope, cursor_pos: &Cursor) -> io::Result<()> {
    let text = rope.to_string();
    let lines: Vec<&str> = text.split('\n').collect();

    execute!(out, cursor::Hide, Clear(ClearType::All))?;

    for (row, line) in lines.iter().enumerate() {
        execute!(
            out,
            cursor::MoveTo(0, row as u16),
            Clear(ClearType::CurrentLine),
            style::Print(format!("{}/{}: {}", row,cursor_pos.index(),line))
        )?;
    }

    // execute!(out, Clear(ClearType::All), style::Print(text))?;

    execute!(
        out,
        cursor::MoveTo(cursor_pos.column as u16, cursor_pos.line as u16),
        cursor::Show
    )?;

    // out.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut rope = Rope::new("");
    let mut cursor = Cursor { line: 0, column: 0 };

    execute!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    loop {
        render(&mut stdout, &rope, &cursor)?;

        let event = event::read()?;
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char(c) => {
                    rope = rope.insert(&c.to_string(), cursor.index()).unwrap();
                    cursor.move_inline_right();
                }

                KeyCode::Enter => {
                    rope = rope.insert("\n", cursor.index()).unwrap();
                    cursor.move_line_down();
                    cursor.column = 0;
                }

                KeyCode::Left => cursor.move_inline_left(),
                KeyCode::Right => cursor.move_inline_right(),
                KeyCode::Up => cursor.move_line_up(),
                KeyCode::Down => cursor.move_line_down(),

                KeyCode::Esc => break,

                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, terminal::LeaveAlternateScreen)?;
    Ok(())
}


// fn read_stdin(buffer: &mut String) -> Result<(), Box<dyn Error>> {
//     buffer.clear();

//     io::stdin().read_line(buffer)?;
//     buffer.pop();

//     Ok(())
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     let mut buffer = String::new();
//     let mut text_buffer = Rope::new("");
//     let mut cursor = Cursor {
//         line: 0,
//         column: 0,
//         index: 0,
//     };

//     loop {
//         read_stdin(&mut buffer)?;

//         let action = match buffer.as_str() {
//             "ML" => Actions::MoveLeft,
//             "MR" => Actions::MoveRight,
//             "MU" => Actions::MoveUp,
//             "MD" => Actions::MoveDown,
//             "DEL" => Actions::Delete,
//             "RE" => Actions::Backspace,
//             "SAVE" => Actions::Save,
//             "OPEN" => Actions::Open,
//             _ => Actions::Insert,
//         };

//         match action {
//             Actions::Insert => {
//                 println!("Before insert cursor at: {}", &cursor.index);
//                 text_buffer = text_buffer
//                     .insert(&buffer, cursor.index)
//                     .expect("Failed to insert stdin buffer to text buffer");

//                 cursor.index += buffer.len();
//             }

//             Actions::MoveLeft => {
//                 println!("Moving left from {}", &cursor.index);
//                 cursor.move_inline_left();
//             }

//             Actions::MoveRight => {
//                 println!("Moving right from {}", &cursor.index);
//                 cursor.move_inline_right();
//             }

//             Actions::Backspace => {
//                 text_buffer = text_buffer
//                     .delete(cursor.index - 1, cursor.index)
//                     .expect("Failed to execute backspace");

//                 cursor.index -= 1;
//             }

//             Actions::Delete => {
//                 if let Some(size) = text_buffer.tree_size()
//                     && cursor.index < size
//                 {
//                     text_buffer = text_buffer.delete(cursor.index, cursor.index + 1).unwrap();
//                 }
//             }

//             Actions::Save => {
//                 println!("Filename:");

//                 read_stdin(&mut buffer)?;

//                 let text = text_buffer.to_string();

//                 fs::write(buffer, text)?;
//                 break;
//             }

//             Actions::Open => {
//                 println!("Filename:");

//                 read_stdin(&mut buffer)?;

//                 let text = fs::read_to_string(&buffer)?;

//                 text_buffer = Rope::new(&text);
//                 cursor.index = text.len();
//             }

//             _ => todo!(),
//         }

//         println!("Full text:\n{}", text_buffer);
//         println!("Cursor at: {}", &cursor.index);
//     }

//     Ok(())
// }
