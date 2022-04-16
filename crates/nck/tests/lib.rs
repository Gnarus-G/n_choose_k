#[cfg(test)]
mod tests {
    use nck::n_choose_k;

    #[test]
    fn test_n_choose_k() {
        assert_eq!(n_choose_k(3, 0), 1);
        assert_eq!(n_choose_k(3, 1), 3);
        assert_eq!(n_choose_k(3, 2), 3);
        assert_eq!(n_choose_k(30, 3), 4060)
    }

    #[test]
    fn test_no_overflow_on_trivial_input() {
        assert_eq!(n_choose_k(50, 1), 50);
        assert_eq!(n_choose_k(90, 1), 90);
        assert_eq!(n_choose_k(35, 34), 35);
        assert_eq!(n_choose_k(10000, 0), 1);
        assert_eq!(n_choose_k(10000, 10000), 1);
    }
}
