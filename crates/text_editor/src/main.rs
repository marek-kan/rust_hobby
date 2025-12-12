use data_structures::binary_tree::rope::Rope;
use std::error::Error;
use std::{fs, io};
use text_editor::core::actions::Actions;
use text_editor::core::management::Cursor;

fn read_stdin(buffer: &mut String) -> Result<(), Box<dyn Error>> {
    buffer.clear();

    io::stdin().read_line(buffer)?;
    buffer.pop();

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let mut text_buffer = Rope::new("");
    let mut cursor = Cursor {
        line: 0,
        column: 0,
        index: 0,
    };

    loop {
        read_stdin(&mut buffer)?;

        let action = match buffer.as_str() {
            "ML" => Actions::MoveLeft,
            "MR" => Actions::MoveRight,
            "MU" => Actions::MoveUp,
            "MD" => Actions::MoveDown,
            "DEL" => Actions::Delete,
            "RE" => Actions::Backspace,
            "SAVE" => Actions::Save,
            "OPEN" => Actions::Open,
            _ => Actions::Insert,
        };

        match action {
            Actions::Insert => {
                println!("Before insert cursor at: {}", &cursor.index);
                text_buffer = text_buffer
                    .insert(&buffer, cursor.index)
                    .expect("Failed to insert stdin buffer to text buffer");

                cursor.index += buffer.len();
            }

            Actions::MoveLeft => {
                println!("Moving left from {}", &cursor.index);
                cursor.move_inline_left();
            }

            Actions::MoveRight => {
                println!("Moving right from {}", &cursor.index);
                cursor.move_inline_right();
            }

            Actions::Backspace => {
                text_buffer = text_buffer
                    .delete(cursor.index - 1, cursor.index)
                    .expect("Failed to execute backspace");

                cursor.index -= 1;
            }

            Actions::Delete => {
                if let Some(size) = text_buffer.tree_size()
                    && cursor.index < size
                {
                    text_buffer = text_buffer.delete(cursor.index, cursor.index + 1).unwrap();
                }
            }

            Actions::Save => {
                println!("Filename:");

                read_stdin(&mut buffer)?;

                let text = text_buffer.to_string();

                fs::write(buffer, text)?;
                break;
            }

            Actions::Open => {
                println!("Filename:");

                read_stdin(&mut buffer)?;

                let text = fs::read_to_string(&buffer)?;

                text_buffer = Rope::new(&text);
                cursor.index = text.len();
            }

            _ => todo!(),
        }

        println!("Full text:\n{}", text_buffer);
        println!("Cursor at: {}", &cursor.index);
    }

    Ok(())
}
