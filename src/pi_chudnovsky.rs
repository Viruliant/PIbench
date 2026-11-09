// Copyright (C) 2023-2026 Roy Pfund. All rights reserved.
//
// Permission is  hereby  granted,  free  of  charge,  to  any  person
// obtaining a copy of  this  software  and  associated  documentation
// files  (the  "Software"),  to  deal   in   the   Software   without
// restriction, including without limitation the rights to use,  copy,
// modify, merge, publish, distribute, sublicense, and/or sell  copies
// of the Software, and to permit persons  to  whom  the  Software  is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and  this  permission  notice  shall  be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT  WARRANTY  OF  ANY  KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES  OF
// MERCHANTABILITY,   FITNESS   FOR   A   PARTICULAR    PURPOSE    AND
// NONINFRINGEMENT.  IN  NO  EVENT  SHALL  THE  AUTHORS  OR  COPYRIGHT
// OWNER(S) BE LIABLE FOR  ANY  CLAIM,  DAMAGES  OR  OTHER  LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING  FROM,
// OUT OF OR IN CONNECTION WITH THE  SOFTWARE  OR  THE  USE  OR  OTHER
// DEALINGS IN THE SOFTWARE.

use dashu::base::SquareRoot;
use dashu::integer::IBig;

const C3_OVER_24: u64 = 10_939_058_860_032_000;

fn bs(a: u64, b: u64) -> (IBig, IBig, IBig) {
    if b - a == 1 {
        if a == 0 {
            return (IBig::from(1u8), IBig::from(1u8), IBig::from(13_591_409u64));
        }
        let p = IBig::from(6 * a - 5) * IBig::from(2 * a - 1) * IBig::from(6 * a - 1);
        let q = IBig::from(a).pow(3) * IBig::from(C3_OVER_24);
        let mut t = &p * IBig::from(13_591_409u64 + 545_140_134u64 * a);
        if a % 2 == 1 {
            t = -t;
        }
        (p, q, t)
    } else {
        let m = (a + b) / 2;
        let (p1, q1, t1) = bs(a, m);
        let (p2, q2, t2) = bs(m, b);
        let t = &t1 * &q2 + &p1 * &t2;
        (&p1 * &p2, q1 * q2, t)
    }
}

/// Compute floor(pi * 10^decimal_digits) using Chudnovsky with binary splitting.
pub fn pi_chudnovsky(decimal_digits: usize) -> IBig {
    const GUARD: usize = 10;
    let d = decimal_digits + GUARD;

    // ~14.18 digits per term.
    let terms = (d / 14 + 2) as u64;
    let (_p, q, t) = bs(0, terms);

    // sqrt(10005) * 10^d via integer square root.
    let one_sq = IBig::from(10u8).pow(2 * d);
    let sqrt_c = (IBig::from(10005u32) * one_sq).sqrt();

    // pi * 10^d = 426880 * sqrt(10005) * Q / T
    let pi = (q * IBig::from(426_880u32) * sqrt_c) / t;

    // Drop the guard digits (truncation == floor for positive values).
    pi / IBig::from(10u8).pow(GUARD)
}
