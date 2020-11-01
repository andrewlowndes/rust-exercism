#![feature(slice_fill)]

pub struct CircularBuffer<T> {
    write_head: usize,
    read_head: usize,
    entries: Vec<Option<T>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            entries: vec![None; capacity],
            write_head: 0,
            read_head: 0,
        }
    }

    fn step_write_head(&mut self) {
        self.write_head = (self.write_head + 1) % self.entries.len();
    }

    fn step_read_head(&mut self) {
        self.read_head = (self.read_head + 1) % self.entries.len();
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.entries[self.write_head].is_some() {
            return Err(Error::FullBuffer);
        }

        self.entries[self.write_head].replace(_element);
        self.step_write_head();

        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.entries.is_empty() {
            return Err(Error::EmptyBuffer);
        }

        if let Some(result) = self.entries[self.read_head].take() {
            self.step_read_head();
            Ok(result)
        } else {
            Err(Error::EmptyBuffer)
        }
    }

    pub fn clear(&mut self) {
        self.entries.fill(None);
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.read_head == self.write_head {
            self.step_read_head();
        }

        self.entries[self.write_head].replace(_element);
        self.step_write_head();
    }
}
