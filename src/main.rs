use std::env::args;
use nchooserust::*;

fn main() {
    let args: Vec<String> = args().collect();

    let n = args.get(1).map(|s| s.parse().unwrap_or(0)).unwrap_or(0);
    let k = args.get(2).map(|s| s.parse().unwrap_or(0)).unwrap_or(0);

    println!("{:?}", n_choose_k(n, k));
}
