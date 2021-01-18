use std::collections::HashMap;


fn main() -> Result<(), std::io::Error> {
    let contents = std::fs::read_to_string("input.txt")?;

    let mut passport: HashMap<&str, &str> = HashMap::new();
    let mut valid_count = 0;

    for line in contents.lines() {
        if line.is_empty() {
            println!("{:?}", passport);
            if validate_password(&passport) {
                valid_count += 1;
            }
            passport.clear();
        }
        else {
            parse_line(line, &mut passport);
        }
    }

    println!("valid count {}", valid_count);

    Ok(())
}

fn parse_line<'a>(line: &'a str, passport: &mut HashMap<&'a str, &'a str>) {
    let pairs: Vec<&str> = line.split_whitespace().collect();

    for pair in pairs {
        println!("pair: {}", pair);
        let pair: Vec<&str> = pair.split(':').collect();

        passport.insert(pair[0], pair[1]);
    }
}


fn validate_password(passport: &HashMap<&str, &str>) -> bool {
    if passport.len() == 8 {
        println!("8");
        return true;
    }

    if passport.len() == 7 && passport.get("cid").is_none() {
        println!("7");
        return true;
    }

    false
}
