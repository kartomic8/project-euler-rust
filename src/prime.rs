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

pub fn is_prime(n: u64) -> bool {
    let top = (n as f64).sqrt() as u64;
    for i in 2..(top + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
