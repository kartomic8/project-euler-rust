pub fn is_prime(n: u64) -> bool {
    let top = (n as f64).sqrt() as u64;
    for i in 2..(top + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub struct PrimeGenerator {
    value: u64,
}

impl PrimeGenerator {
    pub fn new() -> PrimeGenerator {
        PrimeGenerator { value: 2 }
    }
}

impl Iterator for PrimeGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.value;
        let mut next_potential_prime = value + 1;
        loop {
            if is_prime(next_potential_prime) {
                self.value = next_potential_prime;
                break;
            }
            next_potential_prime += 1;
        }
        Some(value)
    }
}

#[cfg(test)]
mod prime_generator_test {
    use super::*;
    #[test]
    fn generate_primes() {
        let prime_generator = PrimeGenerator { value: 2 };
        prime_generator.take(12).for_each(|p| println!("{p}"));
    }
}
