#[macro_use]
extern crate smack;
use smack::*;

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_scalbn(x: f64, mut n: i32) -> f64 {
    let x1p1023 = f64::from_bits(0x7fe0000000000000); // 0x1p1023 === 2 ^ 1023
    let x1p53 = f64::from_bits(0x4340000000000000); // 0x1p53 === 2 ^ 53
    let x1p_1022 = f64::from_bits(0x0010000000000000); // 0x1p-1022 === 2 ^ (-1022)

    let mut y = x;

    if n > 1023 {
        y *= x1p1023;
        n -= 1023;
        if n > 1023 {
            y *= x1p1023;
            n -= 1023;
            if n > 1023 {
                n = 1023;
            }
        }
    } else if n < -1022 {
        /* make sure final n < -53 to avoid double
        rounding in the subnormal range */
        y *= x1p_1022 * x1p53;
        n += 1022 - 53;
        if n < -1022 {
            y *= x1p_1022 * x1p53;
            n += 1022 - 53;
            if n < -1022 {
                n = -1022;
            }
        }
    }
    y * f64::from_bits(((0x3ff + n) as u64) << 52)
}

extern "C" {
    fn musl_scalbn(x: f64, n: i32) -> f64;
}

fn main() {
    let x: f64 = 0.0f64.verifier_nondet();
    let y: i32 = 0i32.verifier_nondet();
    verifier_assume!(!x.is_nan());
    let r1 = rust_scalbn(x, y);
    let r2 = unsafe { musl_scalbn(x, y) };
    verifier_assert!(r1 == r2);
}
