use std::{hash::Hash, iter::Product, thread};

use factorial::{
    factorial,
    utils::utils::{CachedFactorials, FactorialsCache},
};
use num::{one, ToPrimitive, Unsigned};

pub fn n_choose_k<
    Int: Unsigned + Eq + Hash + Clone + From<usize> + PartialOrd + ToPrimitive,
    C: FactorialsCache<Int>,
>(
    n: Int,
    k: Int,
    cache: C,
) -> Int {
    let mut factorials = CachedFactorials::new(cache);

    match k {
        k if k.is_zero() => one(),
        k if k.is_one() => n,
        k if k == n => one(),
        k if (n.clone() - k.clone()).is_one() => n,
        _ => factorials.get(n.clone()) / factorials.get(k.clone()) / factorials.get(n - k),
    }
}

pub fn n_choose_k_multi_threaded<
    Int: Unsigned
        + Eq
        + Hash
        + Clone
        + From<usize>
        + PartialOrd
        + ToPrimitive
        + Send
        + Product
        + 'static,
>(
    n: Int,
    k: Int,
) -> Int {
    match k {
        k if k.is_zero() => one(),
        k if k.is_one() => n,
        k if k == n => one(),
        k if (n.clone() - k.clone()).is_one() => n,
        _ => {
            let n_minus_k = n.clone() - k.clone();
            let first = thread::spawn(move || factorial(n));
            let second = thread::spawn(move || factorial(k));
            let last = thread::spawn(move || factorial(n_minus_k));

            let numerator = first.join().unwrap();
            let div_1 = second.join().unwrap();
            let div_2 = last.join().unwrap();

            numerator / div_1 / div_2
        }
    }
}
