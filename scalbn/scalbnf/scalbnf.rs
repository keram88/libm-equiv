#[macro_use]
extern crate smack;
use smack::*;

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_scalbnf(mut x: f32, mut n: i32) -> f32 {
    let x1p127 = f32::from_bits(0x7f000000); // 0x1p127f === 2 ^ 127
    let x1p_126 = f32::from_bits(0x800000); // 0x1p-126f === 2 ^ -126
    let x1p24 = f32::from_bits(0x4b800000); // 0x1p24f === 2 ^ 24

    if n > 127 {
        x *= x1p127;
        n -= 127;
        if n > 127 {
            x *= x1p127;
            n -= 127;
            if n > 127 {
                n = 127;
            }
        }
    } else if n < -126 {
        x *= x1p_126 * x1p24;
        n += 126 - 24;
        if n < -126 {
            x *= x1p_126 * x1p24;
            n += 126 - 24;
            if n < -126 {
                n = -126;
            }
        }
    }
    x * f32::from_bits(((0x7f + n) as u32) << 23)
}

extern "C" {
    fn musl_scalbnf(x: f32, y: i32) -> f32;
}

fn main() {
    let x: f32 = 0.0f32.verifier_nondet();
    let y: i32 = 0i32.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let r1 = rust_scalbnf(x, y);
    let r2 = unsafe { musl_scalbnf(x, y) };
    verifier_assert!(r1 == r2);
}
