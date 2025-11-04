fn mod_mul_primitive<Inner>(lhs: u64, rhs: u64, modulus: u64) -> u64
where
    u64: TryFrom<Inner>,
    Inner: From<u64>
        + std::ops::Mul<Output = Inner>
        + std::ops::Rem<Output = Inner>
        + std::ops::BitAnd<Output = Inner>,
{
    // This function is quite a bit slower than pure u64 mod
    // It can probably be optimized by avoiding the umodti3 call that generates remainder
    let product = Inner::from(lhs) * Inner::from(rhs);
    let remainder = product % Inner::from(modulus);
    (remainder & u64::MAX.into()).try_into().unwrap_or_default() // Since modulus is a u64, so will the result
}

pub fn mod_pow_primitive<Inner>(mut base: u64, mut exponent: u64, modulus: u64) -> u64
where
    u64: TryFrom<Inner>,
    Inner: From<u64>
        + std::ops::Mul<Output = Inner>
        + std::ops::Rem<Output = Inner>
        + std::ops::BitAnd<Output = Inner>,
{
    if modulus <= 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = mod_mul_primitive(result, base, modulus);
        }
        exponent >>= 1;
        base = mod_mul_primitive(base, base, modulus);
    }
    result
}

#[derive(Debug)]
pub struct Reciprocal {
    divisor: u64,
    reciprocal: u64,
    shift: u32,
}

impl Reciprocal {
    pub fn new(divisor: u64) -> Self {
        divisor.into()
    }

    fn divide_impl(&self, dividend: u128) -> (u64, u64) {
        let dividend_lo = dividend as u64;
        let dividend_hi = (dividend >> 64) as u64;

        // Compute initial quotient and remainder
        let mut quotient_hilo = u128::from(dividend_hi) * u128::from(self.reciprocal);
        quotient_hilo += dividend;
        let quotient_lo = quotient_hilo as u64;
        let mut quotient_hi = (quotient_hilo >> 64) as u64;
        quotient_hi = quotient_hi.wrapping_add(1);
        let mut remainder = dividend_lo.wrapping_sub(quotient_hi.wrapping_mul(self.divisor));

        // Correct remainder and quotient
        if remainder > quotient_lo {
            quotient_hi = quotient_hi.wrapping_sub(1);
            remainder = remainder.wrapping_add(self.divisor);
        }
        if remainder >= self.divisor {
            quotient_hi = quotient_hi.wrapping_add(1);
            remainder = remainder.wrapping_sub(self.divisor);
        }
        (quotient_hi, remainder)
    }

    pub fn mod_pow(&self, mut base: u64, mut exponent: u64) -> u64 {
        let mut result = 1 << self.shift;
        while exponent > 0 {
            if exponent % 2 == 1 {
                let dividend = u128::from(result) * u128::from(base);
                result = self.divide_impl(dividend).1;
            }
            exponent >>= 1;
            let dividend = u128::from(base) * u128::from(base);
            base = self.divide_impl(dividend).1;
        }
        result >> self.shift
    }
}

impl std::ops::Div<&Reciprocal> for u64 {
    type Output = u64;

    fn div(self, rhs: &Reciprocal) -> Self::Output {
        let dividend = u128::from(self) << rhs.shift;
        rhs.divide_impl(dividend).0
    }
}

impl std::ops::Rem<&Reciprocal> for u64 {
    type Output = u64;

    fn rem(self, rhs: &Reciprocal) -> Self::Output {
        let dividend = u128::from(self) << rhs.shift;
        rhs.divide_impl(dividend).1 >> rhs.shift
    }
}

impl std::ops::Div<&Reciprocal> for u128 {
    type Output = u64;

    fn div(self, rhs: &Reciprocal) -> Self::Output {
        let dividend = self << rhs.shift;
        rhs.divide_impl(dividend).0
    }
}

impl std::ops::Rem<&Reciprocal> for u128 {
    type Output = u64;

    fn rem(self, rhs: &Reciprocal) -> Self::Output {
        let dividend = self << rhs.shift;
        rhs.divide_impl(dividend).1 >> rhs.shift
    }
}

impl std::convert::From<u64> for Reciprocal {
    fn from(divisor: u64) -> Self {
        let shift = divisor.leading_zeros();
        let divisor = divisor << shift;
        let lsb_1 = divisor & 1;
        let msb_9 = divisor >> 55;
        let msb_40 = (divisor >> 24) + 1;
        let msb_63 = ((divisor - 1) >> 1) + 1;
        let reciprocal_0 = ((1 << 19) - (3 << 8)) / msb_9;
        let reciprocal_1 = (reciprocal_0 << 11)
            .wrapping_sub(reciprocal_0.wrapping_pow(2).wrapping_mul(msb_40) >> 40)
            .wrapping_sub(1);
        let reciprocal_2 = (reciprocal_1 << 13).wrapping_add(
            reciprocal_1.wrapping_mul((1u64 << 60).wrapping_sub(reciprocal_1.wrapping_mul(msb_40)))
                >> 47,
        );
        let error = (1u128 << 96)
            .wrapping_sub(u128::from(reciprocal_2) * u128::from(msb_63))
            .wrapping_add(u128::from((reciprocal_2 >> 1) * lsb_1));
        let reciprocal_3 =
            (reciprocal_2 << 31) + (error.wrapping_mul(reciprocal_2.into()) >> 65) as u64;
        let reciprocal = reciprocal_3.wrapping_sub(
            ((u128::from(reciprocal_3) + (1 << 64) + 1).wrapping_mul(divisor.into()) >> 64) as u64,
        );
        Self {
            divisor,
            reciprocal,
            shift,
        }
    }
}
