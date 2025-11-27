// SPDX-License-Identifier: MPL-2.0
// Copyright 2025 Ethan Jaszewski

use std::time::{Duration, Instant};

use droplet::polynomial::Polynomial;
use droplet::{Formula, PolyFormula};

fn main() {
    let hex_idx = 5_000_000;
    
    // let formula = Formula::new(2, 2, 1, vec![0, 1]);
    // let digit_idx = hex_idx * 4;

    // let poly_numerator = Polynomial::new([1]);
    // let poly_denominator = Polynomial::new([2, 2]);
    // let poly_formula = PolyFormula::new(2, vec![poly_numerator], vec![poly_denominator]);

    let formula = Formula::new(16, 8, 1, vec![4, 0, 0, -2, -1, -1, 0, 0]);
    let digit_idx = hex_idx;

    let poly_numerators = vec![
        Polynomial::new([4]),
        Polynomial::new([-2]),
        Polynomial::new([-1]),
        Polynomial::new([-1]),
    ];
    let poly_denominators = vec![
        Polynomial::new([1, 8]),
        Polynomial::new([4, 8]),
        Polynomial::new([5, 8]),
        Polynomial::new([6, 8]),
    ];
    let poly_formula = PolyFormula::new(16, poly_numerators, poly_denominators);
    
    let iters = 5;

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

    let mut poly_time = Duration::ZERO;
    let mut poly_val = 0;
    for _ in 0..iters {
        let uint_start = Instant::now();
        poly_val = poly_formula.evaluate(digit_idx);
        poly_time += uint_start.elapsed();
    }
    

    let float_digits = float_val * 48_f64.exp2();
    let hex_digits = float_digits as u64;
    println!("{:08x}", hex_digits);
    println!("{:08x}", uint_val >> 16);
    println!("{:08x}", poly_val >> 16);
    println!("Float: {:?}, UInt: {:?}, Poly: {:?}", float_time / iters, uint_time / iters, poly_time / iters);
}
