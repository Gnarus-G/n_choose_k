#[cfg(test)]
mod tests {
    use memo::{memo, Memo};
    use std::{
        thread,
        time::{Duration, Instant},
    };

    #[test]
    fn it_works() {
        let mut fact = Memo::new(|dur| thread::sleep(dur));
        let then = Instant::now();
        fact.value(Duration::from_secs(1));
        fact.value(Duration::from_secs(1));
        assert!(
            Instant::now().duration_since(then) < Duration::from_secs(2),
            "took too long!"
        )
    }

    #[test]
    fn it_still_works() {
        let expensive_function = &mut |dur| thread::sleep(dur);
        let mut memoized = memo(expensive_function);

        let then = Instant::now();
        memoized(Duration::from_secs(1));
        memoized(Duration::from_secs(1));
        memoized(Duration::from_secs(1));
        assert!(
            Instant::now().duration_since(then) < Duration::from_secs(2),
            "took too long!"
        )
    }
}
