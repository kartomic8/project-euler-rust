use crate::prime;

pub fn summation_of_primes() -> u64 {
    let mut sum = 0;
    for i in 2..2000000 {
        if prime::is_prime(i) {
            sum += i;
        }
    }
    sum
}
