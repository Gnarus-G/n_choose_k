use num::BigUint;

pub fn n_choose_k(n: u128, k: u128) -> BigUint {
    match k {
        0 => bigu128(1),
        1 => bigu128(n),
        k if k == n => bigu128(1),
        k if n - k == 1 => bigu128(n),
        _ => fact(bigu128(n)) / fact(bigu128(k)) / fact(bigu128(n - k)),
    }
}

fn fact(n: BigUint) -> BigUint {
    match n {
        n if is_zero_or_1(&n) => bigu128(1),
        _ => {
            let mut i = bigu128(1);
            let mut ret = bigu128(1);

            while i < n {
                ret *= &i;
                i += 1u128;
            }
            ret * n
        }
    }
}

#[inline]
fn is_zero_or_1(n: &BigUint) -> bool {
    bigu128(0) == *n || bigu128(1) == *n
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
