use std::{env::args, process};
use factorial::factorial;

fn main() {
    let x: num::BigUint = args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or_else(|| {
            eprintln!("Need a number!");
            process::exit(1)
        })
        .parse()
        .unwrap_or_else(|err| {
            eprintln!("Couldn't parse a number the given argument! \n ↪ {}", err);
            process::exit(1)
        });

    println!("{:?}", factorial(x));
}
