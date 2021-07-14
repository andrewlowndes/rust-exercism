use std::{
    borrow::Borrow,
    io::{Read, Write},
    iter::Cycle,
    slice::Iter,
};

pub trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}

#[derive(Clone)]
pub struct Xorcism<'a> {
    key_iter: Cycle<Iter<'a, u8>>,
}

impl<'a> Xorcism<'a> {
    pub fn new<Key>(key: &Key) -> Xorcism
    where
        Key: AsRef<[u8]> + ?Sized,
    {
        Xorcism {
            key_iter: key.as_ref().iter().cycle(),
        }
    }

    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut()
            .for_each(move |item| *item ^= self.key_iter.next().unwrap())
    }

    pub fn munge<'func, 'data, Data>(
        &'func mut self,
        data: Data,
    ) -> impl Iterator<Item = u8> + Captures<'a> + 'data
    where
        Data: IntoIterator + 'data,
        Data::Item: Borrow<u8>,
        Data::IntoIter: 'a,
        'func: 'data,
    {
        data.into_iter()
            .map(move |item| *item.borrow() ^ self.key_iter.next().unwrap())
    }

    pub fn reader<R: Read>(self, reader: R) -> XorcismRead<'a, R> {
        XorcismRead {
            xorism: self,
            reader,
        }
    }

    pub fn writer<W: Write>(self, writer: W) -> XorcismWrite<'a, W> {
        XorcismWrite {
            xorism: self,
            writer,
        }
    }
}

pub struct XorcismRead<'a, T: Read> {
    xorism: Xorcism<'a>,
    reader: T,
}

impl<T: Read> Read for XorcismRead<'_, T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let buf_len = self.reader.read(buf)?;
        self.xorism.munge_in_place(&mut buf[..buf_len]);
        Ok(buf_len)
    }
}

pub struct XorcismWrite<'a, T: Write> {
    xorism: Xorcism<'a>,
    writer: T,
}

impl<T: Write> Write for XorcismWrite<'_, T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut items = buf.to_vec();
        self.xorism.munge_in_place(&mut items);
        self.writer.write(&items)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
