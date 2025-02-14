use crate::prime::is_prime;
pub fn nth_prime(n: u64) -> u64 {
    let mut count = 0;
    let mut i = 2;
    loop {
        if is_prime(i) {
            count += 1;
            if count == n {
                return i;
            }
        }
        i += 1;
    }
}
