pub mod utils;

use std::{hash::Hash, marker::PhantomData};

use num::{range_inclusive, BigUint, ToPrimitive, Unsigned};
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
pub fn factorial(n: BigUint) -> BigUint {
    range_inclusive(big(1), n).product()
}

#[inline(always)]
fn big(i: u128) -> BigUint {
    BigUint::from(i)
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use crate::{
        utils::utils::{FactorialsHashMap, NoCacheCache},
        *,
    };

    #[test]
    fn test_fact_simple() {
        assert_eq!(factorial(big(5)), big(120));
        assert_eq!(factorial(big(3)), big(6));
    }

    #[test]
    fn test_fact_no_cache() {
        let mut cf = CachedFactorials::new(NoCacheCache::new());
        assert_eq!(cf.get(big(5)), big(120));

        assert_eq!(cf.cache.get(big(4)), None);

        assert_eq!(cf.get(big(3)), big(6));
    }

    #[test]
    fn test_cached_run() {
        let mut cf = CachedFactorials::new(FactorialsHashMap::new());
        assert_eq!(cf.get(big(5)), big(120));

        assert_eq!(cf.cache.get(big(4)), Some(big(24)));

        assert_eq!(cf.get(big(3)), big(6));

        let then = Instant::now();
        cf.get(big(25000));
        cf.get(big(25000));
        assert!(Instant::now().duration_since(then) < Duration::from_secs(2));
    }
}
