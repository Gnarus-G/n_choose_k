pub mod utils;

use std::{hash::Hash, iter::Product, marker::PhantomData};

use num::{one, range_inclusive, ToPrimitive, Unsigned};
use utils::utils::{CachedFactorials, FactorialsCache};

impl<I: Hash + Unsigned + Eq + PartialOrd + Clone + ToPrimitive, C: FactorialsCache<I>>
    CachedFactorials<I, C>
{
    pub fn new(cache: C) -> Self {
        CachedFactorials {
            cache,
            phantom: PhantomData,
        }
    }
    pub fn get(&mut self, n: I) -> I {
        match self.cache.get(n.clone()) {
            Some(v) => v.clone(),
            None => range_inclusive(self.cache.last_set(), n)
                .reduce(|x, y| {
                    let p = x * y.clone();
                    self.cache.set(y.clone(), p.clone());
                    p
                })
                .unwrap(),
        }
    }
}

#[inline]
pub fn factorial<I: Hash + Unsigned + Eq + PartialOrd + Clone + ToPrimitive + Product>(n: I) -> I {
    range_inclusive(one(), n).product()
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use num::BigUint;

    use crate::{
        utils::utils::{FactorialsHashMap, NoCacheCache},
        *,
    };

    #[test]
    fn test_fact_simple() {
        assert_eq!(factorial(5usize), (120usize));
        assert_eq!(factorial(3usize), (6usize));
    }

    #[test]
    fn test_fact_no_cache() {
        let mut cf = CachedFactorials::new(NoCacheCache::new());
        assert_eq!(cf.get(5usize), (120usize));

        assert_eq!(cf.cache.get(4usize), None);

        assert_eq!(cf.get(3usize), (6usize));
    }

    #[test]
    fn test_cached_run() {
        let mut cf = CachedFactorials::new(FactorialsHashMap::new());
        assert_eq!(cf.get(5usize), (120usize));

        assert_eq!(cf.cache.get(4usize), Some(24usize));

        assert_eq!(cf.get(3usize), (6usize));

        let mut cf = CachedFactorials::new(FactorialsHashMap::new());
        let then = Instant::now();
        cf.get(BigUint::from(25000u128));
        cf.get(BigUint::from(25000u128));
        assert!(Instant::now().duration_since(then) < Duration::from_secs(2));
    }
}
