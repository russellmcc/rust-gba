/// Simultaneously calculates quotient, remainder, and the
/// absolute value of the quotient.
///
/// # Examples
///
/// ```
/// use gba_bios::div_modulo_absdiv;
/// assert_eq!((-123, -3, 123), div_modulo_absdiv(-1234, 10));
/// ```
#[inline(always)]
pub fn div_modulo_absdiv(num: i32, denom: i32) -> (i32, i32, i32) {
    let div: i32;
    let modulo: i32;
    let abs_div: i32;
    unsafe { asm!("swi 0x06"
        :"={r0}"(div),
         "={r1}"(modulo),
         "={r3}"(abs_div)
        :"{r0}"(num),
         "{r1}"(denom)
    );}
    (div, modulo, abs_div)
}

/// Calculates the square root.
///
/// # Examples
///
/// ```
/// use gba_bios::sqrt;
/// assert_eq!(10, sqrt(100));
/// ```
#[inline(always)]
pub fn sqrt(x: u32) -> u16 {
    let out: u16;
    unsafe { asm!("swi 0x08"
         :  "={r0}"(out)
         :  "{r0}"(x)
    );}
    out
}

/// Calculates arctangent, returns a fixed point signed Q1.14 `t` s.t.
/// `tan(t * TAU/4) = x`
///
/// # Arguments
///
/// * `x` fixed point signed Q1.14 number
///
/// # Examples
///
/// ```
/// use gba_bios::arctan;
/// assert_eq!(0, arctan(0));
/// assert_eq!(0x2000000, arctan(1 << 14));
/// ```
#[inline(always)]
pub fn arctan(x: i16) -> i16 {
    let out: i16;
    unsafe { asm!("swi 0x09"
         :  "={r0}"(out)
         :  "{r0}"(x)
    );}
    out
}

/// Calculates arctan2 returns a fixed point unsigned Q0.16 `t` s.t.
/// `tan(t * TAU) = y / x`
///
/// # Arguments
///
/// * `x` fixed point signed Q1.14 number
/// * `y` fixed point signed Q1.14 number
///
/// # Examples
///
/// ```
/// use gba_bios::arctan2;
/// assert_eq!(0, arctan2(1 << 14, 0));
/// assert_eq!(0x4000000, arctan2(0, 1 << 14));
/// ```
#[inline(always)]
pub fn arctan2(x: i16, y: i16) -> i16 {
    let out: i16;
    unsafe { asm!("swi 0x0A"
         :  "={r0}"(out)
         :  "{r0}"(x), "{r1}"(y)
    );}
    out
}

