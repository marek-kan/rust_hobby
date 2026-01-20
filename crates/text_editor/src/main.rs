use data_structures::binary_tree::rope::Rope;
use std::io::Write;
use std::{fs, io};
use text_editor::core::management::{Cursor, TextBuffer};

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

fn debug_state(label: &str, cursor: &Cursor, buf: &TextBuffer) {
    let text = buf.data.to_string();
    println!("--- {label} ---");
    println!("cursor = ({}, {})", cursor.line, cursor.column);
    println!("cursor.index() = {}", cursor.index);
    println!("rope.len() = {}", text.len());
    println!("State: {:?}", buf.state);
    println!("rope = {:?}", text);
    println!();
}

fn render<W: Write>(out: &mut W, rope: &Rope, cursor: &Cursor) -> io::Result<()> {
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

    queue!(out, move_cursor(&cursor))?;

    out.flush()?;
    Ok(())
}

// fn main() -> io::Result<()> {
//     let mut buf = TextBuffer::new();
//     let mut cursor = Cursor::default();

//     // for ch in vec!['a', '\n', '1', '2', '3'] {
//     //     buf = buf.insert(&ch.to_string(), cursor.index).unwrap();
//     //     cursor.move_by_char(ch);
//     // };
    
//     // debug_state("initial", &cursor, &buf);

//     // cursor.move_line_up(&buf);

//     // debug_state("after move up", &cursor, &buf);

//     // cursor.move_line_down(&buf);

//     // debug_state("after move down", &cursor, &buf);

//     // cursor.move_line_up(&buf);
//     // // buf = buf.insert(&"A", cursor.index).unwrap();
//     // // cursor.move_by_char('A');
//     // for ch in vec!['A', 'B', 'C', 'D', 'E'] {
//     //     buf = buf.insert(&ch.to_string(), cursor.index).unwrap();
//     //     cursor.move_by_char(ch);
//     // };
//     // cursor.move_line_down(&buf);

//     // debug_state("after first insert", &cursor, &buf);

//     // Ok(())
//     for ch in "123\nabc\nABC\n098".chars() {
//         buf = buf.insert(&ch.to_string(), cursor.index).unwrap();
//         cursor.move_by_char(ch);
//     };

//     debug_state("init", &cursor, &buf);

//     cursor.move_line_up(&buf);
//     cursor.move_line_up(&buf);

//     // for i in 0..3 {
//     //     cursor.move_inline_left(&buf);
//     // }

//     buf = buf.delete(cursor.index, fal    NewLine,se).unwrap();

//     // buf = buf.insert("a", cursor.index).unwrap();
//     // cursor.move_by_char('a');

//     debug_state("after delete", &cursor, &buf);

//     // cursor.move_line_up(&buf);

//     // buf = buf.insert("b", cursor.index).unwrap();
//     // cursor.move_by_char('b');

//     // debug_state("b", &cursor, &buf);

//     Ok(())

// }



fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut buf = TextBuffer::new();
    let mut cursor = Cursor::default();

    execute!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    loop {
        render(&mut stdout, &buf.data, &cursor)?;

        let event = event::read()?;
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                continue;
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
                        buf = buf.delete(cursor.index, true).unwrap();
                        cursor.move_inline_left(&buf);
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
