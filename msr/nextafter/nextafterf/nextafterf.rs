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
pub fn rust_nextafterf(x: f32, y: f32) -> f32 {
    if x.is_nan() || y.is_nan() {
        return x + y;
    }

    let mut ux_i = x.to_bits();
    let uy_i = y.to_bits();
    if ux_i == uy_i {
        return y;
    }

    let ax = ux_i & 0x7fff_ffff_u32;
    let ay = uy_i & 0x7fff_ffff_u32;
    if ax == 0 {
        if ay == 0 {
            return y;
        }
        ux_i = (uy_i & 0x8000_0000_u32) | 1;
    } else if ax > ay || ((ux_i ^ uy_i) & 0x8000_0000_u32) != 0 {
        ux_i -= 1;
    } else {
        ux_i += 1;
    }

    let e = ux_i.wrapping_shr(0x7f80_0000_u32);
    // raise overflow if ux_f is infinite and x is finite
    if e == 0x7f80_0000_u32 {
        force_eval!(x + x);
    }
    let ux_f = f32::from_bits(ux_i);
    // raise underflow if ux_f is subnormal or zero
    if e == 0 {
        force_eval!(x * x + ux_f * ux_f);
    }
    ux_f
}

extern "C" {
    fn musl_nextafterf(x: f32, y: f32) -> f32;
}

#[no_mangle]
fn musl_rust() {
    let x1 = 0.0f32.verifier_nondet();
    verifier_assume!(!x1.is_nan());
    let x2 = 0.0f32.verifier_nondet();
    verifier_assume!(!x2.is_nan());
    let y = unsafe { musl_nextafterf(x1, x2) };
    let z = rust_nextafterf(x1, x2);
    verifier_assert!(y == z);
}

fn main() {
    musl_rust();
}
