#[macro_use]
extern crate smack;
use smack::*;

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rust_remquof(mut x: f32, mut y: f32) -> (f32, i32) {
    let ux: u32 = x.to_bits();
    let mut uy: u32 = y.to_bits();
    let mut ex = ((ux >> 23) & 0xff) as i32;
    let mut ey = ((uy >> 23) & 0xff) as i32;
    let sx = (ux >> 31) != 0;
    let sy = (uy >> 31) != 0;
    let mut q: u32;
    let mut i: u32;
    let mut uxi: u32 = ux;

    if (uy << 1) == 0 || y.is_nan() || ex == 0xff {
        return ((x * y) / (x * y), 0);
    }
    if (ux << 1) == 0 {
        return (x, 0);
    }

    /* normalize x and y */
    if ex == 0 {
        i = uxi << 9;
        while (i >> 31) == 0 {
            ex -= 1;
            i <<= 1;
        }
        uxi <<= -ex + 1;
    } else {
        uxi &= (!0) >> 9;
        uxi |= 1 << 23;
    }
    if ey == 0 {
        i = uy << 9;
        while (i >> 31) == 0 {
            ey -= 1;
            i <<= 1;
        }
        uy <<= -ey + 1;
    } else {
        uy &= (!0) >> 9;
        uy |= 1 << 23;
    }

    q = 0;
    if ex + 1 != ey {
        if ex < ey {
            return (x, 0);
        }
        /* x mod y */
        while ex > ey {
            i = uxi.wrapping_sub(uy);
            if (i >> 31) == 0 {
                uxi = i;
                q += 1;
            }
            uxi <<= 1;
            q <<= 1;
            ex -= 1;
        }
        i = uxi.wrapping_sub(uy);
        if (i >> 31) == 0 {
            uxi = i;
            q += 1;
        }
        if uxi == 0 {
            ex = -30;
        } else {
            while (uxi >> 23) == 0 {
                uxi <<= 1;
                ex -= 1;
            }
        }
    }

    /* scale result and decide between |x| and |x|-|y| */
    if ex > 0 {
        uxi -= 1 << 23;
        uxi |= (ex as u32) << 23;
    } else {
        uxi >>= -ex + 1;
    }
    x = f32::from_bits(uxi);
    if sy {
        y = -y;
    }
    if ex == ey || (ex + 1 == ey && (2.0 * x > y || (2.0 * x == y && (q % 2) != 0))) {
        x -= y;
        q += 1;
    }
    q &= 0x7fffffff;
    let quo = if sx ^ sy { -(q as i32) } else { q as i32 };
    if sx {
        (-x, quo)
    } else {
        (x, quo)
    }
}

extern "C" {
    // Musl
    fn musl_remquof(x: f32, y: f32, quo: &mut i32) -> f32;
}

#[no_mangle]
fn musl_rust() {
    let x1 = 0.0f32.verifier_nondet();
    verifier_assume!(!x1.is_nan());
    verifier_assume!(!x1.is_infinite());
    let x2 = 0.0f32.verifier_nondet();
    verifier_assume!(!x2.is_nan());
    verifier_assume!(x2 != 0.0);
    verifier_assume!(!x2.is_infinite());
    let mut quo1 = 0i32.verifier_nondet();
    let rem1 = unsafe { musl_remquof(x1, x2, &mut quo1) };
    let (rem2, quo2) = rust_remquof(x1, x2);
    verifier_assert!(rem1 == rem2);
    verifier_assert!(quo1 == quo2);
}

fn main() {
    musl_rust();
}
