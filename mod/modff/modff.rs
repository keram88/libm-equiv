#![allow(unreachable_code)]
#[macro_use]
extern crate smack;
use smack::*;

extern crate core;

pub fn rust_modff(x: f32) -> (f32, f32) {
    let rv2: f32;
    let mut u: u32 = x.to_bits();
    let mask: u32;
    let e = ((u >> 23 & 0xff) as i32) - 0x7f;

    /* no fractional part */
    if e >= 23 {
        rv2 = x;
        if e == 0x80 && (u << 9) != 0 {
            /* nan */
            return (x, rv2);
        }
        u &= 0x80000000;
        return (f32::from_bits(u), rv2);
    }
    /* no integral part */
    if e < 0 {
        u &= 0x80000000;
        rv2 = f32::from_bits(u);
        return (x, rv2);
    }

    mask = 0x007fffff >> e;
    if (u & mask) == 0 {
        rv2 = x;
        u &= 0x80000000;
        return (f32::from_bits(u), rv2);
    }
    u &= !mask;
    rv2 = f32::from_bits(u);
    return (x - rv2, rv2);
}

extern "C" {
    // Smack
    fn modff(x: f32, y: &mut f32) -> f32;
    // Musl
    fn musl_modff(x: f32, y: &mut f32) -> f32;
}

#[no_mangle]
fn musl_smack() {
    let x = 0.0f32.verifier_nondet();
    let mut y1 = 0.0f32.verifier_nondet();
    let mut y2 = 0.0f32.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let z1 = unsafe { musl_modff(x, &mut y1) };
    let z2 = unsafe { modff(x, &mut y2) };
    verifier_assert!(z1 == z2);
    verifier_assert!(y1 == y2);    
}

#[no_mangle]
fn rust_smack() {
    let x = 0.0f32.verifier_nondet();
    let mut y1 = 0.0f32.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let z1 = unsafe { modff(x, &mut y1) };
    let (z2, y2) = rust_modff(x);
    verifier_assert!(z1 == z2);
    verifier_assert!(y1 == y2);    
}

#[no_mangle]
fn musl_rust() {
    let x = 0.0f32.verifier_nondet();
    let mut y1 = 0.0f32.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let z1 = unsafe { musl_modff(x, &mut y1) };
    let (z2, y2) = rust_modff(x);
    verifier_assert!(z1 == z2);
    verifier_assert!(y1 == y2);    
}

fn main() {
    musl_smack();
    rust_smack();
    musl_rust();
}
