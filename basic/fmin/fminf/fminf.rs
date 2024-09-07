#[macro_use]
extern crate smack;
use smack::*;

extern crate core;

use core::f32;

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_fminf(x: f32, y: f32) -> f32 {
    // IEEE754 says: minNum(x, y) is the canonicalized number x if x < y, y if y < x, the
    // canonicalized number if one operand is a number and the other a quiet NaN. Otherwise it
    // is either x or y, canonicalized (this means results might differ among implementations).
    // When either x or y is a signalingNaN, then the result is according to 6.2.
    //
    // Since we do not support sNaN in Rust yet, we do not need to handle them.
    // FIXME(nagisa): due to https://bugs.llvm.org/show_bug.cgi?id=33303 we canonicalize by
    // multiplying by 1.0. Should switch to the `canonicalize` when it works.
    (if y.is_nan() || x < y { x } else { y }) * 1.0
}

extern "C" {
    // Smack
    fn fminf(x: f32, y: f32) -> f32;
    // Musl
    fn musl_fminf(x: f32, y: f32) -> f32;
}

#[no_mangle]
fn musl_smack() {
    let x1 = 0.0f32.verifier_nondet();
    verifier_assume!(!x1.verifier_is_nan());
    let x2 = 0.0f32.verifier_nondet();
    verifier_assume!(!x2.verifier_is_nan());
    let y = unsafe { musl_fminf(x1, x2) };
    let z = unsafe { fminf(x1, x2) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn rust_smack() {
    let x1 = 0.0f32.verifier_nondet();
    verifier_assume!(!x1.verifier_is_nan());
    let x2 = 0.0f32.verifier_nondet();
    verifier_assume!(!x2.verifier_is_nan());
    let y = rust_fminf(x1, x2);
    let z = unsafe { fminf(x1, x2) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn musl_rust() {
    let x1 = 0.0f32.verifier_nondet();
    verifier_assume!(!x1.verifier_is_nan());
    let x2 = 0.0f32.verifier_nondet();
    verifier_assume!(!x2.verifier_is_nan());
    let y = unsafe { musl_fminf(x1, x2) };
    let z = rust_fminf(x1, x2);
    verifier_assert!(y == z);
}

fn main() {
    musl_smack();
    rust_smack();
    musl_rust();
}