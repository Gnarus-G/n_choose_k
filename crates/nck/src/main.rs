use n_choose_k::*;
use std::{env::args, process::exit};

fn main() {
    let args: Vec<String> = args().collect();

    let [n, k] = parse_two_numbers(&args);

    println!("{:?}", n_choose_k(n, k));
}

fn parse_two_numbers(args: &[String]) -> [u128; 2] {
    let x = args
        .get(1)
        .unwrap_or_else(|| {
            eprintln!("Need to enter at least one number!");
            exit(1)
        })
        .parse()
        .unwrap_or_else(|err| {
            eprintln!(
                "Couldn't parse a number from the first argument! \n ↪ {}",
                err
            );
            exit(1)
        });

    let y = args
        .get(2)
        .map(|s| {
            s.parse().unwrap_or_else(|err| {
                eprintln!(
                    "Couldn't parse a number from the second argument! \n ↪ {}",
                    err
                );
                exit(1);
            })
        })
        .unwrap_or_default();

    [x, y]
}
