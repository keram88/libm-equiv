#[macro_use]
extern crate smack;
use smack::*;

extern crate core;

use core::f32;

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

/// Absolute value (magnitude) (f32)
/// Calculates the absolute value (magnitude) of the argument `x`,
/// by direct manipulation of the bit representation of `x`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_fabsf(x: f32) -> f32 {
    // On wasm32 we know that LLVM's intrinsic will compile to an optimized
    // `f32.abs` native instruction, so we can leverage this for both code size
    // and speed.
    llvm_intrinsically_optimized! {
        #[cfg(target_arch = "wasm32")] {
            return unsafe { ::core::intrinsics::fabsf32(x) }
        }
    }
    f32::from_bits(x.to_bits() & 0x7fffffff)
}

// // PowerPC tests are failing on LLVM 13: https://github.com/rust-lang/rust/issues/88520
// #[cfg(not(target_arch = "powerpc64"))]
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use core::f32::*;

//     #[test]
//     fn sanity_check() {
//         assert_eq!(fabsf(-1.0), 1.0);
//         assert_eq!(fabsf(2.8), 2.8);
//     }

//     /// The spec: https://en.cppreference.com/w/cpp/numeric/math/fabs
//     #[test]
//     fn spec_tests() {
//         assert!(fabsf(NAN).is_nan());
//         for f in [0.0, -0.0].iter().copied() {
//             assert_eq!(fabsf(f), 0.0);
//         }
//         for f in [INFINITY, NEG_INFINITY].iter().copied() {
//             assert_eq!(fabsf(f), INFINITY);
//         }
//     }
// }

extern "C" {
    // Smack
    fn fabsf(x: f32) -> f32;
    // Musl
    fn musl_fabsf(x: f32) -> f32;
}

#[no_mangle]
fn musl_smack() {
    let x = 0.0f32.verifier_nondet();
    verifier_assume!(!x.verifier_is_nan());
    let y = unsafe { musl_fabsf(x) };
    let z = unsafe { fabsf(x) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn rust_smack() {
    let x = 0.0f32.verifier_nondet();
    verifier_assume!(!x.verifier_is_nan());
    let y = rust_fabsf(x);
    let z = unsafe { fabsf(x) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn musl_rust() {
    let x = 0.0f32.verifier_nondet();
    verifier_assume!(!x.verifier_is_nan());
    let y = unsafe { musl_fabsf(x) };
    let z = rust_fabsf(x);
    verifier_assert!(y == z);
}

fn main() {
    musl_smack();
    rust_smack();
    musl_rust();
}
