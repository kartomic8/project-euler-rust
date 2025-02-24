use std::fs;

fn solve() -> i32 {
    let sorted_names = read_file();
    let mut score = 0;
    sorted_names.iter().enumerate().for_each(|(i, name)| {
        score += ((i + 1) as i32) * alphabetical_value(name);
    });
    score
}

fn read_file() -> Vec<String> {
    let file_path = "/Users/kartik/code/learn/rust/euler/project_euler/content/0022_names.txt";
    let contents = fs::read_to_string(file_path).expect("There was an error parsing the file");
    parse_contents(contents)
}

fn parse_contents(contents: String) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let mut current_name = String::from("");
    let mut parsing = false;
    for c in contents.chars() {
        if c == '"' {
            parsing = !parsing;
        } else if c == ',' {
            insert_in_order(current_name, &mut names);
            current_name = String::from("");
        } else if parsing {
            current_name.push(c);
        }
    }
    insert_in_order(current_name, &mut names);
    names
}

fn insert_in_order(s: String, v: &mut Vec<String>) {
    match v.binary_search(&s) {
        Ok(_) => {}
        Err(pos) => v.insert(pos, s),
    }
}

fn alphabetical_value(s: &String) -> i32 {
    let mut value = 0;
    for c in s.chars() {
        value += c.to_ascii_uppercase() as i32 - 64;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let contents = read_file();
        assert!(contents.len() > 5000);
    }

    #[test]
    fn test_alphabetical_value() {
        let test_cases = vec![("ABBEY", 35), ("COLIN", 53), ("ED", 9), ("GABE", 15)];

        for (name, result) in test_cases {
            assert_eq!(result, alphabetical_value(&name.to_string()))
        }
    }

    #[test]
    fn solve() {
        let answer = super::solve();
        println!("Solution to Problem 22: {answer}");
    }
}
