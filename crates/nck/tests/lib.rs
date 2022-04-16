#[cfg(test)]
mod tests {
    use nck::*;

    #[test]
    fn test_n_choose_k() {
        assert_eq!(n_choose_k(3, 0), bigu128(1));
        assert_eq!(n_choose_k(3, 1), bigu128(3));
        assert_eq!(n_choose_k(3, 2), bigu128(3));
        assert_eq!(n_choose_k(30, 3), bigu128(4060))
    }

    #[test]
    fn test_no_overflow_on_trivial_input() {
        assert_eq!(n_choose_k(50, 1), bigu128(50));
        assert_eq!(n_choose_k(90, 1), bigu128(90));
        assert_eq!(n_choose_k(35, 34), bigu128(35));
        assert_eq!(n_choose_k(10000, 0), bigu128(1));
        assert_eq!(n_choose_k(10000, 10000), bigu128(1));
    }

    #[test]
    fn test_no_overflow() {
        assert_eq!(n_choose_k(35, 2), bigu128(595));
    }
}
