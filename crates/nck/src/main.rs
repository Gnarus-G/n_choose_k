use clap::Parser;
use n_choose_k::*;
use std::process;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The cardinality of the set in question.
    #[clap(name = "n")]
    n: Option<u128>,

    /// The amount to choose.
    #[clap(name = "k")]
    k: Option<u128>,

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

    let k = args.k.unwrap_or(1);

    match args.cache {
        true => println!("{:?}", n_choose_k(n, k)),
        false => match args.multi {
            true => println!("{:?}", n_choose_k_multi_threaded(n, k)),
            false => println!("{:?}", n_choose_k(n, k)),
        },
    }
}
