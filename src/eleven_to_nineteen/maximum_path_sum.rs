// Assume input is a vector of ints sequenced from the
// bottom of the pyramid to the top
fn calculate_max_sum(input: &mut [Vec<u64>]) -> u64 {
    let mut sum_row: Vec<u64> = input[0].clone();
    let mut new_sum_row: Vec<u64> = Vec::new();
    for i in 1..input.len() {
        for j in 0..input[i].len() {
            if sum_row[j] > sum_row[j + 1] {
                new_sum_row.push(sum_row[j] + input[i][j]);
            } else {
                new_sum_row.push(sum_row[j + 1] + input[i][j]);
            }
        }
        sum_row = new_sum_row;
        new_sum_row = Vec::new();
    }
    sum_row.first().unwrap().clone()
}

fn parse_input(input: Vec<&str>) -> Vec<Vec<u64>> {
    input.iter().rev().map(parse_line).collect()
}

fn parse_line(line: &&str) -> Vec<u64> {
    line.split(' ')
        .map(|s| s.to_string().parse::<u64>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = &"1 2 3 4 05 6 7 8 9";
        let expected: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(expected, parse_line(input));
    }
    #[test]
    fn test_parse_input() {
        let input = vec!["1", "2 3", "4 5 6"];
        let expected: Vec<Vec<u64>> = vec![vec![4, 5, 6], vec![2, 3], vec![1]];

        assert_eq!(expected, parse_input(input));
    }

    #[test]
    fn test_calculate_sum() {
        let mut input: Vec<Vec<u64>> = vec![vec![4, 5, 6], vec![2, 3], vec![1]];
        let expected = 10;

        assert_eq!(expected, calculate_max_sum(&mut input));
    }

    #[test]
    fn solve() {
        let mut input = vec![
            "75",
            "95 64",
            "17 47 82",
            "18 35 87 10",
            "20 04 82 47 65",
            "19 01 23 75 03 34",
            "88 02 77 73 07 63 67",
            "99 65 04 28 06 16 70 92",
            "41 41 26 56 83 40 80 70 33",
            "41 48 72 33 47 32 37 16 94 29",
            "53 71 44 65 25 43 91 52 97 51 14",
            "70 11 33 28 77 73 17 78 39 68 17 57",
            "91 71 52 38 17 14 91 43 58 50 27 29 48",
            "63 66 04 68 89 53 67 30 73 16 69 87 40 31",
            "04 62 98 27 23 09 70 98 73 93 38 53 60 04 23",
        ];
        let mut sum_input = parse_input(input);
        let solution = calculate_max_sum(&mut sum_input);

        println!("Solution 19: {solution}");
    }
}
