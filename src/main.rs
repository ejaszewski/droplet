// SPDX-License-Identifier: MPL-2.0
// Copyright 2025 Ethan Jaszewski

use std::time::{Duration, Instant};

use droplet::Formula;

fn main() {
    let hex_idx = 5_000_000 - 50;
    
    // let formula = Formula::new(2, 2, 1, vec![0, 1]);
    // let digit_idx = hex_idx * 4;

    let formula = Formula::new(16, 8, 1, vec![4, 0, 0, -2, -1, -1, 0, 0]);
    let digit_idx = hex_idx;
    
    let iters = 1;

    let mut float_time = Duration::ZERO;
    let mut float_val = 0.0;
    for _ in 0..iters {
        let float_start = Instant::now();
        float_val = formula.compute_float(digit_idx);
        float_time += float_start.elapsed();
    }

    let mut uint_time = Duration::ZERO;
    let mut uint_val = 0;
    for _ in 0..iters {
        let uint_start = Instant::now();
        uint_val = formula.compute_integer(digit_idx);
        uint_time += uint_start.elapsed();
    }
    

    let float_digits = float_val * 48_f64.exp2();
    let hex_digits = float_digits as u64;
    println!("{:08x}", hex_digits);
    println!("{:08x}", uint_val >> 16);
    println!("Float: {:?}, UInt: {:?}", float_time / iters, uint_time / iters);
}
