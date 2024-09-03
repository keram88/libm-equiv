#[macro_use]
extern crate smack;
use smack::*;

extern crate core;
use core::f64;

/// Sign of Y, magnitude of X (f64)
///
/// Constructs a number with the magnitude (absolute value) of its
/// first argument, `x`, and the sign of its second argument, `y`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_copysign(x: f64, y: f64) -> f64 {
    let mut ux = x.to_bits();
    let uy = y.to_bits();
    ux &= (!0) >> 1;
    ux |= uy & (1 << 63);
    f64::from_bits(ux)
}

extern "C" {
    // Smack
    fn copysign(x: f64, y: f64) -> f64;
    // Musl
    fn musl_copysign(x: f64, y: f64) -> f64;
}

#[no_mangle]
fn musl_smack() {
    let x1 = 0.0f64.verifier_nondet();
    verifier_assume!(!x1.verifier_is_nan());
    let x2 = 0.0f64.verifier_nondet();
    verifier_assume!(!x2.verifier_is_nan());
    let y = unsafe { musl_copysign(x1, x2) };
    let z = unsafe { copysign(x1, x2) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn rust_smack() {
    let x1 = 0.0f64.verifier_nondet();
    verifier_assume!(!x1.verifier_is_nan());
    let x2 = 0.0f64.verifier_nondet();
    verifier_assume!(!x2.verifier_is_nan());
    let y = rust_copysign(x1, x2);
    let z = unsafe { copysign(x1, x2) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn musl_rust() {
    let x1 = 0.0f64.verifier_nondet();
    verifier_assume!(!x1.verifier_is_nan());
    let x2 = 0.0f64.verifier_nondet();
    verifier_assume!(!x2.verifier_is_nan());
    let y = unsafe { musl_copysign(x1, x2) };
    let z = rust_copysign(x1, x2);
    verifier_assert!(y == z);
}

fn main() {
    musl_rust();
}
