use crate::PolyFormula;
use crate::polynomial::Polynomial;

pub fn bailey_borwein_plouffe_pi() -> PolyFormula<1, 2> {
    let bbp_numerators = vec![
        Polynomial::new([4]),
        Polynomial::new([-2]),
        Polynomial::new([-1]),
        Polynomial::new([-1]),
    ];
    let bbp_denominators = vec![
        Polynomial::new([1, 8]),
        Polynomial::new([4, 8]),
        Polynomial::new([5, 8]),
        Polynomial::new([6, 8]),
    ];
    PolyFormula::new(16, bbp_numerators, bbp_denominators)
}

pub fn bellards_pi() -> PolyFormula<1, 2> {
    let bellards_numerators = vec![
        Polynomial::new([-1 << 5]),
        Polynomial::new([-1]),
        Polynomial::new([1 << 8]),
        Polynomial::new([-1 << 6]),
        Polynomial::new([-1 << 2]),
        Polynomial::new([-1 << 2]),
        Polynomial::new([1]),
    ];
    let bellards_denominators = vec![
        Polynomial::new([1, 4]) * (1 << 6),
        Polynomial::new([3, 4]) * (1 << 6),
        Polynomial::new([1, 10]) * (1 << 6),
        Polynomial::new([3, 10]) * (1 << 6),
        Polynomial::new([5, 10]) * (1 << 6),
        Polynomial::new([7, 10]) * (1 << 6),
        Polynomial::new([9, 10]) * (1 << 6),
    ];
    PolyFormula::new(-1 << 10, bellards_numerators, bellards_denominators)
}
