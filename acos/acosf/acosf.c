/* origin: FreeBSD /usr/src/lib/msun/src/e_acosf.c */
/*
 * Conversion to float by Ian Lance Taylor, Cygnus Support, ian@cygnus.com.
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

#include "../../include/libm.h"
#include "smack.h"
#include "../../libmfuns.h"
typedef float float_t;

static const float
pio2_hi = 1.5707962513e+00, /* 0x3fc90fda */
pio2_lo = 7.5497894159e-08, /* 0x33a22168 */
pS0 =  1.6666586697e-01,
pS1 = -4.2743422091e-02,
pS2 = -8.6563630030e-03,
qS1 = -7.0662963390e-01;

static float R(float z)
{
	float_t p, q;
	p = z*(pS0+z*(pS1+z*pS2));
	q = 1.0f+z*qS1;
	return p/q;
}

float musl_acosf(float x)
{
	float z,w,s,c,df;
	uint32_t hx,ix;

	GET_FLOAT_WORD(hx, x);
	__VERIFIER_equiv_store_unsigned_int(hx, 0);
	ix = hx & 0x7fffffff;
	__VERIFIER_equiv_store_unsigned_int(ix, 1);
	/* |x| >= 1 or nan */
	if (ix >= 0x3f800000) {
		if (ix == 0x3f800000) {
			if (hx >> 31)
				return 2*pio2_hi + 0x1p-120f;
			return 0;
		}
		return 0/(x-x);
	}
	/* |x| < 0.5 */
	if (ix < 0x3f000000) {
		if (ix <= 0x32800000) /* |x| < 2**-26 */
			return pio2_hi + 0x1p-120f;
		__VERIFIER_equiv_store_float(R(x*x), 2);
		__VERIFIER_equiv_store_float(x*R(x*x), 3);
		__VERIFIER_equiv_store_float(pio2_lo-x*R(x*x), 4);
		__VERIFIER_equiv_store_float(x - (pio2_lo-x*R(x*x)), 5);
		__VERIFIER_equiv_store_float(pio2_hi - (x - (pio2_lo-x*R(x*x))), 6);
		return pio2_hi - (x - (pio2_lo-x*R(x*x)));
	}
	/* x < -0.5 */
	if (hx >> 31) {
		z = (1+x)*0.5f;
		s = dummy_sqrtf(z);
		w = R(z)*s-pio2_lo;
		__VERIFIER_equiv_store_float(s+w, 8);
		__VERIFIER_equiv_store_float(pio2_hi-(s+w), 9);
		__VERIFIER_equiv_store_float(2*(pio2_hi - (s+w)), 10);
		return 2*(pio2_hi - (s+w));
	}
	/* x > 0.5 */
	z = (1-x)*0.5f;
	s = dummy_sqrtf(z);
	__VERIFIER_equiv_store_float(s, 11);
	GET_FLOAT_WORD(hx,s);
	__VERIFIER_equiv_store_unsigned_int(hx, 12);
	SET_FLOAT_WORD(df,hx&0xfffff000);
	__VERIFIER_equiv_store_float(df, 13);
	__VERIFIER_equiv_store_float(z-df*df, 14);
	__VERIFIER_equiv_store_float(s+df, 15);
	c = (z-df*df)/(s+df);
	__VERIFIER_equiv_store_float(c, 16);
	__VERIFIER_equiv_store_float(R(z), 17);
	__VERIFIER_equiv_store_float(R(z)*s, 18);
	w = R(z)*s+c;
	__VERIFIER_equiv_store_float(w, 19);
	return 2*(df+w);
}
