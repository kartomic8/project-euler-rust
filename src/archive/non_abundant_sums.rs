use crate::divisors::sum_of_proper_divisors_of;

fn is_abundant(n: i32) -> bool {
    sum_of_proper_divisors_of(n) > n
}

//Solution runtime: 165.67s
fn mark_sum_of_two_abundants() -> i32 {
    const LIMIT: i32 = 28124;
    let abundant_numbers: Vec<i32> = (0..LIMIT).filter(|n| is_abundant(*n)).collect();
    let mut sum_of_two_abundants: Vec<i32> = Vec::new();

    for i in 0..abundant_numbers.len() {
        for j in i..abundant_numbers.len() {
            let sum = abundant_numbers[i] + abundant_numbers[j];
            if sum < LIMIT {
                sum_of_two_abundants.push(sum);
            }
        }
    }

    (0..LIMIT)
        .filter(|n| !sum_of_two_abundants.iter().any(|x| *x == *n))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_abundant() {
        let test_cases = vec![(12, true), (11, false), (10, false)];

        for (n, expected) in test_cases {
            assert_eq!(expected, is_abundant(n));
        }
    }

    #[test]
    fn solve() {
        let answer = mark_sum_of_two_abundants();
        println!("Solution to Non-Abundant Sums: {answer}");
    }
}
