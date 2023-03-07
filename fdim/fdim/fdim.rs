#[macro_use]
extern crate smack;
use smack::*;

extern crate core;

use core::f64;

/// Positive difference (f64)
///
/// Determines the positive difference between arguments, returning:
/// * x - y	if x > y, or
/// * +0	if x <= y, or
/// * NAN	if either argument is NAN.
///
/// A range error may occur.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_fdim(x: f64, y: f64) -> f64 {
    if x.is_nan() {
        x
    } else if y.is_nan() {
        y
    } else if x > y {
        x - y
    } else {
        0.0
    }
}

extern "C" {
    // Smack
    fn fdim(x: f64, y: f64) -> f64;
    // Musl
    fn musl_fdim(x: f64, y: f64) -> f64;
}

#[no_mangle]
fn musl_smack() {
    let x1 = 0.0f64.verifier_nondet();
    verifier_assume!(!x1.is_nan());
    let x2 = 0.0f64.verifier_nondet();
    verifier_assume!(!x2.is_nan());
    let y = unsafe { musl_fdim(x1, x2) };
    let z = unsafe { fdim(x1, x2) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn rust_smack() {
    let x1 = 0.0f64.verifier_nondet();
    verifier_assume!(!x1.is_nan());
    let x2 = 0.0f64.verifier_nondet();
    verifier_assume!(!x2.is_nan());
    let y = rust_fdim(x1, x2);
    let z = unsafe { fdim(x1, x2) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn musl_rust() {
    let x1 = 0.0f64.verifier_nondet();
    verifier_assume!(!x1.is_nan());
    let x2 = 0.0f64.verifier_nondet();
    verifier_assume!(!x2.is_nan());
    let y = unsafe { musl_fdim(x1, x2) };
    let z = rust_fdim(x1, x2);
    verifier_assert!(y == z);
}

fn main() {
    musl_smack();
    rust_smack();
    musl_rust();
}
