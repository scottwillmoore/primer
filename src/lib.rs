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
        while prime_multiple < self.upper_bound {
            self.is_composite[prime_multiple] = true;
            prime_multiple += current_prime;
        }

        let mut next_prime = current_prime + 1;
        while next_prime < self.upper_bound {
            if !self.is_composite[next_prime] {
                break;
            }
            next_prime += 1;
        }

        if current_prime < self.upper_bound {
            self.current_prime = next_prime;
            Some(current_prime)
        } else {
            None
        }
    }
}

pub fn generate(upper_bound: usize) -> Prime {
    assert!(upper_bound > 0);

    Prime {
        is_composite: vec![false; upper_bound],
        current_prime: 2,
        upper_bound,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_10_primes() {
        let primes: Vec<usize> = generate(100).take(10).collect();

        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }
}
