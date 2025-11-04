// SPDX-License-Identifier: MPL-2.0
// Copyright 2025 Ethan Jaszewski

use crate::modular::{Reciprocal, mod_pow_primitive};

mod modular;

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

    pub fn compute_term_float(&self, j: u32, k: u32) -> f64 {
        let mut sum = 0.0;
        for i in 0..=k {
            let denominator = self.denominator(i, j).into();
            let exponent = (k - i).into();
            let numerator = mod_pow_primitive::<u64>(self.b.into(), exponent, denominator);
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
    
    pub fn compute_float(&self, k: u32) -> f64 {
        let mut sum = 0.0;
        for (idx, &a) in self.a.iter().enumerate() {
            if a == 0 {
                continue;
            }
            let j: u32 = u32::try_from(idx).unwrap() + 1;
            let sum_term = (a as f64) * self.compute_term_float(j, k);
            sum = (sum + sum_term).fract();
        }
        if sum < 0.0 {
            1.0 + sum
        } else {
            sum
        }
    }

    pub fn compute_term_integer(&self, j: u32, k: u32) -> u64 {
        let mut sum: u64 = 0;
        for i in 0..=k {
            let denominator = self.denominator(i, j).into();
            let reciprocal = Reciprocal::new(denominator);
            let exponent = (k - i).into();
            let numerator = reciprocal.mod_pow(self.b.into(), exponent);
            let widened_numerator = u128::from(numerator) << 64;
            let sum_term = widened_numerator / &reciprocal;
            sum = sum.wrapping_add(sum_term);
        }
        let num_terms = 64 / self.b.ilog2();
        for i in (k+1)..=(k+num_terms) {
            let denominator = self.denominator(i, j);
            let exponent = i - k;
            let widened_denominator = u128::from(self.b).saturating_pow(exponent) * u128::from(denominator);
            let widened_term: u128 = (1 << 64) / widened_denominator;
            let sum_term = (widened_term & u128::from(u64::MAX)) as u64;
            sum = sum.wrapping_add(sum_term);
        }
        sum
    }
    
    pub fn compute_integer(&self, k: u32) -> u64 {
        let mut sum: u64 = 0;
        for (idx, &a) in self.a.iter().enumerate() {
            if a == 0 {
                continue;
            }
            let j: u32 = u32::try_from(idx).unwrap() + 1;
            let mut sum_term = a.abs() as u64 * self.compute_term_integer(j, k);
            if a < 0 {
                let widened_term = (1 << 64) - u128::from(sum_term);
                sum_term = (widened_term & u128::from(u64::MAX)) as u64;
            }
            sum = sum.wrapping_add(sum_term);
        }
        sum
    }
}