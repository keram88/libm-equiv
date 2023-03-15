#![allow(unreachable_code)]
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

const FP_ILOGBNAN: i32 = -1 - 0x7fffffff;
const FP_ILOGB0: i32 = FP_ILOGBNAN;

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_ilogbf(x: f32) -> i32 {
    let mut i = x.to_bits();
    let e = ((i >> 23) & 0xff) as i32;

    if e == 0 {
        i <<= 9;
        if i == 0 {
            //force_eval!(0.0 / 0.0);
            return FP_ILOGB0;
        }
        /* subnormal x */
        let mut e = -0x7f;
        while (i >> 31) == 0 {
            e -= 1;
            i <<= 1;
        }
        e
    } else if e == 0xff {
        //force_eval!(0.0 / 0.0);
        if (i << 9) != 0 {
            FP_ILOGBNAN
        } else {
            i32::max_value()
        }
    } else {
        e - 0x7f
    }
}

extern "C" {
    // Smack
    // fn ilogbf(x: f32) -> i32;
    // Musl
    fn musl_ilogbf(x: f32) -> i32;
}

// #[no_mangle]
// fn musl_smack() {
//     let x = 0.0f32.verifier_nondet();
//     verifier_assume!(!x.is_nan());
//     let y = unsafe { musl_ilogbf(x) };
//     let z = unsafe { ilogbf(x) };
//     verifier_assert!(y == z);
// }

// #[no_mangle]
// fn rust_smack() {
//     let x = 0.0f32.verifier_nondet();
//     verifier_assume!(!x.is_nan());
//     let y = rust_ilogbf(x);
//     let z = unsafe { ilogbf(x) };
//     verifier_assert!(y == z);
// }

#[no_mangle]
fn musl_rust() {
    let x = 0.0f32.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let y = unsafe { musl_ilogbf(x) };
    let z = rust_ilogbf(x);
    verifier_assert!(y == z);
}

fn main() {
//    musl_smack();
//    rust_smack();
    musl_rust();
}
