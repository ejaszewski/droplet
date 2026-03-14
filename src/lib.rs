// SPDX-License-Identifier: MPL-2.0
// Copyright 2025 Ethan Jaszewski

use std::thread;

use crate::{
    modular::{Reciprocal, mod_pow_primitive},
    polynomial::Polynomial,
    wide::Sum,
};

pub mod formulas;
pub mod modular;
pub mod polynomial;
pub mod wide;

#[derive(Clone)]
pub struct PolyFormula<const N_DEGREE: usize, const D_DEGREE: usize> {
    alternating: bool,
    base_log2: u32,
    numerators: Vec<Polynomial<N_DEGREE>>,
    denominators: Vec<Polynomial<D_DEGREE>>,
}

impl<const N_DEGREE: usize, const D_DEGREE: usize> PolyFormula<N_DEGREE, D_DEGREE> {
    pub fn new(
        alternating: bool,
        base_log2: u32,
        numerators: Vec<Polynomial<N_DEGREE>>,
        denominators: Vec<Polynomial<D_DEGREE>>,
    ) -> Self {
        Self {
            alternating,
            base_log2,
            numerators,
            denominators,
        }
    }

    pub fn evaluate_term<const D: usize>(
        &self,
        term: usize,
        digit: u32,
        offset: u32,
        stride: usize,
    ) -> Sum<D> {
        let numerator_poly = &self.numerators[term];
        let denominator_poly = &self.denominators[term];
        let mut sum = Sum::zero();
        for i in (offset..=digit).step_by(stride) {
            // Evaluate numerator and denominator polynomials
            let denominator = denominator_poly.evaluate(i.into()).unsigned_abs();
            let numerator = numerator_poly.evaluate(i.into());

            // Determine if this term will be positive or negative
            let base_positive = !self.alternating || (i & 1 == 0);
            let term_positive = !(numerator.is_positive() ^ base_positive);

            let reciprocal = Reciprocal::new(denominator);
            let exponent = (digit - i).into();
            let numerator =
                reciprocal.mod_pow_init(numerator.unsigned_abs(), 1u64 << self.base_log2, exponent);
            let wide_numerator = Sum::from_msd(numerator);
            let sum_term = wide_numerator / &reciprocal;

            sum = if term_positive {
                sum + sum_term
            } else {
                sum - sum_term
            };
        }
        // Main thread will compute the correction terms
        let num_terms = if offset == 0 {
            (64 * D) as u32 / self.base_log2
        } else {
            0
        };
        for i in (digit + 1)..=(digit + num_terms) {
            // Evaluate numerator and denominator polynomials
            let denominator = denominator_poly.evaluate(i.into()).unsigned_abs();
            let numerator = numerator_poly.evaluate(i.into());

            // Determine if this term will be positive or negative
            let base_positive = !self.alternating || (i & 1 == 0);
            let term_positive = !(numerator.is_positive() ^ base_positive);

            let reciprocal = Reciprocal::new(denominator);

            let shift = (i - digit) * self.base_log2;
            let wide_numerator: Sum<_> = Sum::from_msd(numerator.unsigned_abs()) >> shift;
            let sum_term = wide_numerator / &reciprocal;

            sum = if term_positive {
                sum + sum_term
            } else {
                sum - sum_term
            };
        }
        sum
    }

    pub fn evaluate<const D: usize>(&self, digit: u32) -> Sum<D> {
        let mut sum = Sum::zero();
        let n_terms = self.numerators.len();
        for term in 0..n_terms {
            let term_value = self.evaluate_term(term, digit, 0, 1);
            sum = sum + term_value;
        }
        sum
    }

    pub fn evaluate_parallel<const D: usize>(&self, digit: u32, threads: usize) -> Sum<D> {
        let n_terms = self.numerators.len();
        let mut handles = Vec::with_capacity(threads);
        for id in 0..threads {
            let thread_formula = self.clone();
            let handle = thread::spawn(move || {
                let mut thread_sum: Sum<D> = Sum::zero();
                for term in 0..n_terms {
                    let term_value = thread_formula.evaluate_term(term, digit, id as u32, threads);
                    thread_sum = thread_sum + term_value;
                }
                thread_sum
            });
            handles.push(handle);
        }
        handles.into_iter().fold(Sum::zero(), |acc, handle| {
            let thread_sum = handle.join().unwrap();
            acc + thread_sum
        })
    }
}

impl<const N: usize, const D: usize> std::fmt::Display for PolyFormula<N, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let numerator_poly = self.numerators.iter().map(|poly| format!("{}", poly));
        let denominator_poly = self.denominators.iter().map(|poly| format!("{}", poly));

        let (numerators, (denominators, division)): (Vec<_>, (Vec<_>, Vec<_>)) = numerator_poly
            .zip(denominator_poly)
            .map(|(n_string, d_string)| {
                let len = n_string.len().max(d_string.len());
                let n_padded = format!("{:^width$}", n_string, width = len);
                let d_padded = format!("{:^width$}", d_string, width = len);
                let divide = "-".repeat(len);
                (n_padded, (d_padded, divide))
            })
            .unzip();

        let base_num = if self.alternating {
            format!("(-1)\u{207F}")
        } else {
            String::from("1")
        };
        let base_denom = format!("{}\u{207F}", (1u64 << self.base_log2));
        let base_len = base_num.len().max(base_denom.len());

        f.write_fmt(format_args!("{:^width$}/ ", base_num, width = base_len))?;
        f.write_str(&numerators.join("   "))?;
        f.write_fmt(format_args!(" \\\n{}| ", "-".repeat(base_len)))?;
        f.write_str(&division.join(" + "))?;
        f.write_fmt(format_args!(
            " |\n{:^width$}\\ ",
            base_denom,
            width = base_len
        ))?; //" |\n\\ ")?;
        f.write_str(&denominators.join("   "))?;
        f.write_str(" /\n")?;
        Ok(())
    }
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
        for i in (k + 1)..=(k + num_terms) {
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
        if sum < 0.0 { 1.0 + sum } else { sum }
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
        for i in (k + 1)..=(k + num_terms) {
            let denominator = self.denominator(i, j);
            let exponent = i - k;
            let widened_denominator =
                u128::from(self.b).saturating_pow(exponent) * u128::from(denominator);
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
