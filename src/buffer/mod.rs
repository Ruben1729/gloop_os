use std::string::String;

pub mod infinite;

pub trait Buffer {
    fn write(&mut self, byte: char);
    fn write_str(&mut self, new_line: String);
    fn write_line(&mut self);

    fn read(&self, cursor:(usize, usize)) -> Option<char>;
    fn read_line(&self, idx: usize) -> Option<String>;

    fn read_cursor(&self) -> Option<char>;
    fn read_cursor_line(&self) -> Option<String>;

    fn delete_char(&mut self);

    fn char_at(&self, cursor: (usize, usize)) -> Option<char>;

    fn move_cursor(&mut self, dx: usize, dy: usize);
    fn move_cursor_to(&mut self, x: usize, y: usize);
    fn clear(&mut self);
}
