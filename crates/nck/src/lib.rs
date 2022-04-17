use num::BigUint;

pub fn n_choose_k(n: u128, k: u128) -> BigUint {
    match k {
        0 => big(1),
        1 => big(n),
        k if k == n => big(1),
        k if n - k == 1 => big(n),
        _ => fact(big(n)) / fact(big(k)) / fact(big(n - k)),
    }
}

#[inline]
fn fact(n: BigUint) -> BigUint {
    factorial::fact_iter(n)
}

#[inline]
pub fn big(i: u128) -> BigUint {
    BigUint::from(i)
}
