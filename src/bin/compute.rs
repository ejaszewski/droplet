// SPDX-License-Identifier: MPL-2.0
// Copyright 2025 Ethan Jaszewski

use std::time::Instant;

use droplet::{formulas, wide::Sum};

fn main() {
    let hex_digit = 3_141_592_650;
    let calc_digit = hex_digit / 5 * 2;
    let bellards_formula = formulas::bellards_pi();

    const WORDS: usize = 24;
    let start = Instant::now();
    let digits: Sum<WORDS> = bellards_formula.evaluate_parallel(calc_digit, 24);
    let time = start.elapsed();

    let n_digits = (WORDS - 1) * 16 - (calc_digit.ilog2() as usize);
    for (idx, word) in digits.digits[1..].iter().enumerate() {
        if n_digits < 16 * idx {
            break;
        }
        println!("{idx:02}: {word:016x}");
    }

    println!("Calcualted {n_digits} digits at offset {hex_digit} in {time:?}");
}