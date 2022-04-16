use nck::n_choose_k;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    let n = args
        .get(1)
        .expect("Need to enter the value for n.")
        .parse()
        .expect("Couldn't parse number n from the arguments given.");

    let k = args
        .get(2)
        .map(|s| {
            s.parse()
                .expect("Couldn't parse number k from the arguments given.")
        })
        .unwrap_or_default();

    println!("{:?}", n_choose_k(n, k));
}
