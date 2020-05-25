use core::*;

#[link(name = "c")]
extern "C"
{
	pub static mut stdin: *const libc::FILE;

	pub static mut stdout: *const libc::FILE;

    pub static mut stderr: *const libc::FILE;
}

pub fn printStrings<'a>(arr: &[&'a str]) {
    for s in arr {
        unsafe { libc::printf("%.*s\0".as_bytes().as_ptr() as *const i8, s.len(), s.as_bytes().as_ptr())};
    }
}

#[macro_export]
macro_rules! printf {
    () => {{}};
    ($($arg:tt),+) => {{
       ::alt_std::io::printStrings(&[$($arg.toString().toStr(),)+])
    }};
}

#[macro_export]
macro_rules! printfn {
    () => {{}};
    ($($arg:tt),+) => {{
       ::alt_std::io::printStrings(&[$($arg.toString().toStr(),)+ "\n"])
    }};
}
