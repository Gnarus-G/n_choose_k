use clap::{ArgEnum, Parser};
use factorial::utils::utils::{FactorialsCache, FactorialsHashMap, NoCacheCache};
use n_choose_k::*;
use num::BigUint;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The cardinality of the set in question.
    #[clap(name = "n")]
    n: BigUint,

    /// The amount to choose.
    #[clap(name = "k")]
    k: BigUint,

    /// Pick a mode; each has a different impact on performance.
    #[clap(arg_enum, default_value = "naive")]
    mode: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Mode {
    NAIVE,
    BLOAT,
    MULTI,
    CACHE,
}

fn main() {
    let args = Args::parse();
    let (n, k) = (args.n, args.k);

    match args.mode {
        Mode::NAIVE => println!("{:?}", n_choose_k_naive(n, k)),
        Mode::BLOAT => println!("{:?}", n_choose_k(n, k, NoCacheCache::new())),
        Mode::CACHE => println!("{:?}", n_choose_k(n, k, FactorialsHashMap::new())),
        Mode::MULTI => println!("{:?}", n_choose_k_multi_threaded(n, k)),
    }
}
