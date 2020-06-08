#![allow(non_snake_case, non_camel_case_types)]
#![no_std]

pub mod ctypes;
pub mod mem;
pub mod io;
pub mod vec;
pub mod string;
pub mod hashmap;

pub use string::*;
pub use vec::*;
pub use hashmap::*;
pub use ctypes::*;
pub use mem::*;


#[cfg(not(test))]
#[panic_handler]
fn alt_std_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}