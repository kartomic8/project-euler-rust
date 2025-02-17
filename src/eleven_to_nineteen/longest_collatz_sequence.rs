use std::collections::HashMap;

pub fn solve() -> u64 {
    longest_collatz_sequence(1_000_000)
}

fn longest_collatz_sequence(n: u64) -> u64 {
    let mut cache: HashMap<u64, u64> = HashMap::new();
    let mut largest_sequence = 0;
    let mut largest_index = 0;
    for i in 1..(n + 1) {
        let len = generate_collatz_sequence_length(i, &mut cache);
        if len > largest_sequence {
            largest_sequence = len;
            largest_index = i;
        }
    }
    largest_index
}

fn generate_collatz_sequence_length(n: u64, prev: &mut HashMap<u64, u64>) -> u64 {
    if let Some(len) = prev.get(&n) {
        *len
    } else if n == 1 {
        prev.insert(1, 1);
        1
    } else if n % 2 == 0 {
        let len = 1 + generate_collatz_sequence_length(n / 2, prev);
        prev.insert(n, len);
        len
    } else {
        let len = 1 + generate_collatz_sequence_length(n * 3 + 1, prev);
        prev.insert(n, len);
        len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_collatz_sequence_length() {
        let cases = [(1, 1), (2, 2), (3, 8), (4, 3), (5, 6), (6, 9), (7, 17)];

        let mut cache: HashMap<u64, u64> = HashMap::new();
        for (input, expected) in cases {
            let len = generate_collatz_sequence_length(input, &mut cache);
            assert_eq!(expected, len);
        }
    }

    #[test]
    fn test_find_largest_collatz_sequence() {
        let largest = longest_collatz_sequence(7);
        assert_eq!(largest, 7);
    }
}
