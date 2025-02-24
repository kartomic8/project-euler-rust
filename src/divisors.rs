pub fn proper_divisors_of(n: i32) -> Vec<i32> {
    let limit: i32 = n / 2 + 1;
    let mut divisors = Vec::new();

    for i in 1..limit {
        if n % i == 0 {
            divisors.push(i);
        }
    }

    divisors
}

pub fn sum_of_proper_divisors_of(n: i32) -> i32 {
    let limit: i32 = n / 2 + 1;
    let mut sum = 0;
    for i in 1..limit {
        if n % i == 0 {
            sum += i;
        }
    }
    sum
}
