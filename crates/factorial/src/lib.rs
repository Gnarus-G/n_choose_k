use num::{range_inclusive, BigUint};

#[inline]
pub fn fact_iter(n: BigUint) -> BigUint {
    range_inclusive(big(1), n).product()
}

pub fn fact_recur(n: BigUint) -> BigUint {
    let is_zero_or_1 = big(0) == n || big(1) == n;

    return if is_zero_or_1 {
        big(1)
    } else {
        &n * fact_recur(&n - big(1))
    };
}

#[inline(always)]
fn big(i: u128) -> BigUint {
    BigUint::from(i)
}

#[test]
#[cfg(test)]
fn test_fact_simple() {
    assert_eq!(fact_recur(big(5)), big(120));
    assert_eq!(fact_recur(big(3)), big(6));

    assert_eq!(fact_iter(big(5)), big(120));
    assert_eq!(fact_iter(big(3)), big(6))
}

#[test]
#[cfg(test)]
fn test_fact_iter() {
    fact_iter(big(99999)); //fact_recur(big(99999)) would cause stack overflow and crash, when implemented with recursion
}
