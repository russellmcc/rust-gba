#[inline(always)]
pub fn soft_reset() -> ! {
    unsafe { asm!("swi 0"); }
    unreachable!();
}