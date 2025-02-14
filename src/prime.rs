pub fn is_prime(n: u64) -> bool {
    let top = (n as f64).sqrt() as u64;
    for i in 2..(top + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
