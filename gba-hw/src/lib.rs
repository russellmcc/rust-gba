#![no_std]

#[macro_use]
extern crate bitflags;
extern crate vcell;

mod registers;
pub use registers::*;

macro_rules! register {
    ($(#[$attr:meta])* pub $name:ident : $type:ty => $addr:expr) => {
        $(#[$attr])*
        #[inline(always)]
        pub unsafe fn $name() -> &'static mut $type {
            &mut *($addr as *mut $type)
        }
    };
}

pub mod interrupts;
pub mod video;
