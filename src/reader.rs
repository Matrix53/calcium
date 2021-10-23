use std::collections::LinkedList;

pub struct Reader {
    buffer: LinkedList<char>,
}

impl Reader {
    pub fn new(str: &String) -> Reader {
        let mut reader = Reader {
            buffer: LinkedList::new(),
        };
        reader.buffer.extend(str.chars());
        reader
    }
    pub fn getc(&mut self) -> char {
        self.buffer.pop_front().unwrap()
    }
    pub fn ungetc(&mut self, chr: &char) {
        self.buffer.push_front(chr.clone());
    }
    pub fn has_next(&mut self) -> bool {
        !self.buffer.is_empty()
    }
}
