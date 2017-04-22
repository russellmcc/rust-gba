#![feature(compiler_builtins_lib, allocator, const_fn, cfg_target_has_atomic, lang_items, linkage)]
#![allocator]
#![no_std]
use core::ptr;
extern crate compiler_builtins;

extern crate volatile_register;
use volatile_register::WO;

extern crate gba_hw;

extern crate linked_list_allocator;
use linked_list_allocator::*;

extern "C" {
    static __eheap_start: usize;
    static __eheap_end: usize;
}

static mut EHEAP: Heap = Heap::empty();

pub unsafe fn init_heap() {
    EHEAP.init(
        (&__eheap_start as *const usize) as usize,
        ((&__eheap_end as *const usize) as usize) -
        ((&__eheap_start as *const usize) as usize));
}

extern crate gba_bios;

#[cfg(target_has_atomic = "ptr")]
compile_error!("Something's gone wrong!  Arm7tdmi has no atomics");

#[cfg(not(target_arch = "arm"))]
compile_error!("This crate is specific to arm");

#[no_mangle]
pub unsafe fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    EHEAP.allocate_first_fit(size, align).unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub unsafe fn __rust_deallocate(ptr: *mut u8, size: usize, align: usize) {
    EHEAP.deallocate(ptr, size, align)
}

extern "C" {
    pub fn __rust_usable_size(size: usize, align: usize) -> usize;
    pub fn __rust_reallocate(ptr: *mut u8, size: usize, new_size: usize, align: usize) -> *mut u8;
    pub fn __rust_reallocate_inplace(ptr: *mut u8,
                                     size: usize,
                                     new_size: usize,
                                     align: usize)
                                     -> usize;
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang="panic_fmt"] pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> ! { loop { } }

#[no_mangle]
pub fn __aeabi_unwind_cpp_pr0() {}

#[no_mangle]
pub fn __aeabi_unwind_cpp_pr1() {}

#[allow(non_snake_case)]
#[no_mangle]
pub fn _Unwind_Resume() {}

extern "C" {
    static mut __data_start: u8;
    static __data_end: u8;
    static __data_lma: u8;

    static mut __iwram_start: u8;
    static __iwram_end: u8;
    static __iwram_lma: u8;

    static mut __ewram_start: u8;
    static __ewram_end: u8;
    static __ewram_lma: u8;

    fn __usr_irq_handler();
}

#[allow(improper_ctypes)]
extern "C" {
    fn main() -> !;
}

unsafe fn load_section(start: &mut u8, end: &u8, lma: &u8) {
    let len_bytes = (end as *const u8 as usize) - (start as *const u8 as usize);
    gba_bios::fast_copy(
        core::slice::from_raw_parts(lma as *const u8, len_bytes),
        core::slice::from_raw_parts_mut(start as *mut u8, len_bytes));
}

#[no_mangle]
pub unsafe fn _start() -> ! {
    let usr_irq_ptr = 0x03007FFC as *mut usize;
    (*usr_irq_ptr) = __usr_irq_handler as usize;
    load_section(&mut __ewram_start, &__ewram_end, &__ewram_lma);
    load_section(&mut __iwram_start, &__iwram_end, &__iwram_lma);
    load_section(&mut __data_start, &__data_end, &__data_lma);
    main()
}

#[link_section=".iwram"]
#[no_mangle]
pub unsafe fn _irq_handler() {
    handle_interrupts()
}

#[linkage = "weak"]
#[no_mangle]
pub fn handle_interrupts() {
    let handled_interrupts = gba_hw::interrupts::SourceSet::all();
    unsafe {
        gba_hw::interrupts::irq_acknowledge().write(handled_interrupts);
        gba_hw::interrupts::irq_acknowledge_bios().write(handled_interrupts);
    }
}
