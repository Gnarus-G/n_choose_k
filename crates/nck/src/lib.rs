pub fn n_choose_k(n: u128, k: u128) -> u128 {
    match k {
        0 => 1,
        1 => n,
        k if k == n => 1,
        k if n - k == 1 => n,
        _ => fact(n) / fact(k) / fact(n - k),
    }
}

fn fact(n: u128) -> u128 {
    match n {
        0 | 1 => 1,
        2 => 2,
        _ => n * fact(n - 1),
    }
}
