#![feature(alloc)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate gba_rt;
extern crate gba_bios;
extern crate gba_hw;

extern crate volatile_register;
use volatile_register::{RW, RO};


#[no_mangle]
pub fn main() -> ! {
    unsafe {
        gba_rt::init_heap();

        let ie_reg = 0x4000200 as *const RW<u16>;
        let if_reg = 0x4000202 as *const RW<u16>;
        let ime_reg = 0x4000208 as *const RW<u16>;
//        let dispstat_reg = 0x4000004 as *const RW<u16>;
        let keycnt_reg = 0x4000132 as *const RW<u16>;
        (*if_reg).write(1 << 12);
        (*ie_reg).write(1 << 12);
        (*ime_reg).write(1);
        (*keycnt_reg).write(1 << 14 | 1 << 3);
        let video_mode = 0x4000000 as *const RW<u32>;
        let p_screen = 0x6000000 as *const RW<u16>;
        let keyinput_reg = 0x4000130 as *const RO<u16>;
        (*video_mode).write(0x403);
        for y in 0..160 {
            for x in 0..240 {
                (*p_screen.offset(x + y*240)).write(31 << 5);
            }
        }

        while (*keyinput_reg).read() & 1 << 4 == 1 << 4 {}

        for y in 0..160 {
            for x in 0..240 {
                (*p_screen.offset(x + y*240)).write(31);
            }
        }

        gba_bios::wait_for_new_interrupt(gba_hw::interrupts::KEYPAD);

        for y in 0..160 {
            for x in 0..240 {
                (*p_screen.offset(x + y*240)).write(31 << 10);
            }
        }
    }
    loop {
    }
}