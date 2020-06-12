#![allow(non_snake_case, non_camel_case_types)]
#![no_std]

pub mod ctypes;
pub mod mem;
pub mod io;
pub mod vec;
pub mod string;
pub mod hash;
pub mod hashmap;
pub mod stream;

pub use string::*;
pub use vec::*;
pub use hashmap::*;
pub use ctypes::*;
pub use mem::*;


//
// TODO: This will break testing in dependant modules (comment until we can pass this information from cargo)
//
// #[cfg(not(test))]
// #[panic_handler]
// fn alt_std_panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {}
// }