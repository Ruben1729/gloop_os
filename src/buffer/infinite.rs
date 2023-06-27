use std::vec::Vec;
use std::vec;
use std::string::String;
use crate::buffer::Buffer;

pub struct InfiniteBuffer {
    lines: Vec<String>,
    pub(crate) cursor: (usize, usize)
}

impl InfiniteBuffer {
    pub fn new() -> Self {
        InfiniteBuffer {
            lines: vec![String::new()],
            cursor: (0,0)
        }
    }
}

impl Buffer for InfiniteBuffer {
    fn write(&mut self, byte: char) {
        if let Some(line) = self.lines.get_mut(self.cursor.0.clone()) {
            line.insert(self.cursor.1.clone(), byte);
        }
    }

    fn write_str(&mut self, new_line: String) {
        if let Some(line) = self.lines.get_mut(self.cursor.0.clone()) {
            line.insert_str(self.cursor.1.clone(), &*new_line);
        }
    }

    fn write_line(&mut self) {
        self.lines.push(String::new());
    }

    fn read(&self) -> Option<char> {
        if let Some(line) = self.lines.get(self.cursor.0.clone()) {
            return line.chars().nth(self.cursor.1.clone());
        }
        None
    }

    fn read_line(&self) -> Option<String> {
        if let Some(line) = self.lines.get(self.cursor.0.clone()) {
            return Some(line.clone());
        }
        None
    }

    fn delete(&mut self) {
        if let Some(line) = self.lines.get_mut(self.cursor.0.clone()) {
            line.remove(self.cursor.1.clone());
        }
    }

    fn char_at(&self, cursor: (usize, usize)) -> Option<char> {
        if let Some(line) = self.lines.get(cursor.0) {
            return line.chars().nth(cursor.1);
        }

        None
    }

    fn move_cursor(&mut self, dx: usize, dy: usize) {
        self.cursor.0 += dx;
        self.cursor.1 += dy;
    }

    fn move_cursor_to(&mut self, x: usize, y: usize) {
        self.cursor.0 = x;
        self.cursor.1 = y;
    }
}