/* origin: FreeBSD /usr/src/lib/msun/src/s_fmaf.c */
/*-
 * Copyright (c) 2005-2011 David Schultz <das@FreeBSD.ORG>
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

#[macro_use]
extern crate smack;
use smack::*;

extern crate core;
use core::f32;
use core::ptr::read_volatile;

// use super::fenv::{
//     feclearexcept, fegetround, feraiseexcept, fetestexcept, FE_INEXACT, FE_TONEAREST, FE_UNDERFLOW,
// };


const FE_TONEAREST: i32 = 0;
const FE_DOWNWARD : i32 =0x400;
const FE_UPWARD: i32 =0x800;
const FE_TOWARDZERO: i32 = 0xc00;
const FE_INEXACT: i32 = 1;

extern "C" {
    fn fegetround() -> i32;
    fn fesetround(x: i32) -> i32;
}

/*
 * Fused multiply-add: Compute x * y + z with a single rounding error.
 *
 * A double has more than twice as much precision than a float, so
 * direct double-precision arithmetic suffices, except where double
 * rounding occurs.
 */

/// Floating multiply add (f32)
///
/// Computes `(x*y)+z`, rounded as one ternary operation:
/// Computes the value (as if) to infinite precision and rounds once to the result format,
/// according to the rounding mode characterized by the value of FLT_ROUNDS.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_fmaf(x: f32, y: f32, mut z: f32, feexcept: i32) -> f32 {
    let xy: f64;
    let mut result: f64;
    let mut ui: u64;
    let e: i32;

    xy = x as f64 * y as f64;
    result = xy + z as f64;
    ui = result.to_bits();
    e = (ui >> 52) as i32 & 0x7ff;
    /* Common case: The double precision result is fine. */
    if (
        /* not a halfway case */
        ui & 0x1fffffff) != 0x10000000 ||
        /* NaN */
        e == 0x7ff ||
        /* exact */
        (result - xy == z as f64 && result - z as f64 == xy) ||
        /* not round-to-nearest */
        unsafe { fegetround() } != FE_TONEAREST
    {
        /*
            underflow may not be raised correctly, example:
            fmaf(0x1p-120f, 0x1p-120f, 0x1p-149f)
        */
	// if e < 0x3ff - 126 && e >= 0x3ff - 149 && fetestexcept(FE_INEXACT) != 0 {
	if e < 0x3ff - 126 && e >= 0x3ff - 149 && feexcept == FE_INEXACT {
            // feclearexcept(FE_INEXACT);
            // prevent `xy + vz` from being CSE'd with `xy + z` above
            let vz: f32 = unsafe { read_volatile(&z) };
            result = xy + vz as f64;
	    // if fetestexcept(FE_INEXACT) != 0 {
            //     feraiseexcept(FE_UNDERFLOW);
            // } else {
            //     feraiseexcept(FE_INEXACT);
            // }
        }
        z = result as f32;
        return z;
    }

    /*
     * If result is inexact, and exactly halfway between two float values,
     * we need to adjust the low-order bit in the direction of the error.
     */
    let neg = ui >> 63 != 0;
    let err = if neg == (z as f64 > xy) {
        xy - result + z as f64
    } else {
        z as f64 - result + xy
    };
    if neg == (err < 0.0) {
        ui += 1;
    } else {
        ui -= 1;
    }
    f64::from_bits(ui) as f32
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn issue_263() {
//         let a = f32::from_bits(1266679807);
//         let b = f32::from_bits(1300234242);
//         let c = f32::from_bits(1115553792);
//         let expected = f32::from_bits(1501560833);
//         assert_eq!(super::fmaf(a, b, c), expected);
//     }
// }

extern "C" {
    fn musl_fmaf(x: f32, y: f32, z: f32, feexcept: i32) -> f32;
}

#[no_mangle]
fn musl_rust() {
    let x: f32 = 0.0f32.verifier_nondet();
    let y: f32 = 0.0f32.verifier_nondet();
    let z: f32 = 0.0f32.verifier_nondet();
    verifier_assume!(!x.is_nan());
    verifier_assume!(!y.is_nan());
    verifier_assume!(!z.is_nan());

    // If x is zero and y is infinite or if x is infinite and y is zero, and z is not a NaN, then NaN is returned and FE_INVALID is raised    
    verifier_assume!(!(x == 0.0 && y.is_infinite()));
    verifier_assume!(!(x.is_infinite() && y == 0.0));

    // If x*y is an exact infinity and z is an infinity with the opposite sign, NaN is returned
    // Inexact
    verifier_assume!(!(x*y).is_infinite());

    // Rounding mode
    let rnd: i32 = 0i32.verifier_nondet();
    // verifier_assume!(rnd == FE_TONEAREST || rnd == FE_DOWNWARD || rnd == FE_UPWARD || rnd == FE_TOWARDZERO);
    verifier_assume!(rnd == FE_TONEAREST);
    unsafe { fesetround(rnd) };

    let feexcept: i32 = 0i32.verifier_nondet();
    // verifier_assume!(feexcept == 0 || feexcept == FE_INEXACT);
    verifier_assume!(feexcept == 0);
    
    let r1 = rust_fmaf(x, y, z, feexcept);
    let r2 = unsafe { musl_fmaf(y, x, z, feexcept) };
    verifier_assert!(r1 == r2);
}

fn main() {
    musl_rust();
}
