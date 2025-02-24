const DIGITS: [u16; 9] = [
    //"one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    3, 3, 5, 4, 4, 3, 5, 5, 4,
];
const TEENS: [u16; 9] = [
    //"eleven",
    6, //"twelve",
    6, //"thirteen",
    8, //"fourteen",
    8, //"fifteen",
    7, //"sixteen",
    7, //"seventeen",
    9, //"eighteen",
    8, //"nineteen",
    8,
];
const TENS: [u16; 8] = [
    //"twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    6, 6, 5, 5, 5, 7, 6, 6,
];

fn number_letter_counts() -> u16 {
    let digits: u16 = DIGITS.iter().sum();
    let teens: u16 = TEENS.iter().sum();
    let tens: u16 = TENS.map(|d| d * 10 + digits).iter().sum();
    let hundreds: u16 = DIGITS.map(|d| d + 7).iter().sum();
    let hundreds_and: u16 = DIGITS.map(|d| (d + 7 + 3) * 99).iter().sum();
    let thousand: u16 = "onethousand".len() as u16;

    // the three is for "ten"
    (digits + tens + teens + 3) * 10 + hundreds_and + thousand + hundreds
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve() {
        println!("{}", number_letter_counts())
    }
}
