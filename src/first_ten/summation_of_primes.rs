use crate::benchmark;
use crate::prime::{self, generate_primes_up_to_2};

pub fn summation_of_primes() -> u64 {
    let mut sum = 0;
    for i in 2..2000000 {
        if prime::is_prime(i) {
            sum += i;
        }
    }
    sum
}

pub fn summation_of_primes_fast() -> u64 {
    generate_primes_up_to_2(2000000)
        .iter()
        .map(|x| *x as u64)
        .fold(u64::MIN, |x: u64, acc: u64| x + acc)
}

pub fn benchmark() {
    benchmark::run("Original", || {
        summation_of_primes;
    });
    benchmark::run("Optimized", || {
        summation_of_primes_fast;
    });
}
