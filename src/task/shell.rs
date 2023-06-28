use crate::{print, println};
use crate::shell::parse_command;
use crate::vga_buffer::WRITER;

pub fn update_shell(character: char) {
    if character as u8 == 8 {
        WRITER.lock().delete_byte()
    } else if character as u8 == 10 {
        let command_str = WRITER.lock().get_line();

        println!();
        let result = parse_command(command_str);

        match result {
            Ok(_) => {
                print!("> ");
            }
            Err(e) => {
                print!("ERROR: {}\n> ", e);
            }
        }

    } else {
        print!("{}", character);
    }
}