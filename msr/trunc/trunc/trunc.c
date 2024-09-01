#include "../../../internal/libm.h"
#include "smack.h"

double musl_trunc(double x)
{
	union {double f; uint64_t i;} u = {x};
	int e = (int)(u.i >> 52 & 0x7ff) - 0x3ff + 12;
	uint64_t m;

	if (e >= 52 + 12)
		return x;
	if (e < 12)
		e = 1;
	m = -1ULL >> e;
	if ((u.i & m) == 0)
		return x;
	FORCE_EVAL(x + 0x1p120f);
	u.i &= ~m;
	return u.f;
}

/* int main() { */
/*   double x = __VERIFIER_nondet_double(); */
/*   __VERIFIER_assume(!isnan(x)); */
/*   __VERIFIER_assert(musl_trunc(x) == trunc(x)); */
/*   return 0; */
/* } */
