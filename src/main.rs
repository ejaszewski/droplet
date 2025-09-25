// SPDX-License-Identifier: MPL-2.0
// Copyright 2025 Ethan Jaszewski

fn mod_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }
    result
}

fn compute_fractional(n: u32) -> f64 {
    let mut sum = 0.0;
    for k in 0..=n.into() {
        let polynomial = 2 * k + 2;
        let numerator = mod_pow(2, u64::from(n) - k, polynomial);
        let term = numerator as f64 / polynomial as f64;
        sum = (sum + term).fract();
    }
    for k in (n+1)..(n+32) {
        let polynomial = 2 * k + 2;
        let exponent: i64 = i64::from(n) - i64::from(k);
        let term = (exponent as f64).exp2() / (polynomial as f64);
        sum = (sum + term).fract();
    }
    sum
}

fn main() {
    let hex_idx = 250_000_000 - 50;
    let float_val = compute_fractional(hex_idx * 4);
    let mut float_acc = float_val;
    let mut hex_digits = 0;
    for _ in 0..8 {
        hex_digits <<= 4;
        float_acc = float_acc.fract();
        float_acc *= 16.0;
        hex_digits += float_acc.floor() as u32;
    }
    println!("{:x}", hex_digits);
}
