use crate::vec::*;
use ::core::*;
use ::core::cmp::*;
use crate::hash::*;

#[repr(C)]
pub struct String {
    data    : Vec<u8>
}

impl String {
    pub fn new() -> Self { Self { data: Vec::new() } }
    pub fn from(s: &str) -> Self {
        let mut st = Self::new();
        for c in s.bytes() {
            st.data.pushBack(c);
        }
        st
    }

    pub fn toStr(&self) -> &str {
        ::core::str::from_utf8(self.data.asArray()).expect("Error getting string out")
    }

    pub fn add(&mut self, u: u8) {
        self.data.pushBack(u);
    }

    pub fn asArray(&self) -> &[u8] { self.data.asArray() }
}

pub trait Append<T> {
    fn append(&mut self, other: T);
}

impl Append<&str> for String {
    fn append(&mut self, s: &str) {
        for c in s.bytes() {
            self.data.pushBack(c);
        }
    }
}

impl Append<&String> for String {
    fn append(&mut self, s: &String) {
        for c in s.toStr().bytes() {
            self.data.pushBack(c);
        }
    }
}

impl PartialEq<String> for String {
    fn eq(&self, other: &Self) -> bool {
        let ls = self.data.len();
        let lo = other.data.len();
        if ls != lo { return false }
        for i in 0..self.data.len() {
            if self.data[i] != other.data[i] { return false }
        }
        true
    }
}

impl Eq for String {}

impl PartialEq<&str> for String {
    fn eq(&self, other: &&str) -> bool {
        let ob = (*other).as_bytes();
        let ls = self.data.len();
        let lo = ob.len();
        if ls != lo { return false }
        for i in 0..self.data.len() {
            if self.data[i] != ob[i] { return false }
        }
        true
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        String::from(self.toStr())
    }
}

impl fmt::Write for String {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.append(s);
        Ok(())
    }

    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.add(c as u8);
        Ok(())
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.toStr())
    }
}

impl Hash for String {
    fn hash(&self) -> usize {
        self.asArray().hash()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testConversion() {
        {
            let u : u32 = 12345;
            if String::from("12345") != u.toString() {
                panic!("fail {}", u.toString().toStr());
            }
        }

        {
            let i : i32 = -12345;
            if String::from("-12345") != i.toString() {
                panic!("fail {}", i.toString().toStr());
            }
        }
    }
}
