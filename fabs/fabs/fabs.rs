#[macro_use]
extern crate smack;
use smack::*;

extern crate core;

macro_rules! llvm_intrinsically_optimized {
    (#[cfg($($clause:tt)*)] $e:expr) => {
        #[cfg(all(feature = "unstable", $($clause)*))]
        {
            if true { // thwart the dead code lint
                $e
            }
        }
    };
}

macro_rules! force_eval {
    ($e:expr) => {
        unsafe { ::core::ptr::read_volatile(&$e) }
    };
}

use core::u64;

/// Absolute value (magnitude) (f64)
/// Calculates the absolute value (magnitude) of the argument `x`,
/// by direct manipulation of the bit representation of `x`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_fabs(x: f64) -> f64 {
    // On wasm32 we know that LLVM's intrinsic will compile to an optimized
    // `f64.abs` native instruction, so we can leverage this for both code size
    // and speed.
    llvm_intrinsically_optimized! {
        #[cfg(target_arch = "wasm32")] {
            return unsafe { ::core::intrinsics::fabsf64(x) }
        }
    }
    f64::from_bits(x.to_bits() & (u64::MAX / 2))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use core::f64::*;

//     #[test]
//     fn sanity_check() {
//         assert_eq!(fabs(-1.0), 1.0);
//         assert_eq!(fabs(2.8), 2.8);
//     }

//     /// The spec: https://en.cppreference.com/w/cpp/numeric/math/fabs
//     #[test]
//     fn spec_tests() {
//         assert!(fabs(NAN).is_nan());
//         for f in [0.0, -0.0].iter().copied() {
//             assert_eq!(fabs(f), 0.0);
//         }
//         for f in [INFINITY, NEG_INFINITY].iter().copied() {
//             assert_eq!(fabs(f), INFINITY);
//         }
//     }
// }

extern "C" {
    // Smack
    fn fabs(x: f64) -> f64;
    // Musl
    fn musl_fabs(x: f64) -> f64;
}

#[no_mangle]
fn musl_smack() {
    let x = 0.0f64.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let y = unsafe { musl_fabs(x) };
    let z = unsafe { fabs(x) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn rust_smack() {
    let x = 0.0f64.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let y = rust_fabs(x);
    let z = unsafe { fabs(x) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn musl_rust() {
    let x = 0.0f64.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let y = unsafe { musl_fabs(x) };
    let z = rust_fabs(x);
    verifier_assert!(y == z);
}

fn main() {
    musl_smack();
    rust_smack();
    musl_rust();
}
