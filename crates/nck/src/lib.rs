use num::{range_inclusive, BigUint};

pub fn n_choose_k(n: u128, k: u128) -> BigUint {
    match k {
        0 => bigu128(1),
        1 => bigu128(n),
        k if k == n => bigu128(1),
        k if n - k == 1 => bigu128(n),
        _ => fact(bigu128(n)) / fact(bigu128(k)) / fact(bigu128(n - k)),
    }
}

#[inline]
fn fact(n: BigUint) -> BigUint {
    range_inclusive(bigu128(1), n).product()
}

#[inline]
pub fn bigu128(i: u128) -> BigUint {
    BigUint::from(i)
}

#[test]
#[cfg(test)]
fn test_fact_simple() {
    assert_eq!(fact(bigu128(5)), bigu128(120))
}

#[test]
#[cfg(test)]
fn test_fact_no_crash() {
    fact(bigu128(35)); //fact(35u128) would cause multiply by overflow error and crash
    fact(bigu128(99999)); //fact(bigu128(99999)) would cause stack overflow and crash, when implemented with recursion
}
