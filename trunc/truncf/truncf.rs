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

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_truncf(x: f32) -> f32 {
    // On wasm32 we know that LLVM's intrinsic will compile to an optimized
    // `f32.trunc` native instruction, so we can leverage this for both code size
    // and speed.
    llvm_intrinsically_optimized! {
        #[cfg(target_arch = "wasm32")] {
            return unsafe { ::core::intrinsics::truncf32(x) }
        }
    }
    let x1p120 = f32::from_bits(0x7b800000); // 0x1p120f === 2 ^ 120

    let mut i: u32 = x.to_bits();
    let mut e: i32 = (i >> 23 & 0xff) as i32 - 0x7f + 9;
    let m: u32;

    if e >= 23 + 9 {
        return x;
    }
    if e < 9 {
        e = 1;
    }
    m = -1i32 as u32 >> e;
    if (i & m) == 0 {
        return x;
    }
    force_eval!(x + x1p120);
    i &= !m;
    f32::from_bits(i)
}

extern "C" {
    fn truncf(x: f32) -> f32;
    fn musl_truncf(x: f32) -> f32;
}

#[no_mangle]
fn musl_smack() {
    let x: f32 = 0.0f32.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let y = unsafe { musl_truncf(x) };
    let z = unsafe { truncf(x) };
    verifier_assert!(y == z);

}

#[no_mangle]
fn rust_smack() {
    let x: f32 = 0.0f32.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let y = rust_truncf(x);
    let z = unsafe { truncf(x) };
    verifier_assert!(y == z);

}

#[no_mangle]
fn musl_rust() {
    let x: f32 = 0.0f32.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let y = rust_truncf(x);
    let z = unsafe { truncf(x) };
    verifier_assert!(y == z);

}

fn main() {
    musl_smack();
    rust_smack();
    musl_rust();
}
