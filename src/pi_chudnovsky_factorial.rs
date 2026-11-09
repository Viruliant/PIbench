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

use dashu::{float::FBig, integer::IBig};
use std::convert::TryFrom;

fn factorial(n: u32) -> IBig {
    (1..=n).fold(IBig::from(1u8), |acc, x| acc * x)
}

/// Compute floor(pi * 10^decimal_digits) using Chudnovsky with factorials.
pub fn pi_chudnovsky_factorial(decimal_digits: usize) -> IBig {
    // Decimal digits -> binary precision.
    let bits =
        (decimal_digits as f64 * std::f64::consts::LOG2_10).ceil() as usize
            + 128;

    // Roughly 14 digits per Chudnovsky term.
    let terms = decimal_digits / 14 + 2;

    let mut sum: FBig = FBig::ZERO.with_precision(bits).value();

    for k in 0..terms {
        let k_u32 = k as u32;

        let numer = factorial(6 * k_u32)
            * IBig::from(13591409u64 + 545140134u64 * k as u64);

        let denom = factorial(3 * k_u32)
            * factorial(k_u32).pow(3)
            * IBig::from(640320u64).pow((3 * k_u32) as usize);

        // Ensure FBig conversions inherit high precision
        let term =
            FBig::try_from(numer).unwrap().with_precision(bits).value()
            / FBig::try_from(denom).unwrap().with_precision(bits).value();

        if k % 2 == 0 {
            sum += term;
        } else {
            sum -= term;
        }
    }

    let c =
        FBig::try_from(426880u64)
            .unwrap()
            .with_precision(bits)
            .value()
        * FBig::try_from(10005u64)
            .unwrap()
            .with_precision(bits)
            .value()
            .sqrt();

    let pi = (c / sum).with_precision(bits).value();
    let scale = IBig::from(10u8).pow(decimal_digits);
    let scaled = pi * FBig::try_from(scale).unwrap().with_precision(bits).value();

    // Truncation toward zero == floor for positive values.
    scaled.to_int().value()
}
