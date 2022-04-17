#[cfg(test)]
mod tests {
    use n_choose_k::*;

    #[test]
    fn test_n_choose_k() {
        assert_eq!(n_choose_k(3, 0), big(1));
        assert_eq!(n_choose_k(3, 1), big(3));
        assert_eq!(n_choose_k(3, 2), big(3));
        assert_eq!(n_choose_k(30, 3), big(4060))
    }

    #[test]
    fn test_no_overflow_on_trivial_input() {
        assert_eq!(n_choose_k(50, 1), big(50));
        assert_eq!(n_choose_k(90, 1), big(90));
        assert_eq!(n_choose_k(35, 34), big(35));
        assert_eq!(n_choose_k(10000, 0), big(1));
        assert_eq!(n_choose_k(10000, 10000), big(1));
    }

    #[test]
    fn test_no_overflow() {
        assert_eq!(n_choose_k(35, 2), big(595));
    }
}
