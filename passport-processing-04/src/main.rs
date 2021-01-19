use std::collections::HashMap;

const BIRTH_YEAR: &str = "byr";
const ISSUE_YEAR: &str = "iyr";
const EXPIRATION_YEAR: &str = "eyr";
const HEIGHT: &str = "hgt";
const HAIR_COLOR: &str = "hcl";
const EYE_COLOR: &str = "ecl";
const PASSPORT_ID: &str = "pid";

fn main() -> Result<(), std::io::Error> {
    let contents = std::fs::read_to_string("input.txt")?;

    let mut passport: HashMap<&str, &str> = HashMap::new();
    let mut valid_count = 0;

    for line in contents.lines() {
        if line.is_empty() {
            if validate_password(&passport) {
                valid_count += 1;
                println!("{:?}", passport);
            }
            passport.clear();
        } else {
            parse_line(line, &mut passport);
        }
    }

    println!("valid count {}", valid_count);

    Ok(())
}

fn parse_line<'a>(line: &'a str, passport: &mut HashMap<&'a str, &'a str>) {
    let pairs: Vec<&str> = line.split_whitespace().collect();

    for pair in pairs {
        let pair: Vec<&str> = pair.split(':').collect();

        passport.insert(pair[0], pair[1]);
    }
}

fn validate_password(passport: &HashMap<&str, &str>) -> bool {
    let mut is_valid = true;
    is_valid &= validate_number(passport, BIRTH_YEAR, 1920, 2002);
    is_valid &= validate_number(passport, ISSUE_YEAR, 2010, 2020);
    is_valid &= validate_number(passport, EXPIRATION_YEAR, 2020, 2030);
    is_valid &= validate_height(passport.get(HEIGHT));
    is_valid &= validate_hair_color(passport.get(HAIR_COLOR));
    is_valid &= validate_eye_color(passport.get(EYE_COLOR));
    is_valid &= validate_passport_id(passport.get(PASSPORT_ID));

    is_valid
}

fn validate_passport_id(passport_id: Option<&&str>) -> bool {
    if let Some(passport_id) = passport_id {
        if passport_id.len() == 9 {
            for character in passport_id.chars() {
                if !character.is_numeric() {
                    return false;
                }

                return true;
            }
        }
    }

    false
}

fn validate_eye_color(eye_color: Option<&&str>) -> bool {
    let mut is_valid = false;

    if let Some(eye_color) = eye_color {
        match *eye_color {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => is_valid = true,
            _ => is_valid = false,
        }
    }

    is_valid
}

fn validate_hair_color(hair_color: Option<&&str>) -> bool {
    if let Some(hair_color) = hair_color {
        if hair_color.len() == 7 {
            let chars: Vec<char> = hair_color.chars().collect();
            if *chars.first().unwrap() != '#' {
                return false;
            }
            for i in 1..chars.len() {
                match chars[i] {
                    '0'..='9' | 'a'..='f' => (),
                    _ => return false,
                }

            }
            return true;
        }
    }

    false
}

fn validate_height(height: Option<&&str>) -> bool {
    let mut is_valid = false;

    if let Some(height) = height {
        let (height, metric) = height.split_at(height.len() - 2);

        let height: usize = height.parse().unwrap_or(0);
        if metric == "in" && height >= 59 && height <= 75 {
            is_valid = true;
        }

        if metric == "cm" && height >= 150 && height <= 193 {
            is_valid = true;
        }
    }

    is_valid
}

/// Validating BYR
fn validate_number(
    passport: &HashMap<&str, &str>,
    code: &str,
    min_value: usize,
    max_value: usize,
) -> bool {
    let mut is_valid = false;
    if let Some(value) = passport.get(code) {
        let value: usize = value.parse().unwrap_or(0);
        if value >= min_value && value <= max_value {
            is_valid = true;
        }
    }

    is_valid
}
