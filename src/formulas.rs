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
    PolyFormula::new(false, 4, bbp_numerators, bbp_denominators)
}

pub fn euler_pi() -> PolyFormula<1,2> {
    let euler_numerators = vec![
        Polynomial::new([2]),
        Polynomial::new([2]),
        Polynomial::new([1]),
    ];
    let euler_denominators = vec![
        Polynomial::new([1, 4]),
        Polynomial::new([2, 4]),
        Polynomial::new([3, 4]),
    ];
    PolyFormula::new(true, 2, euler_numerators, euler_denominators)
}

pub fn bellards_pi() -> PolyFormula<1, 2> {
    let bellards_numerators = vec![
        Polynomial::new([-1]),
        Polynomial::new([-1]),
        Polynomial::new([4]),
        Polynomial::new([-1]),
        Polynomial::new([-1]),
        Polynomial::new([-1]),
        Polynomial::new([1]),
    ];
    let bellards_denominators = vec![
        Polynomial::new([1, 4]) * 2i32,
        Polynomial::new([3, 4]) * 2i32.pow(6),
        Polynomial::new([1, 10]),
        Polynomial::new([3, 10]),
        Polynomial::new([5, 10]) * 2i32.pow(4),
        Polynomial::new([7, 10]) * 2i32.pow(4),
        Polynomial::new([9, 10]) * 2i32.pow(6),
    ];
    PolyFormula::new(true, 10, bellards_numerators, bellards_denominators)
}
