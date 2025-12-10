use data_structures::binary_tree::rope::Rope;
use std::io;
use text_editor::core::actions::Actions;
use text_editor::core::management::Cursor;

fn main() -> Result<(), io::Error> {
    let mut buffer = String::new();
    let mut text_buffer = Rope::new("");
    let mut cursor = Cursor {
        line: 0,
        column: 0,
        index: 0,
    };

    while true {
        io::stdin().read_line(&mut buffer)?;
        buffer.pop(); // remove the trailing '\n' from pressing Enter

        let action = match buffer.as_str() {
            "ML" => Actions::MoveLeft,
            "MR" => Actions::MoveRight,
            "MU" => Actions::MoveUp,
            "MD" => Actions::MoveDown,
            "DEL" => Actions::Delete,
            "RE" => Actions::Backspace,
            _ => Actions::Insert,
        };

        match action {
            Actions::Insert => {
                println!("Before insert cursor at: {}", &cursor.index);
                text_buffer = text_buffer
                    .insert(&buffer, cursor.index)
                    .expect("Failed to insert stdin buffer to text buffer");

                cursor.index += buffer.len();
                println!("After insert cursor at: {}", &cursor.index);
            }
            Actions::MoveLeft => {
                println!("Moving left from {}", &cursor.index);
                cursor.move_inline_left();
            }
            Actions::MoveRight => {
                println!("Moving right from {}", &cursor.index);
                cursor.move_inline_right();
            }
            _ => todo!(),
        }

        buffer.clear();

        println!("Full text:\n{}", text_buffer.to_string());
    }

    Ok(())
}
