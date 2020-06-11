use crate::vec::*;

pub trait Stream : Drop {
    /// get the current position
    fn tell(&self) -> usize;

    /// get the size of the stream
    fn size(&self) -> usize;

    /// close the stream
    fn close(&mut self);
}

pub trait StreamReader : Stream {
    fn read(&mut self, buff: &mut [u8]) -> Result<usize, ()>;
    fn isEOF(&self) -> bool;
}

pub trait StreamWriter : Stream {
    fn write(&mut self, buff: &[u8]) -> Result<usize, ()>;
}

pub trait StreamSeek : Stream {
    fn seek(&mut self, cursor: usize) -> Result<usize, ()>;
}

pub struct MemoryStreamWriter {
    data    : Vec<u8>,
}

impl MemoryStreamWriter {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn data(&self) -> &Vec<u8> { &self.data }
    pub fn dataMut(&mut self) -> &mut Vec<u8> { &mut self.data }
}

impl Drop for MemoryStreamWriter {
    fn drop(&mut self) {}
}

impl Stream for MemoryStreamWriter {
    fn tell(&self) -> usize { self.data.len() }
    fn size(&self) -> usize { self.data.len() }
    fn close(&mut self) {}
}

impl StreamWriter for MemoryStreamWriter {
    fn write(&mut self, buff: &[u8]) -> Result<usize, ()> {
        self.data.append(buff);
        Ok(buff.len())
    }
}

///////////////////////////////////////////////////////////////////////////////

pub struct MemoryStreamReader {
    data    : Vec<u8>,
    cursor  : usize,
}

impl MemoryStreamReader {
    pub fn from(src: &[u8]) -> Self {
        let mut v = Vec::new();
        v.append(src);
        Self { data : v, cursor: 0 }
    }
}

impl Stream for MemoryStreamReader {
    fn tell(&self) -> usize { self.cursor }
    fn size(&self) -> usize { self.data.len() }
    fn close(&mut self) {}
}

impl Drop for MemoryStreamReader {
    fn drop(&mut self) {}
}

impl StreamReader for MemoryStreamReader {
    fn isEOF(&self) -> bool { self.cursor == self.data.len() }
    fn read(&mut self, buff: &mut [u8]) -> Result<usize, ()> {
        let readLen =
            if buff.len() > self.data.len() - self.cursor {
                self.data.len() - self.cursor
            } else {
                buff.len()
            };
        for c in 0..readLen {
            buff[c] = self.data[self.cursor + c];
        }
        self.cursor += readLen;
        Ok(readLen)
    }
}

///////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testMemoryWriteStream() {
        let mut msw = MemoryStreamWriter::new();
        let string = "hello world".as_bytes();
        assert!(msw.write(string) == Result::Ok(11));
        assert!(msw.tell() == 11);
        assert!(msw.size() == 11);
        let data = msw.data().asArray();
        assert!(data.len() == 11);
        for i in 0..string.len() {
            assert!(data[i] == string[i]);
        }
    }

    #[test]
    fn testMemoryReadStream() {
        let mut msr = MemoryStreamReader::from("hello world".as_bytes());
        assert!(msr.tell() == 0);
        assert!(msr.size() == 11);
        let string = "hello world".as_bytes();
        let mut buff = [0u8; 11];
        assert!(msr.read(&mut buff) == Result::Ok(11));
        for i in 0..string.len() {
            assert!(buff[i] == string[i]);
        }
    }
}