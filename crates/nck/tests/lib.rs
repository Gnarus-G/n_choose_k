#[cfg(test)]
mod tests {
    use std::time::{Instant, Duration};

    use factorial::utils::utils::{FactorialsCache, FactorialsHashMap, NoCacheCache};
    use n_choose_k::*;
    use num::BigUint;

    #[test]
    fn test_n_choose_k() {
        let n_choose_k = |n, k| n_choose_k(n, k, NoCacheCache::new());
        assert_eq!(n_choose_k(3, 0), 1);
        assert_eq!(n_choose_k(3, 1), 3);
        assert_eq!(n_choose_k(3, 2), 3);
        assert_eq!(n_choose_k(10, 3), 120)
    }

    #[test]
    fn test_n_choose_k_multi_threaded() {
        assert_eq!(n_choose_k_multi_threaded(3, 0), 1);
        assert_eq!(n_choose_k_multi_threaded(3, 1), 3);
        assert_eq!(n_choose_k_multi_threaded(3, 2), 3);
        assert_eq!(n_choose_k_multi_threaded(10, 3), 120);
    }

    #[test]
    fn test_no_overflow_on_trivial_input() {
        let n_choose_k = |n, k| n_choose_k(n, k, NoCacheCache::new());
        assert_eq!(n_choose_k(50, 1), 50);
        assert_eq!(n_choose_k(90, 1), 90);
        assert_eq!(n_choose_k(35, 34), 35);
        assert_eq!(n_choose_k(10000, 0), 1);
        assert_eq!(n_choose_k(10000, 10000), 1);
    }

    #[test]
    fn test_no_overflow() {
        let n_choose_k = |n, k| n_choose_k(n, k, FactorialsHashMap::new());

        let then = Instant::now();
        n_choose_k(BigUint::from(25000u128), BigUint::from(2u8));
        n_choose_k(BigUint::from(25000u128), BigUint::from(2u8));
        assert!(Instant::now().duration_since(then) < Duration::from_secs(5));
    }
}
