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
mod fractional_approx;
mod pi_chudnovsky;
mod pi_chudnovsky_factorial;

use fractional_approx::fractional_approx;
use pi_chudnovsky::pi_chudnovsky;
use pi_chudnovsky_factorial::pi_chudnovsky_factorial;
use dashu as dsu; // dashu = "0.6.2"
use dsu::integer::IBig;
use pico_args;

fn parse_args() -> (bool, bool, usize, usize) {
    let mut args = pico_args::Arguments::from_env();
    let show_hex_dump = args.contains("--hex-dump");
    let show_approx = args.contains("--show-approx");
    let base: usize = args.value_from_str("-b").unwrap_or(10);
    let digits: usize = args.free_from_str().unwrap_or(100);

    if !(2..=36).contains(&base) {
        eprintln!("Error: Base must be between 2 and 36");
        std::process::exit(1);
    }

    (show_hex_dump, show_approx, base, digits)
}

fn main() {
    let (show_hex_dump, show_approx, base, digits) = parse_args();

    // Benchmark binary splitting version
    let start = std::time::Instant::now();
    let scaled_pi = pi_chudnovsky(digits);
    let elapsed_bs = start.elapsed();

    // Benchmark factorial version
    let start = std::time::Instant::now();
    let scaled_pi_fact = pi_chudnovsky_factorial(digits);
    let elapsed_fact = start.elapsed();

    println!("Pi × 10^{} (binary splitting):", digits);
    println!("{}", scaled_pi);
    println!("\nBinary splitting: {:.6}s", elapsed_bs.as_secs_f64());

    println!("\nFactorial version:");
    let output_fact = scaled_pi_fact.in_radix(base.try_into().unwrap()).to_string();
    println!("{}", output_fact.to_uppercase());
    println!("Factorial version: {:.6}s", elapsed_fact.as_secs_f64());

    assert_eq!(scaled_pi, scaled_pi_fact);
    println!("\nBoth implementations agree.");

    if show_hex_dump {
        println!("Hex dump not implemented.");
    }

    if show_approx {
        fractional_approx(22, 7, digits, base, &scaled_pi);
    }

    // Also test other approximations
    fractional_approx(355, 113, digits, base, &scaled_pi);
}
