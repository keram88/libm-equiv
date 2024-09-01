#[macro_use]
extern crate smack;
use smack::*;

extern crate core;
use core::f64;
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
pub fn rust_trunc(x: f64) -> f64 {
    // On wasm32 we know that LLVM's intrinsic will compile to an optimized
    // `f64.trunc` native instruction, so we can leverage this for both code size
    // and speed.
    llvm_intrinsically_optimized! {
        #[cfg(target_arch = "wasm32")] {
            return unsafe { ::core::intrinsics::truncf64(x) }
        }
    }
    let x1p120 = f64::from_bits(0x4770000000000000); // 0x1p120f === 2 ^ 120

    let mut i: u64 = x.to_bits();
    let mut e: i64 = (i >> 52 & 0x7ff) as i64 - 0x3ff + 12;
    let m: u64;

    if e >= 52 + 12 {
        return x;
    }
    if e < 12 {
        e = 1;
    }
    m = -1i64 as u64 >> e;
    if (i & m) == 0 {
        return x;
    }
    force_eval!(x + x1p120);
    i &= !m;
    f64::from_bits(i)
}

extern "C" {
    fn trunc(x: f64) -> f64;
    fn musl_trunc(x: f64) -> f64;
}

#[no_mangle]
fn musl_rust() {
    let x: f64 = 0.0f64.verifier_nondet();
    verifier_assume!(!x.is_nan());
    verifier_assert!(unsafe { musl_trunc(x) } == rust_trunc(x));
}

#[no_mangle]
fn rust_smack() {
    let x: f64 = 0.0f64.verifier_nondet();
    verifier_assume!(!x.is_nan());
    verifier_assert!(rust_trunc(x) == unsafe { trunc(x) } );
}

#[no_mangle]
fn musl_smack() {
    let x: f64 = 0.0f64.verifier_nondet();
    verifier_assume!(!x.is_nan());
    verifier_assert!(unsafe { musl_trunc(x) } == unsafe { trunc(x) } );
}

fn main() {
    musl_smack();
}
