#[cfg(test)]
mod tests {
    use memo::Memo;
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
}
