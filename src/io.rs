use core::*;

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