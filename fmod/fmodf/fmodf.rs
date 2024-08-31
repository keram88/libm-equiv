#[macro_use]
extern crate smack;
use smack::*;

extern crate core;

use core::f32;
use core::u32;

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_fmodf(x: f32, y: f32) -> f32 {
    let mut uxi = x.to_bits();
    let mut uyi = y.to_bits();
    let mut ex = (uxi >> 23 & 0xff) as i32;
    let mut ey = (uyi >> 23 & 0xff) as i32;
    let sx = uxi & 0x80000000;
    let mut i;

    if uyi << 1 == 0 || y.is_nan() || ex == 0xff {
        return (x * y) / (x * y);
    }

    if uxi << 1 <= uyi << 1 {
        if uxi << 1 == uyi << 1 {
            return 0.0 * x;
        }

        return x;
    }

    /* normalize x and y */
    if ex == 0 {
        i = uxi << 9;
        while i >> 31 == 0 {
            ex -= 1;
            i <<= 1;
        }

        uxi <<= -ex + 1;
    } else {
        uxi &= u32::MAX >> 9;
        uxi |= 1 << 23;
    }

    if ey == 0 {
        i = uyi << 9;
        while i >> 31 == 0 {
            ey -= 1;
            i <<= 1;
        }

        uyi <<= -ey + 1;
    } else {
        uyi &= u32::MAX >> 9;
        uyi |= 1 << 23;
    }

    /* x mod y */
    while ex > ey {
        i = uxi.wrapping_sub(uyi);
        if i >> 31 == 0 {
            if i == 0 {
                return 0.0 * x;
            }
            uxi = i;
        }
        uxi <<= 1;

        ex -= 1;
    }

    i = uxi.wrapping_sub(uyi);
    if i >> 31 == 0 {
        if i == 0 {
            return 0.0 * x;
        }
        uxi = i;
    }

    while uxi >> 23 == 0 {
        uxi <<= 1;
        ex -= 1;
    }

    /* scale result up */
    if ex > 0 {
        uxi -= 1 << 23;
        uxi |= (ex as u32) << 23;
    } else {
        uxi >>= -ex + 1;
    }
    uxi |= sx;

    f32::from_bits(uxi)
}

extern "C" {
    fn musl_fmodf(x: f32, y: f32) -> f32;
    fn fmodf(x: f32, y: f32) -> f32;
}

#[no_mangle]
fn musl_smack() {
    let x: f32 = 0.0f32.verifier_nondet();
    let y: f32 = 0.0f32.verifier_nondet();
    verifier_assume!(!x.is_nan() && !y.is_nan());
    verifier_assume!(y != 0.0);
    verifier_assume!(x.is_finite() && y.is_finite());
    let r1 = unsafe { musl_fmodf(x, y) };
    let r2 = unsafe { fmodf(x, y) };
    verifier_assert!(r1 == r2);
}

#[no_mangle]
fn rust_smack() {
    let x: f32 = 0.0f32.verifier_nondet();
    let y: f32 = 0.0f32.verifier_nondet();
    verifier_assume!(!x.is_nan() && !y.is_nan());
    verifier_assume!(y != 0.0);
    verifier_assume!(x.is_finite() && y.is_finite());
    let r1 = rust_fmodf(x, y);
    let r2 = unsafe { fmodf(x, y) };
    verifier_assert!(r1 == r2);
}

#[no_mangle]
fn musl_rust() {
    let x: f32 = 0.0f32.verifier_nondet();
    let y: f32 = 0.0f32.verifier_nondet();
    verifier_assume!(!x.is_nan() && !y.is_nan());
    verifier_assume!(y != 0.0);
    verifier_assume!(x.is_finite() && y.is_finite());
    let r1 = rust_fmodf(x, y);
    let r2 = unsafe { musl_fmodf(x, y) };
    verifier_assert!(r1 == r2);
}

fn main() {
    musl_rust();
}
