fn sum_amicable_numbers() {
    let mut cache: [Option<i32>; 10000] = [None; 10000];
    let mut sum = 0;
    for i in 0..10000 {
        let divisor_sum = sum_of_proper_divisors_of(i);
        cache[i as usize] = Some(divisor_sum);
        if divisor_sum < 10000 && cache[divisor_sum as usize].is_some() {
            if let Some(s) = cache[divisor_sum as usize] {
                sum += if s == i && i != divisor_sum {
                    i + divisor_sum
                } else {
                    0
                };
            }
        }
    }

    println!("Sum of amicable numbers: {sum}");
}
// fn amicable_pair(i: i32, cache: &mut [Option<(i32, i32)>]) -> Option<(i32, i32)> {
//     let divisors = proper_divisors_of(i);
//     let sum = divisors.iter().sum();

//     let divisors = proper_divisors_of(sum);
//     let sum2: i32 = divisors.iter().sum();
//     if sum2 == i {
//         let pair = (i, sum);
//         cache[i as usize] = Some(pair);
//         cache[sum as usize] = Some(pair);
//         Some(pair)
//     } else {
//         None
//     }
// }

fn sum_of_proper_divisors_of(n: i32) -> i32 {
    let limit: i32 = n / 2 + 1;
    let mut sum = 0;
    for i in 1..limit {
        if n % i == 0 {
            sum += i;
        }
    }
    sum
}

fn digit_sum(n: i32) -> i32 {
    let mut m = n;
    let mut sum = 0;

    while m > 0 {
        sum += m % 10;
        m /= 10;
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_proper_divisors() {
        let result = sum_of_proper_divisors_of(220);
        let expected = vec![1, 2, 3, 5, 10, 11, 20, 22, 44, 55, 110].iter().sum();
        assert_eq!(result, expected);
    }

    // #[test]
    // fn test_amicable_pair() {
    //     let mut cache: [Option<(i32, i32)>; 300] = [None; 300];
    //     let result = amicable_pair(220, &mut cache);

    //     assert_eq!(result, Some((220, 284)));
    //     assert_eq!(cache[220], Some((220, 284)));
    //     assert_eq!(cache[284], Some((220, 284)));
    // }

    #[test]
    fn solve() {
        sum_amicable_numbers();

        let n = 5 % 1;
        println!("{n}");
    }
}
