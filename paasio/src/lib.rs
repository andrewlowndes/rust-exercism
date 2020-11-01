use std::io::{Read, Result, Write};

pub struct ReadStats<R: Read> {
    reader: R,
    byte_count: usize,
    read_count: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(reader: R) -> ReadStats<R> {
        ReadStats {
            reader,
            byte_count: 0,
            read_count: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.byte_count
    }

    pub fn reads(&self) -> usize {
        self.read_count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.reader.read(buf);

        if let Ok(num_bytes) = result {
            self.byte_count += num_bytes;
            self.read_count += 1;
        }

        result
    }
}

pub struct WriteStats<W> {
    writer: W,
    byte_count: usize,
    write_count: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(writer: W) -> WriteStats<W> {
        WriteStats {
            writer,
            byte_count: 0,
            write_count: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.byte_count
    }

    pub fn writes(&self) -> usize {
        self.write_count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let result = self.writer.write(buf);

        if let Ok(num_bytes) = result {
            self.byte_count += num_bytes;
            self.write_count += 1;
        }

        result
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
