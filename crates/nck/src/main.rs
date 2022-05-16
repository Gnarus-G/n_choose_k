use clap::{ArgEnum, Parser};
use factorial::utils::utils::{FactorialsCache, FactorialsHashMap, NoCacheCache};
use n_choose_k::*;
use num::BigUint;
use std::io::{stdin, stdout, Write};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Interactive mode
    #[clap(short, long)]
    interactive: bool,

    /// The cardinality of the set in question.
    #[clap(name = "n", required_unless_present = "interactive")]
    n: Option<BigUint>,

    /// The amount to choose.
    #[clap(name = "k", required_unless_present = "interactive")]
    k: Option<BigUint>,

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

    #[inline]
    fn with_mode(mode: Mode, n: BigUint, k: BigUint) {
        match mode {
            Mode::NAIVE => println!("{:?}", n_choose_k_naive(n, k)),
            Mode::BLOAT => println!("{:?}", n_choose_k(n, k, NoCacheCache::new())),
            Mode::CACHE => println!("{:?}", n_choose_k(n, k, FactorialsHashMap::new())),
            Mode::MULTI => println!("{:?}", n_choose_k_multi_threaded(n, k)),
        };
    }

    if !args.interactive {
        let (n, k) = (args.n, args.k);
        return with_mode(args.mode, n.unwrap(), k.unwrap());
    }

    let mut buffer = String::new();

    #[inline]
    fn read_stdin_as_cli_args(buffer: &mut String) {
        buffer.push_str("nck "); //mock cli executable name
        stdin().read_line(buffer).ok();
        buffer.remove(buffer.len() - 1); // remove \n
    }

    loop {
        print!("\n>");
        stdout().flush().ok();
        read_stdin_as_cli_args(&mut buffer);
        match Args::try_parse_from(buffer.split(" ")) {
            Ok(args) => {
                let (n, k) = (args.n, args.k);
                with_mode(args.mode, n.unwrap(), k.unwrap());
            }
            Err(err) => err.print().expect("Something went wrong and we couldn't write the error"),
        };
        buffer.clear();
    }
}
