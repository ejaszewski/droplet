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

#[derive(Debug)]
pub struct Divisor {
    divisor: u64,
    reciprocal: u64,
    shift: u32,
}

impl Divisor {
    pub fn new(divisor: u64) -> Self {
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
            reciprocal_1.wrapping_mul((1u64 << 60).wrapping_sub(reciprocal_1.wrapping_mul(msb_40))) >> 47
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
            shift
        }
    }

    pub fn divide(&self, dividend: u128) -> (u64, u64) {
        let dividend_hilo = dividend << self.shift;
        let dividend_lo = dividend_hilo as u64;
        let dividend_hi = (dividend_hilo >> 64) as u64;
        
        // Compute initial quotient and remainder
        let mut quotient_hilo = u128::from(dividend_hi) * u128::from(self.reciprocal);
        quotient_hilo += dividend_hilo;
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
        (quotient_hi, remainder >> self.shift)
    }

    pub fn mod_mul(&self, lhs: u64, rhs: u64) -> u64 {
        self.divide(u128::from(lhs) * u128::from(rhs)).1
    }
}

pub fn mod_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    if modulus <= 1 {
        return 0;
    }
    let divisor = Divisor::new(modulus);
    let mut result = 1;
    base = base % modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = divisor.mod_mul(result, base);
        }
        exponent >>= 1;
        base = divisor.mod_mul(base, base);
    }
    result
}
