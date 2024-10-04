#[macro_use]
extern crate smack;
use smack::*;

extern crate core;

use core::f32;
pub fn rust_frexp(x: f64) -> (f64, i32) {
    let mut y = x.to_bits();
    let ee = ((y >> 52) & 0x7ff) as i32;

    if ee == 0 {
        if x != 0.0 {
            let x1p64 = f64::from_bits(0x43f0000000000000);
            let (x, e) = rust_frexp(x * x1p64);
            return (x, e - 64);
        }
        return (x, 0);
    } else if ee == 0x7ff {
        return (x, 0);
    }

    let e = ee - 0x3fe;
    y &= 0x800fffffffffffff;
    y |= 0x3fe0000000000000;
    return (f64::from_bits(y), e);
}

extern "C" {
    fn musl_frexp(x: f64, y: &mut i32) -> f64;
}

fn main() {
    let x: f64 = 0.0f64.verifier_nondet();
    verifier_assume!(!x.is_nan());
    verifier_assume!(!x.is_infinite());
    let (z1, y1) = rust_frexp(x);
    let mut y: i32 = 0i32.verifier_nondet();
    let z = unsafe { musl_frexp(x, &mut y) };
    verifier_assert!(z == z1);
    verifier_assert!(y == y1);
}
