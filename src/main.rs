extern crate prime_sieve;

fn main() {
    for prime in prime_sieve::generate().take(10) {
        println!("{}", prime);
    }
}
