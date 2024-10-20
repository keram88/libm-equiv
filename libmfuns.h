#include "smack.h"
#include <math.h>
int __isnan(double);
int __isnanf(float);


float dummy_sqrtf(float x) {
  if (__isnanf(x) || x < 0) return nanf(0);
  __VERIFIER_assert(!isnan(x));
  float ret = __VERIFIER_nondet_float();
  #ifdef FLOAT_ENABLED
  __SMACK_top_decl("function abst__dummy_sqrtf(x: bvfloat) returns (bvfloat);");
  #else
  __SMACK_top_decl("function abst__dummy_sqrtf(x: float) returns (float);");
  #endif
  __SMACK_code("@f := abst__dummy_sqrtf(@f);", ret, x);
  __VERIFIER_assume(!isnan(ret));
  __VERIFIER_assume(ret >= 0);
  return ret;
}

double dummy_sqrt(double x) {
  if (__isnan(x) || x < 0) { __VERIFIER_assert(0); return nanf(0); }
  double ret = __VERIFIER_nondet_double();
  #ifdef FLOAT_ENABLED
  __SMACK_top_decl("function abst__dummy_sqrt(x: bvdouble) returns (bvdouble);");
  #else
  __SMACK_top_decl("function abst__dummy_sqrt(x: float) returns (float);");
  #endif
  __SMACK_code("@ := abst__dummy_sqrt(@);", ret, x);
  __VERIFIER_assume(!isnan(ret));
  __VERIFIER_assume(ret >= 0);
  return ret;
}

#ifndef FLOAT_ENABLED
// int __isnanf(float x) {
//   int ret = __VERIFIER_nondet_int();
//   __SMACK_top_decl("function abst__isnanf(x: float) returns (i32);");
//   __SMACK_code("@f := abst__isnanf(@f);", ret, x);
//   return ret;
// }
int __isnan(double x) {
  int ret = __VERIFIER_nondet_int();
  __SMACK_top_decl("function abst__isnan(x: float) returns (i32);");
  __SMACK_code("@ := abst__isnan(@);", ret, x);
  return ret;
}
#endif