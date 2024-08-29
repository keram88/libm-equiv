#![allow(unreachable_code)]
#[macro_use]
extern crate smack;
use smack::*;

extern crate core;

pub fn rust_modf(x: f64) -> (f64, f64) {
    let rv2: f64;
    let mut u = x.to_bits();
    let mask: u64;
    let e = ((u >> 52 & 0x7ff) as i32) - 0x3ff;

    /* no fractional part */
    if e >= 52 {
        rv2 = x;
        if e == 0x400 && (u << 12) != 0 {
            /* nan */
            return (x, rv2);
        }
        u &= 1 << 63;
        return (f64::from_bits(u), rv2);
    }

    /* no integral part*/
    if e < 0 {
        u &= 1 << 63;
        rv2 = f64::from_bits(u);
        return (x, rv2);
    }

    mask = ((!0) >> 12) >> e;
    if (u & mask) == 0 {
        rv2 = x;
        u &= 1 << 63;
        return (f64::from_bits(u), rv2);
    }
    u &= !mask;
    rv2 = f64::from_bits(u);
    return (x - rv2, rv2);
}

extern "C" {
    // Smack
    fn modf(x: f64, y: &mut f64) -> f64;
    // Musl
    fn musl_modf(x: f64, y: &mut f64) -> f64;
}

#[no_mangle]
fn musl_smack() {
    let x = 0.0f64.verifier_nondet();
    let mut y1 = 0.0f64.verifier_nondet();
    let mut y2 = 0.0f64.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let z1 = unsafe { musl_modf(x, &mut y1) };
    let z2 = unsafe { modf(x, &mut y2) };
    verifier_assert!(z1 == z2);
    verifier_assert!(y1 == y2);    
}

#[no_mangle]
fn rust_smack() {
    let x = 0.0f64.verifier_nondet();
    let mut y1 = 0.0f64.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let z1 = unsafe { modf(x, &mut y1) };
    let (z2, y2) = rust_modf(x);
    verifier_assert!(z1 == z2);
    verifier_assert!(y1 == y2);    
}

#[no_mangle]
fn musl_rust() {
    let x = 0.0f64.verifier_nondet();
    let mut y1 = 0.0f64.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let z1 = unsafe { musl_modf(x, &mut y1) };
    let (z2, y2) = rust_modf(x);
    verifier_assert!(z1 == z2);
    verifier_assert!(y1 == y2);    
}

fn main() {
    musl_smack();
    rust_smack();
    musl_rust();
}
