#[macro_use]
extern crate smack;
use smack::*;

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_rint(x: f64) -> f64 {
    let one_over_e = 1.0 / f64::EPSILON;
    let as_u64: u64 = x.to_bits();
    let exponent: u64 = as_u64 >> 52 & 0x7ff;
    let is_positive = (as_u64 >> 63) == 0;
    if exponent >= 0x3ff + 52 {
        x
    } else {
        let ans = if is_positive {
            x + one_over_e - one_over_e
        } else {
            x - one_over_e + one_over_e
        };

        if ans == 0.0 {
            if is_positive {
                0.0
            } else {
                -0.0
            }
        } else {
            ans
        }
    }
}

// // PowerPC tests are failing on LLVM 13: https://github.com/rust-lang/rust/issues/88520
// #[cfg(not(target_arch = "powerpc64"))]
// #[cfg(test)]
// mod tests {
//     use super::rint;

//     #[test]
//     fn negative_zero() {
//         assert_eq!(rint(-0.0_f64).to_bits(), (-0.0_f64).to_bits());
//     }

//     #[test]
//     fn sanity_check() {
//         assert_eq!(rint(-1.0), -1.0);
//         assert_eq!(rint(2.8), 3.0);
//         assert_eq!(rint(-0.5), -0.0);
//         assert_eq!(rint(0.5), 0.0);
//         assert_eq!(rint(-1.5), -2.0);
//         assert_eq!(rint(1.5), 2.0);
//     }
// }

extern "C" {
    // Smack
    fn rint(x: f64) -> f64;
    // Musl
    fn musl_rint(x: f64) -> f64;
}

#[no_mangle]
fn musl_smack() {
    let x1 = 0.0f64.verifier_nondet();
    verifier_assume!(!x1.is_nan());
    let y = unsafe { musl_rint(x1) };
    let z = unsafe { rint(x1) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn rust_smack() {
    let x1 = 0.0f64.verifier_nondet();
    verifier_assume!(!x1.is_nan());
    let y = rust_rint(x1);
    let z = unsafe { rint(x1) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn musl_rust() {
    let x1 = 0.0f64.verifier_nondet();
    verifier_assume!(!x1.is_nan());
    let y = unsafe { musl_rint(x1) };
    let z = rust_rint(x1);
    verifier_assert!(y == z);
}

fn main() {
    musl_smack();
    rust_smack();
    musl_rust();
}
