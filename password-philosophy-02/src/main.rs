use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut count = 0;

    for line in contents.lines() {
        let modified: String = line.chars().map(|c| match c {
            '-' => ' ',
            ':' => ' ',
            _ => c
        }).collect();

        let params: Vec<&str> = modified.split_whitespace().collect();

        if params.len() == 4 {
            let min_count: usize = params[0].parse().unwrap();
            let max_count: usize = params[1].parse().unwrap();
            let check_char = params[2].chars().next().unwrap();
            let password = params[3];

            if validate_password(min_count, max_count, check_char, password) {
                count += 1;
                println!("{} {} {} {}", min_count, max_count, check_char, password);
            }
        }
    }
    println!("count is {}", count);
}

fn validate_password(min_count: usize, max_count: usize, letter: char, password: &str) -> bool {
    contains_letter_on_position(password, letter, min_count - 1) ^ contains_letter_on_position(password, letter, max_count - 1)
}

fn contains_letter_on_position(string: &str, letter: char, position: usize) -> bool {
    let mut contains = false;
    if let Some(lettr) = string.chars().nth(position) {
        if lettr == letter {
            contains = true;
        }
    }

    contains
}

