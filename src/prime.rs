use crate::benchmark;

pub fn is_prime(n: u64) -> bool {
    let top = (n as f64).sqrt() as u64;
    for i in 2..(top + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

struct Entry {
    n: u32,
    is_composite: bool,
}

pub fn generate_primes_up_to(n: u32) -> Vec<u32> {
    let mut marked: Vec<Entry> = {
        (2..(n + 1))
            .map(|x| Entry {
                n: x,
                is_composite: false,
            })
            .collect()
    };
    let mut primes: Vec<u32> = Vec::new();
    let mut p = 2;
    loop {
        marked.iter_mut().for_each(|e| {
            if e.n % p == 0 && e.n != p {
                e.is_composite = true;
            }
        });
        if let Some(next_prime) = marked.iter().find_map(|e| match e.is_composite {
            true => None,
            false if e.n <= p => None,
            false => Some(e.n),
        }) {
            primes.push(p);
            p = next_prime;
        } else {
            break;
        }
    }
    primes
}

pub fn generate_primes_up_to_2(n: u32) -> Vec<u32> {
    let mut naturals: Vec<u32> = (2..(n + 1)).collect();
    let mut p = 2;

    loop {
        naturals = naturals
            .iter()
            .filter_map(|x| {
                if *x == p || *x % p != 0 {
                    Some(*x)
                } else {
                    None
                }
            })
            .collect();
        if let Some(next_prime) = naturals.iter().find(|x| **x > p) {
            p = *next_prime;
        } else {
            break;
        }
    }

    naturals
}

pub fn benchmark() {
    // benchmark::run("Original", || {
    //     generate_primes_up_to(2000000);
    // });
    // benchmark::run("2nd version", || {
    //     generate_primes_up_to_2(2000000);
    // });
    benchmark::run("3rd version", || {
        generate_primes_up_to_3(2000000);
    });
}

pub fn generate_primes_up_to_3(n: u32) -> Vec<u32> {
    let mut naturals: Vec<bool> = (2..(n + 1)).map(|x| false).collect();
    let mut p = 2;

    loop {
        naturals
            .iter_mut()
            .enumerate()
            .for_each(|(i, is_composite)| {
                let n = i + 2;
                if n != p && n % p == 0 {
                    *is_composite = true;
                }
            });
        if let Some((next_prime_index, _)) = naturals
            .iter()
            .enumerate()
            .find(|(i, is_composite)| i + 2 > p && !(**is_composite))
        {
            p = next_prime_index + 2;
        } else {
            break;
        }
    }

    naturals
        .iter()
        .enumerate()
        .filter_map(|(i, is_composite)| {
            if !*is_composite {
                Some((i + 2) as u32)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(generate_primes_up_to_3(10), vec![2, 3, 5, 7])
    }
    #[test]
    fn case2() {
        assert_eq!(
            generate_primes_up_to_3(50),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
        );
    }
    #[test]
    fn case3() {
        let primes = generate_primes_up_to_3(200000);
        primes.iter().for_each(|n| println!("{n}"));
    }
}
