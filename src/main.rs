fn main() {
    println!("Solution 1:{}", multiples_of_three_and_five(1000));
    println!("Solution 2:{}", even_fibonacci_sum(4000000));
    println!("Solution 3:{}", largest_prime_factor(600851475143));
    println!("Solution 4:{:?}", largest_palindromic_product());
    println!("Solution 5:{}", smallest_divisible_by_all(20));
}

fn multiples_of_three_and_five(n: u32) -> u32 {
    let mut sum = 0;
    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

fn even_fibonacci_sum(n: u32) -> u32 {
    let mut sum = 0;
    let mut a = 1;
    let mut b = 2;
    while b < n {
        if b % 2 == 0 {
            sum += b;
        }
        let c = a + b;
        a = b;
        b = c;
    }
    sum
}

// Return the largest prime factor of
// the number n
fn largest_prime_factor(n: u64) -> u64 {
    let mut i = 2;
    let mut largest = 2;
    let mut product = n;
    while product != 1 {
        if product % i == 0 {
            product /= i;
            largest = i;
        } else {
            i += 1;
        }
    }
    largest
}

const PALINDROME_MIN: u32 = 10000;
const PALINDROME_MAX: u32 = 998001;
fn largest_palindromic_product() -> Option<u32> {
    for n in (PALINDROME_MIN..PALINDROME_MAX).rev() {
        if is_palindrome(n) && is_product_of_two_three_digit_numbers(n) {
            return Some(n);
        }
    }
    None
}

fn is_palindrome(n: u32) -> bool {
    let mut temp = n;
    let mut reversed = 0;
    while temp > 0 {
        reversed *= 10;
        reversed += temp % 10;
        temp /= 10;
    }
    n == reversed
}

fn is_product_of_two_three_digit_numbers(n: u32) -> bool {
    for i in 100..1000 {
        if n % i == 0 && n / i <= 999 && n / i >= 100 {
            println!("{} / {} = {}", n, i, n / i);
            return true;
        }
    }
    false
}

fn smallest_divisible_by_all(n: u64) -> u64 {
    let mut i = n;
    loop {
        for j in 2..(n + 1) {
            if i % j != 0 {
                i += n;
                break;
            }
            if j == n {
                return i;
            }
        }
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(121));
        assert!(!is_palindrome(123));
        assert!(is_palindrome(12321));
        assert!(!is_palindrome(12345));
        assert!(!is_palindrome(123456));
        assert!(!is_palindrome(1234567));
        assert!(is_palindrome(123454321));
    }
    #[test]
    fn test_is_product_of_two_three_digit_numbers() {
        assert!(is_product_of_two_three_digit_numbers(18125));
    }
}
