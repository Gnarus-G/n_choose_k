use clap::Parser;
use factorial::utils::utils::{FactorialsCache, FactorialsHashMap, NoCacheCache};
use n_choose_k::*;
use num::BigUint;
use std::process;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The cardinality of the set in question.
    #[clap(name = "n")]
    n: Option<BigUint>,

    /// The amount to choose.
    #[clap(name = "k")]
    k: Option<BigUint>,

    /// Enable multithreading.
    #[clap(short, long)]
    multi: bool,

    /// Enable caching (also disables multithreading).
    #[clap(short, long)]
    cache: bool,
}

fn main() {
    let args = Args::parse();

    let n = args.n.unwrap_or_else(|| {
        eprintln!("Need to enter at least one number!");
        process::exit(1)
    });

    let k = args.k.unwrap_or(BigUint::from(1u8));

    match args.cache {
        true => println!("{:?}", n_choose_k(n, k, FactorialsHashMap::new())),
        false => match args.multi {
            true => println!("{:?}", n_choose_k_multi_threaded(n, k)),
            false => println!("{:?}", n_choose_k(n, k, NoCacheCache::new())),
        },
    }
}
