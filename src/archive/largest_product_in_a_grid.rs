pub fn solve_for_official_input() {
    let official_input: Vec<Vec<i64>> = vec![
        parse_row("08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08"),
        parse_row("49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00"),
        parse_row("81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65"),
        parse_row("52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91"),
        parse_row("22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80"),
        parse_row("24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50"),
        parse_row("32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70"),
        parse_row("67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21"),
        parse_row("24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72"),
        parse_row("21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95"),
        parse_row("78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92"),
        parse_row("16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57"),
        parse_row("86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58"),
        parse_row("19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40"),
        parse_row("04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66"),
        parse_row("88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69"),
        parse_row("04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36"),
        parse_row("20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16"),
        parse_row("20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54"),
        parse_row("01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48"),
    ];
    println!(
        "Solution 11: {:#?}",
        largest_product_in_a_grid(&official_input)
    );
}

fn parse_row(row: &str) -> Vec<i64> {
    row.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

// Assumption: length and width are the same
fn largest_product_in_a_grid(input: &[Vec<i64>]) -> i64 {
    let mut largest = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if can_across(j.try_into().unwrap()) {
                let product = product_across(i, j, input);
                if product > largest {
                    largest = product;
                }
            }
            if can_down(i.try_into().unwrap()) {
                let product = product_down(i, j, input);
                if product > largest {
                    largest = product;
                }
            }
            if can_down_right(i.try_into().unwrap(), j.try_into().unwrap()) {
                let product = product_down_right(i, j, input);
                if product > largest {
                    largest = product;
                }
            }
            if can_up_right(i.try_into().unwrap(), j.try_into().unwrap()) {
                let product = product_up_right(i, j, input);
                if product > largest {
                    largest = product;
                }
            }
        }
    }
    largest
}

fn can_across(j: i64) -> bool {
    j + 3 < 20
}

fn product_across(i: usize, j: usize, input: &[Vec<i64>]) -> i64 {
    let mut product = 1;

    for k in 0..4 {
        product *= input[i][j + k];
    }
    product
}

fn can_down(i: i64) -> bool {
    i + 3 < 20
}

fn product_down(i: usize, j: usize, input: &[Vec<i64>]) -> i64 {
    let mut product = 1;

    for l in 0..4 {
        product *= input[i + l][j];
    }
    product
}

fn can_down_right(i: i64, j: i64) -> bool {
    can_across(j) && can_down(i)
}

fn product_down_right(i: usize, j: usize, input: &[Vec<i64>]) -> i64 {
    let mut product = 1;
    for k in 0..4 {
        product *= input[i + k][j + k];
    }
    product
}

fn can_up(i: i64) -> bool {
    i >= 3
}

fn can_up_right(i: i64, j: i64) -> bool {
    can_up(i) && can_down(j)
}

fn product_up_right(i: usize, j: usize, input: &[Vec<i64>]) -> i64 {
    let mut product = 1;
    for k in 0..4 {
        product *= input[i - k][j + k];
    }
    product
}
