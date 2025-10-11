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

pub fn mod_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mod_mul = if modulus < u32::MAX.into() {
        mod_mul_primitive::<u64>
    } else {
        mod_mul_primitive::<u128>
    };

    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = mod_mul(result, base, modulus);
        }
        exponent >>= 1;
        base = mod_mul(base, base, modulus);
    }
    result
}
