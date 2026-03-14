use crate::modular::Reciprocal;

#[derive(Debug)]
pub struct Sum<const D: usize> {
    pub digits: [u64; D],
}

impl<const D: usize> Sum<D> {
    pub fn zero() -> Self {
        Self { digits: [0; D] }
    }

    pub fn from_msd(high: u64) -> Self {
        let mut digits = [0; D];
        digits[0] = high;
        Self { digits }
    }

    pub fn from_lsd(low: u64) -> Self {
        let mut digits = [0; D];
        digits[D - 1] = low;
        Self { digits }
    }
}

impl<const D: usize> std::ops::Div<&Reciprocal> for Sum<D> {
    type Output = Self;

    fn div(self, rhs: &Reciprocal) -> Self::Output {
        let mut output = Self::zero();
        self.digits
            .iter()
            .enumerate()
            .fold(0u64, |remainder, (idx, word)| {
                let dividend = (u128::from(remainder) << 64) + u128::from(*word);
                let quotient = dividend / rhs;
                let remainder = dividend % rhs;
                output.digits[idx] = quotient;
                remainder
            });
        output
    }
}

impl<const D: usize> std::ops::Add for Sum<D> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Self::zero();
        let lhs_iter = self.digits.into_iter();
        let rhs_iter = rhs.digits.into_iter();
        lhs_iter
            .zip(rhs_iter)
            .enumerate()
            .rev()
            .fold(false, |carry, (idx, (lhs, rhs))| {
                let (sum, did_carry) = lhs.carrying_add(rhs, carry);
                output.digits[idx] = sum;
                did_carry
            });
        output
    }
}

impl<const D: usize> std::ops::Sub for Sum<D> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut output = Self::zero();
        let lhs_iter = self.digits.into_iter();
        let rhs_iter = rhs.digits.into_iter();
        lhs_iter
            .zip(rhs_iter)
            .enumerate()
            .rev()
            .fold(false, |carry, (idx, (lhs, rhs))| {
                let (diff, did_borrow) = lhs.borrowing_sub(rhs, carry);
                output.digits[idx] = diff;
                did_borrow
            });
        output
    }
}

impl<const D: usize> std::ops::Shr<u32> for Sum<D> {
    type Output = Self;
    
    fn shr(self, rhs: u32) -> Self::Output {
        let mut output = Self::zero();
        let words = (rhs / 64) as usize;
        let shift = rhs % 64;
        let hi_mask = if shift > 0 { u64::MAX } else { 0 };
        if words < D {
            output.digits[words] = self.digits[0] >> shift;
        }
        for idx in (words + 1)..D {
            let hi_bits = self.digits[idx - words - 1] << (64 - shift);
            let lo_bits = self.digits[idx - words] >> shift;
            let word = (hi_bits & hi_mask) | lo_bits;
            output.digits[idx] = word;
        }
        output
    }
}