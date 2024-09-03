#[macro_use]
extern crate smack;
use smack::*;

extern crate core;

use core::f32;
/// Sign of Y, magnitude of X (f32)
///
/// Constructs a number with the magnitude (absolute value) of its
/// first argument, `x`, and the sign of its second argument, `y`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_copysignf(x: f32, y: f32) -> f32 {
    let mut ux = x.to_bits();
    let uy = y.to_bits();
    ux &= 0x7fffffff;
    ux |= uy & 0x80000000;
    f32::from_bits(ux)
}

extern "C" {
    // Smack
    fn copysignf(x: f32, y: f32) -> f32;
    // Musl
    fn musl_copysignf(x: f32, y: f32) -> f32;
}

#[no_mangle]
fn musl_smack() {
    let x1 = 0.0f32.verifier_nondet();
    verifier_assume!(!x1.verifier_is_nan());
    let x2 = 0.0f32.verifier_nondet();
    verifier_assume!(!x2.verifier_is_nan());
    let y = unsafe { musl_copysignf(x1, x2) };
    let z = unsafe { copysignf(x1, x2) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn rust_smack() {
    let x1 = 0.0f32.verifier_nondet();
    verifier_assume!(!x1.verifier_is_nan());
    let x2 = 0.0f32.verifier_nondet();
    verifier_assume!(!x2.verifier_is_nan());
    let y = rust_copysignf(x1, x2);
    let z = unsafe { copysignf(x1, x2) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn musl_rust() {
    let x1 = 0.0f32.verifier_nondet();
    verifier_assume!(!x1.verifier_is_nan());
    let x2 = 0.0f32.verifier_nondet();
    verifier_assume!(!x2.verifier_is_nan());
    let y = unsafe { musl_copysignf(x1, x2) };
    let z = rust_copysignf(x1, x2);
    verifier_assert!(y == z);
}

fn main() {
    musl_rust();
}
