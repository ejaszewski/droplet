// SPDX-License-Identifier: MPL-2.0
// Copyright 2025 Ethan Jaszewski

use droplet::Formula;

fn main() {
    let hex_idx = 50_000 - 50;
    
    // let log2_formula = Formula::new(2, 2, 1, vec![0, 1]);
    // let float_val = log2_formula.compute(hex_idx * 4);
    
    let pi_formula = Formula::new(16, 8, 1, vec![4, 0, 0, -2, -1, -1, 0, 0]);
    let float_val = pi_formula.compute(hex_idx);

    let float_digits = float_val * 32_f64.exp2();
    let hex_digits = float_digits as u64;
    println!("{:08x}", hex_digits);
}
