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

/// Ceil (f32)
///
/// Finds the nearest integer greater than or equal to `x`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_ceilf(x: f32) -> f32 {
    // On wasm32 we know that LLVM's intrinsic will compile to an optimized
    // `f32.ceil` native instruction, so we can leverage this for both code size
    // and speed.
    llvm_intrinsically_optimized! {
        #[cfg(target_arch = "wasm32")] {
            return unsafe { ::core::intrinsics::ceilf32(x) }
        }
    }
    let mut ui = x.to_bits();
    let e = (((ui >> 23) & 0xff).wrapping_sub(0x7f)) as i32;

    if e >= 23 {
        return x;
    }
    if e >= 0 {
        let m = 0x007fffff >> e;
        if (ui & m) == 0 {
            return x;
        }
        force_eval!(x + f32::from_bits(0x7b800000));
        if ui >> 31 == 0 {
            ui += m;
        }
        ui &= !m;
    } else {
        force_eval!(x + f32::from_bits(0x7b800000));
        if ui >> 31 != 0 {
            return -0.0;
        } else if ui << 1 != 0 {
            return 1.0;
        }
    }
    f32::from_bits(ui)
}

extern "C" {
    // Smack
    fn ceilf(x: f32) -> f32;
    // Musl
    fn musl_ceilf(x: f32) -> f32;
}

#[no_mangle]
fn musl_smack() {
    let x = 0.0f32.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let y = unsafe { musl_ceilf(x) };
    let z = unsafe { ceilf(x) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn rust_smack() {
    let x = 0.0f32.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let y = rust_ceilf(x);
    let z = unsafe { ceilf(x) };
    verifier_assert!(y == z);
}

#[no_mangle]
fn musl_rust() {
    let x = 0.0f32.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let y = unsafe { musl_ceilf(x) };
    let z = rust_ceilf(x);
    verifier_assert!(y == z);
}

fn main() {
    musl_rust();
}

