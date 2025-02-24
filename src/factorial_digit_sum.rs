use std::num::Saturating;

fn factorial(n: u128) -> u128 {
    let mut product: Saturating<u128> = Saturating(1);
    for i in 2..(n + 1) {
        let new_product = product * Saturating(i);
        if new_product == product {
            println!("Saturated at {i}");
            break;
        }
        product = new_product;
    }
    println!("{product}");
    product.0
}

fn factorial_sum() {
    let mut digits: [u32; 1000] = [0; 1000];
    let mut first = 0;
    let mut last = 0;
    let mut sum = 0;
    digits[0] = 1;

    for n in 2..1000 {
        let mut carry = 0;
        for m in first..last + 1 {
            carry = digits[m] * n + carry;
            digits[m] = carry % 10000;
            if m == first && carry % 10000 == 0 {
                first += 1;
            }
            carry /= 10000;
        }
        if carry == 0 {
            last = last + 1;
            digits[last] = carry;
        }
    }

    for x in first..last + 1 {
        sum += digits[x] % 10
            + (digits[x] / 10) % 10
            + (digits[x] / 100) % 10
            + (digits[x] / 1000) % 10;
    }
    println!("Factorial Digit Sum(1000): {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve() {
        factorial_sum();
    }

    #[test]
    fn div() {
        let i: u32 = 2 / 10000;

        println!("{i}");
    }
}
