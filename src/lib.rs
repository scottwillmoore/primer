pub struct Prime {
    sieve: Vec<bool>,
    current: usize,
}

impl Iterator for Prime {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let prime = self.current + 2;

        let sieve: Vec<bool> = self.sieve
            .iter()
            .enumerate()
            .map(|(index, &value)| {
                if value {
                    (index + 2) % prime != 0
                } else {
                    false
                }
            })
            .collect();
        self.sieve = sieve;

        println!("{:#?}", &self.sieve);

        let (next, _) = self.sieve
            .iter()
            .enumerate()
            .find(|&(_, &value)| value == true)
            .unwrap();
        self.current = next;

        if next * next > self.sieve.len() * self.sieve.len() {
            None
        } else {
            Some(prime)
        }
    }
}

pub fn generate(size: usize) -> Prime {
    let sieve = vec![true; size];

    Prime { sieve, current: 0 }
}
