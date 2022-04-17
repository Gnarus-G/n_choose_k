use std::collections::HashMap;

use num::{range_inclusive, BigUint};

#[derive(Debug)]
pub struct CachedFactorials {
    cache: HashMap<BigUint, BigUint>,
}

impl CachedFactorials {
    pub fn new() -> Self {
        let mut cache = HashMap::new();
        cache.insert(big(0), big(1));
        cache.insert(big(1), big(1));
        cache.insert(big(2), big(2));
        CachedFactorials { cache }
    }
    pub fn get(&mut self, n: BigUint) -> BigUint {
        match self.cache.get(&n) {
            Some(v) => v.clone(),
            None => {
                let len: BigUint = self.cache.len().to_owned().into();
                let last_good_key = len - 1u8;

                range_inclusive(last_good_key, n)
                    .reduce(|x, y| {
                        let p = x * y.clone();
                        self.cache.insert(y, p.clone());
                        p
                    })
                    .unwrap()
            }
        }
    }
}

#[inline]
pub fn fact_iter(n: BigUint) -> BigUint {
    range_inclusive(big(1), n).product()
}

#[inline(always)]
fn big(i: u128) -> BigUint {
    BigUint::from(i)
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use crate::*;

    #[test]
    fn test_fact_simple() {
        assert_eq!(fact_iter(big(5)), big(120));
        assert_eq!(fact_iter(big(3)), big(6));
    }

    #[test]
    fn test_cached_run() {
        let mut cf = CachedFactorials::new();
        assert_eq!(cf.get(big(5)), big(120));

        assert_eq!(cf.cache.get(&big(4)), Some(&big(24)));

        assert_eq!(cf.get(big(3)), big(6));

        let then = Instant::now();
        cf.get(big(25000));
        cf.get(big(25000));
        assert!(Instant::now().duration_since(then) < Duration::from_secs(2));
    }
}
