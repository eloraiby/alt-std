use core::*;
use crate::string::*;
use crate::ctypes::*;
use crate::stream::*;

#[link(name = "c")]
extern "C"
{
	pub static mut stdin: *mut libc::FILE;

	pub static mut stdout: *mut libc::FILE;

    pub static mut stderr: *mut libc::FILE;
}

pub fn fprintStrings<'a>(f: *mut libc::FILE, arr: &[&'a str]) {
    for s in arr {
        unsafe { libc::fprintf(f,"%.*s\0".as_bytes().as_ptr() as *const i8, s.len(), s.as_bytes().as_ptr())};
    }
}

#[macro_export]
macro_rules! printf {
    () => {{}};
    ($($arg:expr),+) => {{
       unsafe { ::alt_std::io::fprintStrings(::alt_std::io::stdout, &[$(($arg).toString().toStr(),)+]) }
    }};
}

#[macro_export]
macro_rules! printfn {
    () => {{}};
    ($($arg:expr),+) => {{
       unsafe { ::alt_std::io::fprintStrings(::alt_std::io::stdout, &[$(($arg).toString().toStr(),)+ "\n"]) }
    }};
}

#[macro_export]
macro_rules! error {
    () => {{}};
    ($($arg:expr),+) => {{
       unsafe { ::alt_std::io::fprintStrings(::alt_std::io::stderr, &[$(($arg).toString().toStr(),)+]) }
    }};
}

#[macro_export]
macro_rules! errorn {
    () => {{}};
    ($($arg:expr),+) => {{
       unsafe { ::alt_std::io::fprintStrings(::alt_std::io::stderr, &[$(($arg).toString().toStr(),)+ "\n"]) }
    }};
}

////////////////////////////////////////////////////////////////////////////////
pub struct FileWriter {
    file    : *mut ::libc::FILE,
}

impl FileWriter {
    pub fn create(fname: &str) -> Result<Self, ()> {
        let mut sname = String::from(fname);
        sname.add('\0' as u8);
        let f = unsafe { ::libc::fopen(sname.toStr().as_bytes().as_ptr() as *const i8, "wb\0".as_bytes().as_ptr() as *const i8) };
        if f as * const _ == ::core::ptr::null() {
            Result::Err(())
        } else {
            Result::Ok(Self { file: f })
        }
    }

    pub fn write(&mut self, bytes: &[u8]) -> Result<usize, ()> {
        let count = unsafe {  ::libc::fwrite(bytes.as_ptr() as *const c_void, 1, bytes.len(), self.file) };
        if count < bytes.len() {
            if unsafe { ::libc::ferror(self.file) } != 0 {
                Result::Err(())
            } else {
                Result::Ok(count)
            }
        } else {
            Result::Ok(count)
        }
    }

    pub fn tell(&self) -> usize { unsafe { ::libc::ftell(self.file) as usize } }
    pub fn size(&self) -> usize {
        unsafe {
            let curr = self.tell();
            ::libc::fseek(self.file, 0, ::libc::SEEK_END);
            let size = self.tell();
            ::libc::fseek(self.file, curr as i64, ::libc::SEEK_SET);
            size
        }
    }
}

impl Drop for FileWriter {
    fn drop(&mut self) {
        unsafe { ::libc::fclose(self.file) };
    }
}

impl Stream for FileWriter {
    fn tell(&self) -> usize { self.tell() }
    fn size(&self) -> usize { self.size() }
}

impl StreamSeek for FileWriter {
    fn seek(&mut self, pos: usize) -> Result<usize, ()> {
        unsafe { ::libc::fseek(self.file, pos as i64, ::libc::SEEK_SET) };
        Result::Ok(self.tell())
    }
}

impl StreamWriter for FileWriter {
    fn write(&mut self, buff: &[u8]) -> Result<usize, ()> {
        self.write(buff)
    }
}

////////////////////////////////////////////////////////////////////////////////
pub struct FileReader {
    file    : *mut ::libc::FILE,
}

impl FileReader {
    pub fn open(fname: &str) -> Result<Self, ()> {
        let mut sname = String::from(fname);
        sname.add('\0' as u8);
        let f = unsafe { ::libc::fopen(sname.toStr().as_bytes().as_ptr() as *const i8, "rb\0".as_bytes().as_ptr() as *const i8) };
        if f as * const _ == ::core::ptr::null() {
            Result::Err(())
        } else {
            Result::Ok(Self { file: f })
        }
    }

    pub fn tell(&self) -> usize { unsafe { ::libc::ftell(self.file) as usize } }
    pub fn size(&self) -> usize {
        unsafe {
            let curr = self.tell();
            ::libc::fseek(self.file, 0, ::libc::SEEK_END);
            let size = self.tell();
            ::libc::fseek(self.file, curr as i64, ::libc::SEEK_SET);
            size
        }
    }

    pub fn read(&mut self, buff: &mut [u8]) -> Result<usize, ()> {
        let count = unsafe {  ::libc::fread(buff.as_mut_ptr() as *mut c_void, 1, buff.len(), self.file) };
        if count < buff.len() {
            if unsafe { ::libc::ferror(self.file) } != 0 {
                Result::Err(())
            } else {
                Result::Ok(count)
            }
        } else {
            Result::Ok(count)
        }
    }
}


impl Drop for FileReader {
    fn drop(&mut self) {
        unsafe { ::libc::fclose(self.file) };
    }
}

impl Stream for FileReader {
    fn tell(&self) -> usize { self.tell() }
    fn size(&self) -> usize { self.size() }
}

impl StreamSeek for FileReader {
    fn seek(&mut self, pos: usize) -> Result<usize, ()> {
        unsafe { ::libc::fseek(self.file, pos as i64, ::libc::SEEK_SET) };
        Result::Ok(self.tell())
    }
}

impl StreamReader for FileReader {
    fn read(&mut self, buff: &mut [u8]) -> Result<usize, ()> {
        self.read(buff)
    }

    fn isEOF(&self) -> bool {
        if unsafe { ::libc::feof(self.file) } != 0 {
            true
        } else {
            false
        }
    }
}

pub trait StreamSeek : Stream {
    fn seek(&mut self, cursor: usize) -> Result<usize, ()>;
}

////////////////////////////////////////////////////////////////////////////////

pub struct File {
}

impl File {

    pub fn exist(fname: &str) -> bool {
        let f = FileReader::open(fname);
        match f {
            Ok(_) => true,
            _ => false,
        }
    }

    pub fn remove(fname: &str) -> Result<(), ()> {
        let mut sname = String::from(fname);
        sname.add('\0' as u8);
        let f = unsafe { ::libc::remove(sname.toStr().as_bytes().as_ptr() as *const i8) };
        if f < 0 {
            Err(())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testCreateReadRemoveFile() {
        {
            let mut f = FileWriter::create("test.txt");
            let s = "Hello File";
            match &mut f {
                Ok(f) => {
                    let res = f.write(s.as_bytes());
                    assert!(res.unwrap() == s.as_bytes().len());
                    assert!(File::exist("test.txt"));
                },
                _ => panic!("couldn't create file!")
            }
        }

        {
            let mut f = FileReader::open("test.txt");
            let s = "Hello File";
            match &mut f {
                Ok(f) => {
                    assert!(f.size() == 10);
                    let mut buff : [u8; 10] = [0; 10];
                    let res = f.read(&mut buff);
                    assert!(res.unwrap() == s.as_bytes().len());
                    assert!(str::from_utf8(&buff).unwrap() == s);
                },
                _ => panic!("couldn't open file!")
            }
        }

        {
            File::remove("test.txt").unwrap();
        }
    }
}