This directory contains math functions which have a corresponding
implementation in SMACK. This allows for equivalence checking
between MUSL, Rust and SMACK implementations of the functions.
This can be done by supplying the `--entry-point <entry-point>`
option of `regtest.py`. The available entry points to choose
from are:
`musl_rust`
`musl_smack`
`rust_smack`
Which compare MUSL and Rust implementations, MUSL and SMACK
implementations and Rust and SMACK implementations
respectively.