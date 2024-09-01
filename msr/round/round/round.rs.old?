#[macro_use]
extern crate smack;
use smack::*;

extern crate core;

use core::f64;

extern "C" {
    fn trunc(x: f64) -> f64;
    fn copysign(x: f64, y: f64) -> f64;
}


#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_round(x: f64) -> f64 {
    unsafe { trunc(x + copysign(0.5 - 0.25 * f64::EPSILON, x)) }
}

// #[cfg(test)]
// mod tests {
//     use super::round;

//     #[test]
//     fn negative_zero() {
//         assert_eq!(round(-0.0_f64).to_bits(), (-0.0_f64).to_bits());
//     }

//     #[test]
//     fn sanity_check() {
//         assert_eq!(round(-1.0), -1.0);
//         assert_eq!(round(2.8), 3.0);
//         assert_eq!(round(-0.5), -1.0);
//         assert_eq!(round(0.5), 1.0);
//         assert_eq!(round(-1.5), -2.0);
//         assert_eq!(round(1.5), 2.0);
//     }
// }

extern "C" {
    // Smack
    fn round(x: f64) -> f64;
    // Musl
    fn musl_round(x: f64) -> f64;
}

#[no_mangle]
fn musl_smack() {
    let x = 0.0f64.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let y = unsafe { musl_round(x) };
    let z = unsafe { round(x) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn rust_smack() {
    let x = 0.0f64.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let y = rust_round(x);
    let z = unsafe { round(x) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn musl_rust() {
    let x = 0.0f64.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let y = unsafe { musl_round(x) };
    let z = rust_round(x);
    verifier_assert!(y == z);
}

fn main() {
    musl_smack();
    rust_smack();
    musl_rust();
}
