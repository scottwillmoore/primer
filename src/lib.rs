pub struct Prime {
    is_composite: Vec<bool>,
    current_prime: usize,
    upper_bound: usize,
}

impl Iterator for Prime {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let current_prime = self.current_prime;
        let mut prime_multiple = current_prime * 2;
        while prime_multiple <= self.upper_bound {
            self.is_composite[prime_multiple] = true;
            prime_multiple += current_prime;
        }

        let mut next_prime = current_prime + 1;
        while next_prime <= self.upper_bound {
            if !self.is_composite[next_prime] {
                break;
            }
            next_prime += 1;
        }

        if current_prime <= self.upper_bound {
            self.current_prime = next_prime;
            Some(current_prime)
        } else {
            None
        }
    }
}

pub fn generate(upper_bound: usize) -> Prime {
    Prime {
        is_composite: vec![false; upper_bound + 1],
        current_prime: 2,
        upper_bound,
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
