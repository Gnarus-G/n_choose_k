use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    let n = args.get(1).map(|s| s.parse().unwrap_or(0)).unwrap_or(0);
    let k = args.get(2).map(|s| s.parse().unwrap_or(0)).unwrap_or(0);

    println!("{:?}", n_choose_k(n, k));
}

fn n_choose_k(n: u128, k: u128) -> u128 {
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
