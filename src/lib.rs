#![feature(iterator_step_by)]
#![feature(test)]

extern crate test;

pub struct Sieve {
    is_prime: Vec<bool>,
    upper_bound: usize,
}

impl Sieve {
    pub fn new(upper_bound: usize) -> Sieve {
        assert!(upper_bound > 1);

        let mut is_prime = vec![true; upper_bound];

        is_prime[0] = false;
        is_prime[1] = false;

        let limit = (upper_bound as f64).sqrt() as usize;
        for i in 2..(limit + 1) {
            if is_prime[i] {
                let i_squared = i.pow(2);
                for j in (i_squared..upper_bound).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }

        Sieve {
            is_prime,
            upper_bound,
        }
    }

    pub fn iter<'a>(&'a self) -> Box<Iterator<Item = usize> + 'a> {
        Box::new(
            self.is_prime
                .iter()
                .enumerate()
                .filter(|&(_, is_prime)| *is_prime)
                .map(|(prime, _)| prime),
        )
    }

    pub fn upper_bound(&self) -> usize {
        self.upper_bound
    }

    pub fn is_prime(&self, n: usize) -> Option<bool> {
        self.is_prime.iter().nth(n).map(|is_prime| *is_prime)
    }

    pub fn nth_prime(&self, n: usize) -> Option<usize> {
        self.is_prime
            .iter()
            .enumerate()
            .filter(|&(_, is_prime)| *is_prime)
            .nth(n)
            .map(|(prime, _)| prime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn first_ten_primes() {
        let sieve = Sieve::new(30);
        let primes: Vec<_> = sieve.iter().collect();
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn first_ten_composites() {
        let sieve = Sieve::new(20);
        let primes: Vec<_> = sieve.iter().collect();
        let composites = vec![1, 4, 6, 8, 9, 10, 12, 14, 15, 16];

        for composite in &composites {
            assert!(!primes.contains(composite));
        }
    }

    #[test]
    fn assorted_large_primes() {
        let sieve = Sieve::new(1_000_000);
        let primes: Vec<_> = sieve.iter().collect();
        let validated_primes = vec![557, 3_677, 7_927, 27_337, 59_357, 128_189, 611_921, 882_313];

        for validated_prime in &validated_primes {
            assert!(primes.contains(validated_prime));
        }
    }

    #[test]
    fn assorted_large_composites() {
        let sieve = Sieve::new(1_000_000);
        let primes: Vec<_> = sieve.iter().collect();
        let composites = vec![275, 1_056, 6_084, 11_697, 56_806, 159_815, 419_746, 800_867];

        for composite in &composites {
            assert!(!primes.contains(composite));
        }
    }

    #[bench]
    fn sum_primes_under_two_million(b: &mut Bencher) {
        b.iter(|| {
            let sieve = Sieve::new(2_000_000);
            let sum: usize = sieve.iter().sum();

            assert_eq!(sum, 142_913_828_922);
        });
    }
}
