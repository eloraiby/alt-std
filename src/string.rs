use crate::vec::*;
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

pub trait ToString {
    fn toString(&self) -> String;
}

impl ToString for &str {
    fn toString(&self) -> String { String::from(*self) }
}

macro_rules! impl_signed {
    ($t:ident) => {
        impl ToString for $t {
            fn toString(&self) -> String {
                let mut v = Vec::<u8>::new();
                let mut s = String::new();
                let mut current = *self;
                let sign = current < 0;
                if sign { current = -current }
                loop {
                    let u = current % 10;
                    v.pushBack((u + '0' as Self) as u8);
                    current /= 10;
                    if current == 0 {
                        if sign { v.pushBack('-' as u8); }
                        for i in 0..v.len() {
                            s.add(v[v.len() - i - 1]);
                        }
                        return s;
                    }
                }
            }
        }
    };
}

macro_rules! impl_unsigned {
    ($t:ident) => {
        impl ToString for $t {
            fn toString(&self) -> String {
                let mut v = Vec::<u8>::new();
                let mut s = String::new();
                let mut current = *self;
                loop {
                    let u = current % 10;
                    v.pushBack((u + '0' as Self) as u8);
                    current /= 10;
                    if current == 0 {
                        for i in 0..v.len() {
                            s.add(v[v.len() - i - 1]);
                        }
                        return s;
                    }
                }
            }
        }
    };
}

impl ToString for char {
    fn toString(&self) -> String {
        let mut s = String::new();
        s.add(*self as u8);
        s
    }
}

impl ToString for bool {
    fn toString(&self) -> String {
        let tf = if *self { "true" } else { "fasle" };
        String::from(tf)
    }
}

impl ToString for String {
    fn toString(&self) -> String {
        self.clone()
    }
}

impl_signed!(i8);
impl_signed!(i16);
impl_signed!(i32);
impl_signed!(i64);

impl_unsigned!(u8);
impl_unsigned!(u16);
impl_unsigned!(u32);
impl_unsigned!(u64);

impl_unsigned!(usize);

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
