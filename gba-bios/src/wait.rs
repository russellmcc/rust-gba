pub use gba_hw::interrupts::SourceSet;

/// Halts CPU until any enabled interrupt is hit.
///
/// Note that `IE` must be non-zero, or else this will never return!
/// Also, at least one interrupt must be enabled by a peripheral.
///
/// Note that for some sources, disabling interrupts via the `CPSR` or `IME`
/// will not prevent this function from returning.  I haven't done
/// extensive testing, but at least the keypad interrupts need
/// `IME` enabled.
///
/// If `IME` or `CPSR` is enabled, the interrupt
/// handler will be called before this function returns.
///
/// CPU is in a somewhat low-powered state during execution of this
/// function.
#[inline(always)]
pub fn wait_for_any_enabled_interrupt() {
    unsafe {asm!("swi 0x06");}
}

/// Puts CPU into super-low power state until an interrupt occurs.
///
/// Only the following sources can wake up from stop:
///
///  * Keypad
///  * Cartridge interrupts
///  * SIO
///
/// Note that at least one of these sources must be enabled in `IE` and
/// its peripheral control register or else this will never return!
///
/// Note that this function works even if you disable interrupts via
/// the `CPSR` or `IME` registers.
///
/// Also note that when the interrupt occurs, the interrupt will not
/// actually fire.
#[inline(always)]
pub fn stop_and_wait_for_any_enabled_interrupt() {
    unsafe {asm!("swi 0x03");}
}


/// Waits for one of a specific set of interrupts to be
/// fired.
///
/// # Arguments
///
/// * `interrupts`: the set of interrupts to wait for.
///   the function will return whenever any of these
///   is fired.
///
/// Note that the interrupts you are waiting for must
/// already be enabled, both in `IE` and the peripheral.
///
/// This function will return immediately if an interrupt
/// of the requested type has happened since the last wait.
///
/// This function is unsafe because it will turn on the
/// `IME` register.
#[inline(always)]
pub unsafe fn wait_for_interrupt(interrupts: SourceSet) {
    let interrupts_bits = interrupts.bits();
    let mode = 0u16;
    asm!("swi 0x04"
         :
         : "{r0}"(mode)
         , "{r1}"(interrupts_bits));
}

/// Waits for one of a specific set of interrupts to be
/// fired.
///
/// # Arguments
///
/// * `interrupts`: the set of interrupts to wait for.
///   the function will return whenever any of these
///   is fired.
///
/// Note that the interrupts you are waiting for must
/// already be enabled, both in `IE` and the peripheral.
///
/// This function will only return when a brand new
/// interrupt from the specified step occurs.  Pending
/// interrupts will be ignored.
///
/// This function is unsafe because it will turn on the
/// `IME` register.
#[inline(always)]
pub unsafe fn wait_for_new_interrupt(interrupts: SourceSet) {
    let interrupts_bits = interrupts.bits();
    let mode = 1u16;
    asm!("swi 0x04"
         :
         : "{r0}"(mode)
         , "{r1}"(interrupts_bits));
}

/// Waits for a `VBLANK` interrupt to occur
///
/// Note that the `VBLANK` interrupt must be enabled,
/// both in the `IE` register and the `DISPCNT` register.
///
/// This function will only return on the next VBLANK.
///
/// This function is unsafe because it will turn on the
/// `IME` register.
#[inline(always)]
pub unsafe fn wait_for_new_vblank() {
    asm!("swi 0x05":::"r0 r1");
}

