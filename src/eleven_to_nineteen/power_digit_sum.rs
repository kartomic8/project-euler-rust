pub fn solve() {
    let mut product = "1".to_owned();

    for _ in 1..1001 {
        product = double(product.as_str());
    }

    println!("{}", sum_digits(product.as_str()));
}

fn sum_digits(input: &str) -> u64 {
    input.chars().map(|c| c.to_digit(10).unwrap() as u64).sum()
}

fn double(a: &str) -> String {
    let carry = 0;
    let result = "".to_owned();

    let (carry, product) = a
        .chars()
        .rev()
        .fold((carry, result), |(carry, result), digit| {
            let digit = digit.to_digit(10).unwrap();
            let product_chunk = digit * 2 + carry;
            let digit_to_add = product_chunk % 10;
            let carry = (product_chunk - digit_to_add) / 10;
            let result = (product_chunk % 10).to_string() + &result;
            (carry, result)
        });
    if carry != 0 {
        carry.to_string() + &product
    } else {
        product
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmark;

    #[test]
    fn test_double() {
        let a = "3";

        assert_eq!("6", double(a))
    }

    #[test]
    fn test_manual_multiple() {
        assert_eq!("24", double("12"));
    }

    #[test]
    fn test_carry() {
        assert_eq!("110", double("55"));
    }

    #[test]
    fn benchmark() {
        benchmark::run("Solution 16", || {
            let result = double("1000000000000000000000000000000000");
            println!("{result}");
        })
    }

    #[test]
    fn test_solve() {
        solve();
    }
}
