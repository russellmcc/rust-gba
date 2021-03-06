#![feature(alloc)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate gba_rt;
extern crate gba_bios;
extern crate gba_hw;

use gba_hw::{ReadWrite, ReadOnly};


#[no_mangle]
pub fn main() -> ! {
    unsafe {
        gba_rt::init_heap();

        let ie_reg = 0x4000200 as *mut ReadWrite<u16>;
        let if_reg = 0x4000202 as *mut ReadWrite<u16>;
        let ime_reg = 0x4000208 as *mut ReadWrite<u16>;
//        let dispstat_reg = 0x4000004 as *const RW<u16>;
        let keycnt_reg = 0x4000132 as *mut ReadWrite<u16>;
        (*if_reg).write(1 << 12);
        (*ie_reg).write(1 << 12);
        (*ime_reg).write(1);
        (*keycnt_reg).write(1 << 14 | 1 << 3);
        let display_control = gba_hw::video::display_control();
        let p_screen = 0x6000000 as *mut ReadWrite<u16>;
        let keyinput_reg = 0x4000130 as *const ReadOnly<u16>;
        display_control.write(
            gba_hw::video::DisplayControlWrite::default()
                .set_video_mode(3)
                .set_display_layers(gba_hw::video::BG2));
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