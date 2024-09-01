#[macro_use]
extern crate smack;
use smack::*;

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_rintf(x: f32) -> f32 {
    let one_over_e = 1.0 / f32::EPSILON;
    let as_u32: u32 = x.to_bits();
    let exponent: u32 = as_u32 >> 23 & 0xff;
    let is_positive = (as_u32 >> 31) == 0;
    if exponent >= 0x7f + 23 {
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
//     use super::rintf;

//     #[test]
//     fn negative_zero() {
//         assert_eq!(rintf(-0.0_f32).to_bits(), (-0.0_f32).to_bits());
//     }

//     #[test]
//     fn sanity_check() {
//         assert_eq!(rintf(-1.0), -1.0);
//         assert_eq!(rintf(2.8), 3.0);
//         assert_eq!(rintf(-0.5), -0.0);
//         assert_eq!(rintf(0.5), 0.0);
//         assert_eq!(rintf(-1.5), -2.0);
//         assert_eq!(rintf(1.5), 2.0);
//     }
// }

extern "C" {
    // Smack
    fn rintf(x: f32) -> f32;
    // Musl
    fn musl_rintf(x: f32) -> f32;
}

#[no_mangle]
fn musl_smack() {
    let x1 = 0.0f32.verifier_nondet();
    verifier_assume!(!x1.is_nan());
    let y = unsafe { musl_rintf(x1) };
    let z = unsafe { rintf(x1) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn rust_smack() {
    let x1 = 0.0f32.verifier_nondet();
    verifier_assume!(!x1.is_nan());
    let y = rust_rintf(x1);
    let z = unsafe { rintf(x1) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn musl_rust() {
    let x1 = 0.0f32.verifier_nondet();
    verifier_assume!(!x1.is_nan());
    let y = unsafe { musl_rintf(x1) };
    let z = rust_rintf(x1);
    verifier_assert!(y == z);
}

fn main() {
    musl_rust();
}
