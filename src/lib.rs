#![feature(iterator_step_by)]

pub struct Prime {
    sieve: Vec<bool>,
    current: usize,
}

impl Prime {
    fn get_next_prime(&self, prime: usize) -> Option<usize> {
        self.sieve
            .iter()
            .enumerate()
            .skip(prime + 1)
            .find(|&(_, &is_composite)| !is_composite)
            .map(|(next_prime, _)| next_prime)
    }

    fn mark_prime_multiples(&mut self, prime: usize) {
        self.sieve
            .iter_mut()
            .step_by(prime)
            .skip(1)
            .for_each(|is_composite| *is_composite = true);
    }
}

impl Iterator for Prime {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let maybe_next_prime = self.get_next_prime(self.current);

        maybe_next_prime.and_then(|next_prime| {
            self.mark_prime_multiples(next_prime);
            self.current = next_prime;
            Some(next_prime)
        })
    }
}

pub fn generate(upper_bound: usize) -> Prime {
    Prime {
        sieve: vec![false; upper_bound + 1],
        current: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_ten_primes() {
        let primes: Vec<usize> = generate(30).take(10).collect();

        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn first_ten_composites() {
        let primes: Vec<usize> = generate(20).collect();
        let composites = vec![1, 4, 6, 8, 9, 10, 12, 14, 15, 16];

        for composite in &composites {
            assert!(!primes.contains(composite));
        }
    }

    #[test]
    fn assorted_large_primes() {
        let primes: Vec<usize> = generate(1_000_000).collect();
        let validated_primes = vec![557, 3_677, 7_927, 27_337, 59_357, 128_189, 611_921, 882_313];

        for validated_prime in &validated_primes {
            assert!(primes.contains(validated_prime));
        }
    }

    #[test]
    fn assorted_large_composites() {
        let primes: Vec<usize> = generate(20).collect();
        let composites = vec![275, 1_056, 6_084, 11_697, 56_806, 159_815, 419_746, 800_867];

        for composite in &composites {
            assert!(!primes.contains(composite));
        }
    }
}
