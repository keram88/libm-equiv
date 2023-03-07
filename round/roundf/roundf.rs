#[macro_use]
extern crate smack;
use smack::*;

extern crate core;

use core::f32;

extern "C" {
    fn truncf(x: f32) -> f32;
    fn copysignf(x: f32, y: f32) -> f32;
}

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_roundf(x: f32) -> f32 {
    unsafe { truncf(x + copysignf(0.5 - 0.25 * f32::EPSILON, x)) }
}

// // PowerPC tests are failing on LLVM 13: https://github.com/rust-lang/rust/issues/88520
// #[cfg(not(target_arch = "powerpc64"))]
// #[cfg(test)]
// mod tests {
//     use super::roundf;

//     #[test]
//     fn negative_zero() {
//         assert_eq!(roundf(-0.0_f32).to_bits(), (-0.0_f32).to_bits());
//     }

//     #[test]
//     fn sanity_check() {
//         assert_eq!(roundf(-1.0), -1.0);
//         assert_eq!(roundf(2.8), 3.0);
//         assert_eq!(roundf(-0.5), -1.0);
//         assert_eq!(roundf(0.5), 1.0);
//         assert_eq!(roundf(-1.5), -2.0);
//         assert_eq!(roundf(1.5), 2.0);
//     }
// }

extern "C" {
    // Smack
    fn roundf(x: f32) -> f32;
    // Musl
    fn musl_roundf(x: f32) -> f32;
}

#[no_mangle]
fn musl_smack() {
    let x = 0.0f32.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let y = unsafe { musl_roundf(x) };
    let z = unsafe { roundf(x) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn rust_smack() {
    let x = 0.0f32.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let y = rust_roundf(x);
    let z = unsafe { roundf(x) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn musl_rust() {
    let x = 0.0f32.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let y = unsafe { musl_roundf(x) };
    let z = rust_roundf(x);
    verifier_assert!(y == z);
}

fn main() {
    musl_smack();
    rust_smack();
    musl_rust();
}
