use std::fs;

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("input.txt")?;

    let mut seats: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        seats.push(line.chars().collect());
    }

    part_1(&seats);
    Ok(())
}

fn part_1(seats: &Vec<Vec<char>>) {
    let mut cloned = seats.clone();
    cloned[0][0] = 'C';
    println!("{} a {}", cloned[0][0], seats[0][0]);
}
