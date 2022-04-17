use factorial::utils::utils::{CachedFactorials, FactorialsCache, NoCacheCache};
use num::BigUint;

pub fn n_choose_k(n: u128, k: u128) -> BigUint {
    let mut factorials = CachedFactorials::new(NoCacheCache::new());

    match k {
        0 => big(1),
        1 => big(n),
        k if k == n => big(1),
        k if n - k == 1 => big(n),
        _ => factorials.get(big(n)) / factorials.get(big(k)) / factorials.get(big(n - k)),
    }
}

#[inline]
pub fn big(i: u128) -> BigUint {
    BigUint::from(i)
}
