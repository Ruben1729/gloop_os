use std::vec::Vec;
use std::string::String;
use crate::buffer::Buffer;

pub struct InfiniteBuffer {
    lines: Vec<String>,
    pub(crate) cursor: (usize, usize)
}

impl InfiniteBuffer {
    pub fn new(x: usize, y: usize) -> Self {
        InfiniteBuffer {
            lines: Vec::new(),
            cursor: (x,y)
        }
    }
}

impl Buffer for InfiniteBuffer {
    fn write(&mut self, byte: char) {
        if let Some(line) = self.lines.get_mut(self.cursor.0.clone()) {
            line.insert(self.cursor.1.clone(), byte);
        } else {
            let current_len = self.lines.len();
            for _ in current_len..self.cursor.0.clone() {
                self.lines.push(String::new());
            }

            self.lines.push(String::from(byte));
        }

        self.cursor.1 += 1;
    }

    fn write_str(&mut self, new_line: String) {
        self.cursor.1 += new_line.len();

        if let Some(line) = self.lines.get_mut(self.cursor.0.clone()) {
            line.insert_str(self.cursor.1.clone(), &*new_line);
        } else {
            let current_len = self.lines.len();
            for _ in current_len..self.cursor.0.clone() {
                self.lines.push(String::new());
            }

            self.lines.push(new_line);
        }
    }

    fn write_line(&mut self) {
        self.lines.push(String::new());
        self.cursor.1 = 0;  // column
        self.cursor.0 += 1; // row
    }

    fn read(&self, cursor: (usize, usize)) -> Option<char> {
        if let Some(line) = self.lines.get(cursor.0.clone()) {
            return line.chars().nth(cursor.1.clone());
        }
        None
    }

    fn read_line(&self, idx: usize) -> Option<String> {
        if let Some(line) = self.lines.get(idx) {
            return Some(line.clone());
        }
        None
    }

    fn read_cursor(&self) -> Option<char> {
        if let Some(line) = self.lines.get(self.cursor.0.clone()) {
            return line.chars().nth(self.cursor.1.clone());
        }
        None
    }

    fn read_cursor_line(&self) -> Option<String> {
        if let Some(line) = self.lines.get(self.cursor.0.clone()) {
            return Some(line.clone())
        }
        None
    }

    fn delete_char(&mut self) {
        if let Some(line) = self.lines.get_mut(self.cursor.0.clone()) {
            if line.len() > 0 {
                line.remove(self.cursor.1.clone() - 1);
                self.cursor.1 -= 1;
            }
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

    fn clear(&mut self) {
        self.lines.clear();
        self.cursor = (0,0);
    }
}