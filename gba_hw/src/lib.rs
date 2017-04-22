#![no_std]

#[macro_use]
extern crate bitflags;
extern crate volatile_register;
extern crate vcell;

macro_rules! register {
    ($(#[$attr:meta])* pub $name:ident : $type:ty => $addr:expr) => {
        $(#[$attr])*
        #[inline(always)]
        pub unsafe fn $name() -> &'static $type {
            &*($addr as *const $type)
        }
    };
}

pub mod interrupts;
pub mod video;
