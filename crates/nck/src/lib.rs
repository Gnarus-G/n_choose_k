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
    let is_zero_or_1 = bigu128(0) == n || bigu128(1) == n;

    return if is_zero_or_1 {
        bigu128(1)
    } else {
        &n * fact(&n - bigu128(1))
    };
}

#[inline]
pub fn bigu128(i: u128) -> BigUint {
    BigUint::from(i)
}

#[test]
#[cfg(test)]
fn test_fact_no_crash() {
    fact(bigu128(35)); //fact(35u128) would cause overflow and crash;
}
