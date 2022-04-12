#[cfg(test)]
mod tests {
    use nchooserust::n_choose_k;

    #[test]
    fn test_n_choose_k() {
        assert_eq!(n_choose_k(3, 0), 1);
        assert_eq!(n_choose_k(3, 1), 3);
        assert_eq!(n_choose_k(3, 2), 3);
        assert_eq!(n_choose_k(30, 3), 4060)
    }
}
