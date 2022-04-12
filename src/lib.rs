pub fn n_choose_k(n: u128, k: u128) -> u128 {
    fact(n) / fact(k) / fact(n - k)
}

fn fact(n: u128) -> u128 {
    match n {
        0 => 1,
        1 => 1,
        2 => 2,
        _ => n * fact(n - 1),
    }
}
