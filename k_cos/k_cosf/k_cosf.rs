/* origin: FreeBSD /usr/src/lib/msun/src/k_cosf.c */
/*
 * Conversion to float by Ian Lance Taylor, Cygnus Support, ian@cygnus.com.
 * Debugged and optimized by Bruce D. Evans.
 */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunPro, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */

#[macro_use]
extern crate smack;
use smack::*;

/* |cos(x) - c(x)| < 2**-34.1 (~[-5.37e-11, 5.295e-11]). */
const C0: f64 = -0.499999997251031003120; /* -0x1ffffffd0c5e81.0p-54 */
const C1: f64 = 0.0416666233237390631894; /*  0x155553e1053a42.0p-57 */
const C2: f64 = -0.00138867637746099294692; /* -0x16c087e80f1e27.0p-62 */
const C3: f64 = 0.0000243904487962774090654; /*  0x199342e0ee5069.0p-68 */

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub(crate) fn k_cosf(x: f64) -> f32 {
    let z = x * x;
    verifier_equiv_check_f64(z, 0);
    let w = z * z;
    verifier_equiv_check_f64(w, 1);
    let r = C2 + z * C3;
    verifier_equiv_check_f64(r, 2);
    verifier_equiv_check_f64((((1.0 + z * C0) + w * C1) + (w * z) * r), 3);
    verifier_equiv_check_f32((((1.0 + z * C0) + w * C1) + (w * z) * r) as f32, 4);
    (((1.0 + z * C0) + w * C1) + (w * z) * r) as f32
}

extern "C" {
    fn __kernel_cosdf(x: f64) -> f32;
}

fn main() {
    let x = 1.0f64.verifier_nondet();
    verifier_assume!(-0.785398164 <= x && x <= 0.785398164);
    let bsd_res = unsafe { __kernel_cosdf(x) };
    let rust_res = k_cosf(x);
    verifier_assert!(rust_res == bsd_res);
}