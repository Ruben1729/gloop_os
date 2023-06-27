use lazy_static::lazy_static;

use std::string::String;
use std::vec::Vec;
use std::vec;

use crate::file_system::*;

pub const COMMAND_ARG_MAX: usize = 100;
pub const COMMANDS_LEN_MAX: usize = 500;

pub enum ShellError {
    InsufficientArguments,
    TooManyArguments,
    CommandNotFound
}

lazy_static! {
    pub static ref SHELL_COMMANDS: Vec<(String, usize, fn(Vec<&str>) -> Result<(), &'static str>)> = vec![
        (String::from("cd"), 1, change_directory),
        (String::from("ls"), 0, list_inodes),
        (String::from("mkfile"), 1, create_file),
        (String::from("mkdir"), 1, create_folder)
    ];
}

pub fn parse_command(command_str: String) -> Result<(), &'static str> {
    let mut cmd: Vec<&str> = command_str.split(" ").collect();
    cmd.retain(|s| !s.is_empty());
    cmd.remove(0);
    let cmd_key: String = String::from(cmd.remove(0));

    for item in SHELL_COMMANDS.iter() {
        if item.0.eq(&cmd_key) {
            if cmd.len() < item.1 {
                return Err("Not enough arguments.");
            }

            return item.2(cmd);
        }
    }

    Err("Unable to find command.")
}
