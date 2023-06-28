use crate::buffer::infinite::InfiniteBuffer;

pub struct Bim {
    pub buffer: InfiniteBuffer,
    pub cursor: (usize, usize),
    pub mode: BimMode
}

impl Bim {
}

pub enum BimMode {
    NORMAL,
    COMMAND,
    INSERT,
    REPLACE
}
