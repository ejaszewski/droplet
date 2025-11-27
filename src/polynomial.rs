pub struct Polynomial<const DEGREE: usize> {
    coefficients: [i32; DEGREE],
}

impl<const DEGREE: usize> Polynomial<DEGREE> {
    pub fn new(coefficients: [i32; DEGREE]) -> Self {
        coefficients.into()
    }

    pub fn evaluate(&self, x: i64) -> i64 {
        let mut coeff_iter = self.coefficients.iter().copied().rev();
        let mut result = coeff_iter.next().unwrap_or(0).into();
        for coeff in coeff_iter {
            result *= x;
            result += i64::from(coeff);
        }
        result
    }
}

impl<const N: usize> std::convert::From<[i32; N]> for Polynomial<N> {
    fn from(coefficients: [i32; N]) -> Self {
        Self { coefficients }
    }
}

impl<const DEGREE: usize> std::fmt::Display for Polynomial<DEGREE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const SUPERSCRIPTS: [char; 10] = [
            '\u{2070}', '\u{00B9}', '\u{00B2}', '\u{00B3}', '\u{2074}', '\u{2075}', '\u{2076}',
            '\u{2077}', '\u{2078}', '\u{2079}',
        ];
        // Special case for null polynomial
        if DEGREE == 0 {
            return f.write_str("0");
        }
        let mut terms: Vec<String> = vec![];
        // Constant term
        if DEGREE == 1 || self.coefficients[0] != 0 {
            terms.push(format!("{}", self.coefficients[0]));
        }
        for (mut degree, &coeff) in self.coefficients.iter().enumerate().skip(1) {
            if coeff == 0 {
                continue;
            }
            let mut exponent_chars = vec![];
            if degree > 1 {
                while degree > 0 {
                    let digit = degree % 10;
                    degree /= 10;
                    exponent_chars.push(SUPERSCRIPTS[digit]);
                }
            }
            terms.push(format!(
                "{} {}x{}",
                if coeff > 0 { "+" } else { "-" },
                coeff.abs(),
                exponent_chars.iter().rev().collect::<String>()
            ));
        }
        f.write_str(terms.join(" ").as_str())
    }
}
