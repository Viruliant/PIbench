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

use dashu::integer::IBig;
use std::convert::TryInto;

pub fn fractional_approx(
    numerator: u64,
    denominator: u64,
    digits: usize,
    base: usize,
    pi: &IBig,
) {
    let scale = IBig::from(base).pow(digits);

    let scaled_approx =
        (IBig::from(numerator) * &scale) / IBig::from(denominator);

    let radix: u8 = base.try_into().unwrap();

    let approx_digits =
        scaled_approx.in_radix(radix).to_string().to_uppercase();

    let pi_digits =
        pi.in_radix(radix).to_string().to_uppercase();

    let matching = approx_digits
        .chars()
        .zip(pi_digits.chars())
        .take_while(|(a, b)| a == b)
        .count();

    println!("{}/{} approximation (base {})", numerator, denominator, base);
    println!("{}", approx_digits);
    println!("Correct leading digits: {}", matching);

    if matching < pi_digits.len() && matching < approx_digits.len() {
        let expected = pi_digits.chars().nth(matching).unwrap();
        let actual = approx_digits.chars().nth(matching).unwrap();

        println!(
            "First mismatch at digit {}: expected '{}', got '{}'",
            matching + 1,
            expected,
            actual
        );
    } else {
        println!("Approximation is accurate to all {} digits", pi_digits.len());
    }
}
