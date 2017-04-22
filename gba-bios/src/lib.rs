#![feature(asm)]
#![no_std]

#[cfg(not(target_arch = "arm"))]
compile_error!("This crate is specific to the gameboy advance");

extern crate gba_hw;

mod arith;
pub use arith::*;

mod reset;
pub use reset::*;

mod mem;
pub use mem::*;

mod wait;
pub use wait::*;