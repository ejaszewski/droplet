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

pub struct Formula {
    b: u32,
    n: u32,
    s: u32,
    a: Vec<i32>,
}

impl Formula {
    pub fn new(b: u32, n: u32, s: u32, a: Vec<i32>) -> Self {
        Formula { b, n, s, a }
    }

    fn denominator(&self, i: u32, j: u32) -> u32 {
        (self.n * i + j).pow(self.s)
    }

    pub fn compute_term(&self, j: u32, k: u32) -> f64 {
        let mut sum = 0.0;
        for i in 0..=k {
            let denominator = self.denominator(i, j).into();
            let exponent = (k - i).into();
            let numerator = mod_pow(self.b.into(), exponent, denominator);
            let sum_term = numerator as f64 / denominator as f64;
            sum = (sum + sum_term).fract();
        }
        let num_terms = 53 / self.b.ilog2();
        for i in (k+1)..=(k+num_terms) {
            let denominator = self.denominator(i, j);
            let exponent = i64::from(k) - i64::from(i);
            let numerator = (self.b as f64).powi(exponent as i32);
            let sum_term = numerator / (denominator as f64);
            sum = (sum + sum_term).fract();
        }
        sum
    }
    
    pub fn compute(self, k: u32) -> f64 {
        let mut sum = 0.0;
        for (idx, &a) in self.a.iter().enumerate() {
            if a == 0 {
                continue;
            }
            let j: u32 = u32::try_from(idx).unwrap() + 1;
            let sum_term = (a as f64) * self.compute_term(j, k);
            sum = (sum + sum_term).fract();
        }
        if sum < 0.0 {
            1.0 + sum
        } else {
            sum
        }
    }
}