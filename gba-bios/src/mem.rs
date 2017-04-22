/// Quickly copies memory, subject to a few conditions.
///
/// # Arguments
///
/// * `source` 4-byte aligned slice to copy from
/// * `dest` 4-byte aligned slice to copy to
///
/// # Caveats
///
/// Note that `source` and `dest` must have the same length,
/// and that this length *must* be a multiple of 32 bytes.
#[inline(always)]
pub fn fast_copy(source: &[u8], dest: &mut [u8]) {
    let src_ptr = source.as_ptr();
    let dst_ptr = dest.as_mut_ptr();
    let len_bytes = source.len();

    // Check pre-conditions in debug mode.
    debug_assert!(src_ptr as usize & 0x3 == 0);
    debug_assert!(dst_ptr as usize & 0x3 == 0);
    debug_assert!(source.len() == dest.len());
    debug_assert!(len_bytes & 31 == 0);

    let len_words = len_bytes >> 2;

    unsafe {
        asm!("swi 0x0c"
             :
             : "{r0}"(src_ptr)
             , "{r1}"(dst_ptr)
             , "{r2}"(len_words)
        );
    }
}

/// Quickly fills memory, subject to a few conditions.
///
/// # Arguments
///
/// * `val` value to fill memory with.
/// * `dest` 4-byte aligned slice to fill, must be a multiple of
///   32 bytes.
#[inline(always)]
pub fn fast_set(val: u32, dest: &mut [u8]) {
    let dst_ptr = dest.as_mut_ptr();
    let len_bytes = dest.len();

    // Check pre-conditions in debug mode.
    debug_assert!(dst_ptr as usize & 0x3 == 0);
    debug_assert!(len_bytes & 31 == 0);

    let len_words = len_bytes >> 2;

    unsafe {
        asm!("swi 0x0c"
             :
             : "{r0}"(val)
             , "{r1}"(dst_ptr)
             , "{r2}"(len_words)
        );
    }
}