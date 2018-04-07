#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate primer;

const USAGE: &'static str = "
Usage:
    prime_sieve (--count=N | --bound=N) [--sieve=METHOD]
    prime_sieve --help
    prime_sieve --version

Options:
    -h --help           Show help.
    -v --version        Show version.
    -b --bound N        Number of intergers to check for primality.
    -c --count N        Number of primes to generate.
    -m --method METHOD  Method used to generate the primes [default: eratosthenes].
    -f --format FORMAT  Format used to print the primes [default: compact].
";

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize)]
struct Args {
    flag_bound: Option<usize>,
    flag_count: Option<usize>,
    flag_method: Method,
    flag_format: Format,
}

#[derive(Debug, Deserialize)]
enum Method {
    Eratosthenes,
}

#[derive(Debug, Deserialize)]
enum Format {
    Compact,
}

fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|docopt| {
            docopt
                .help(true)
                .version(Some(String::from(VERSION)))
                .deserialize()
        })
        .unwrap_or_else(|error| error.exit());

    // TODO: handle both bound and count arguments
    let bound = args.flag_bound.unwrap();

    let sieve = primer::Sieve::new(bound);
    for prime in sieve.iter() {
        print!("{}, ", prime);
    }
    println!();
}
