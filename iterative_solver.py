#! /usr/bin/env python3

import argparse
import re

def analyze_rust(fname):
    rprefix_pat=r'verifier_equiv_(?:check|assume)_'
    rtype_pat=r'f64|f32|u64|i64|i32|u32|u16|i16|i8|u8'
    suffix_pat=r'\s*\([^,]+,\s*(\d+)\s*\)\s*;'
    equiv_ca_pat = rprefix_pat+r'(?:'+rtype_pat+')'+suffix_pat
    equiv_ca_check_pat = rprefix_pat+r'(?!'+rtype_pat+r')[\w]*'+suffix_pat

    with open(fname) as f:
        text = f.read()
    matches = re.findall(equiv_ca_pat, text)
    return matches if matches is not None else []

def analyze_c(fname):
    cprefix_pat='__VERIFIER_equiv_store_'
    ctype_pat=r'double|float|unsigned_long|signed_long|signed_int|unsigned_int|int|signed_short|unsigned_short|short|signed_char|unsigned_char|char'
    suffix_pat=r'\s*\([^,]+,\s*(\d+)\s*\)\s*;'
    equiv_store_pat = cprefix_pat+r'(?:'+ctype_pat+')'+suffix_pat
    equiv_store_check_pat = cprefix_pat+r'(?!'+ctype_pat+r')[\w]*'+suffix_pat

    with open(fname) as f:
        text = f.read()
    matches = re.findall(equiv_store_pat, text)
    if len(matches) != len(set(matches)):
        print("Warning: mismatch on store: {}. Unique {}".format(len(matches), len(set(matches))))
    return matches if matches is not None else []

def analyze(files):
    c_tags = []
    rust_tags = []
    for file in files:
        if file.endswith('.c'):
            c_tags += analyze_c(file)
        elif file.endswith('.rs'):
            rust_tags += analyze_rust(file)
    if set(rust_tags) != set(c_tags):
        print("Mismatch between Rust and C: {} {}".format(rust_tags, c_tags))
    return rust_tags

def transform_rust(file, tag):
    pass

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument(
        'input_files',
        metavar='input_files',
        nargs='+',
        type=str
    )
    parser.add_argument(
        '--analyze',
        action='store_true',
        default=False
    )
    parser.add_argument(
        '--check',
        metavar='check',
        nargs=1,
        type=str
    )
    args = parser.parse_args()
    if args.analyze:
        tags = analyze(args.input_files)
        print(tags)
    else:
        pass