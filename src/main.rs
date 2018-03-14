extern crate prime_sieve;

fn main() {
    for prime in prime_sieve::generate(15).take(5) {
        println!("{}", prime);
    }
}
