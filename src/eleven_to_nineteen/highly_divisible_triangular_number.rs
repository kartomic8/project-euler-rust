use crate::prime::PrimeGenerator;

pub fn solve() -> u64 {
    let mut divisors = 0;
    let mut n: u64 = 0;
    let mut tri = 0;
    while divisors < 500 {
        tri = triangle(n);
        divisors = count_divisors(tri);
        n += 1;
    }
    tri
}

fn triangle(n: u64) -> u64 {
    n * (n + 1) / 2
}

fn count_divisors(n: u64) -> u64 {
    prime_factorize(n)
        .iter()
        .fold(1, |acc, (_, c)| acc * (c + 1))
}

fn prime_factorize(n: u64) -> Vec<(u64, u64)> {
    let mut product = n;
    let mut factorization: Vec<(u64, u64)> = Vec::new();
    let mut prime_gen = PrimeGenerator::new();
    let mut prime_factor = prime_gen.next().unwrap();
    let mut count = 0;
    while product > 1 {
        if product % prime_factor == 0 {
            count += 1;
            product /= prime_factor;
        } else {
            if count > 0 {
                factorization.push((prime_factor, count));
            }
            prime_factor = prime_gen.next().unwrap();
            count = 0;
        }
    }
    if count > 0 {
        factorization.push((prime_factor, count));
    }
    factorization
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_triangle() {
        let expected = [1, 3, 6, 10, 15, 21, 28];

        for i in 0..expected.len() {
            assert_eq!(expected[i] as u64, triangle((i + 1) as u64));
        }
    }

    #[test]
    fn test_prime_factorize() {
        let factors_of_32: Vec<(u64, u64)> = vec![(2, 5)];

        assert_eq!(factors_of_32, prime_factorize(32));
    }
    #[test]
    fn test_prime_factorize_2() {
        let factors_of_30: Vec<(u64, u64)> = vec![(2, 1), (3, 1), (5, 1)];

        assert_eq!(factors_of_30, prime_factorize(30));
    }

    #[test]
    fn test_count_divisors() {
        let cases = [(30, 8), (128, 8), (49, 3), (72, 12)];

        for (input, output) in cases {
            assert_eq!(output, count_divisors(input));
        }
    }
    #[test]
    fn test_solve() {
        println!("{}", solve());
    }
}
