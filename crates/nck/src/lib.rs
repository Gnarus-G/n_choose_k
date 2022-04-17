use std::thread;

use factorial::{
    factorial,
    utils::utils::{CachedFactorials, FactorialsCache, NoCacheCache},
};
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

pub fn n_choose_k_multi_threaded(n: u128, k: u128) -> BigUint {
    match k {
        0 => big(1),
        1 => big(n),
        k if k == n => big(1),
        k if n - k == 1 => big(n),
        _ => {
            let first = thread::spawn(move || factorial(big(n)));
            let second = thread::spawn(move || factorial(big(k)));
            let last = thread::spawn(move || factorial(big(n - k)));

            let numerator = first.join().unwrap();
            let div_1 = second.join().unwrap();
            let div_2 = last.join().unwrap();

            numerator / div_1 / div_2
        }
    }
}

#[inline]
pub fn big(i: u128) -> BigUint {
    BigUint::from(i)
}
