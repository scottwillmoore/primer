extern crate prime_sieve;

fn main() {
    for prime in prime_sieve::generate(100) {
        print!("{}, ", prime);
    }
    println!();
}
