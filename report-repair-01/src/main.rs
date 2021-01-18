use std::fs;
fn main() {
    let foo = fs::read_to_string("input.txt").unwrap();
    let mut numbers: Vec<u32> = Vec::new();

    for line in foo.lines() {
        println!("{}", line);

        if let Ok(number) = line.parse() {
            numbers.push(number);
        }
    }

    for i in 0..numbers.len() - 2 {
        for j in (i + 1)..numbers.len() - 1 {
            for k in (j + 1)..numbers.len() {
                println!("{}:{} + {}:{} + {}:{} = {}", i, numbers[i], j, numbers[j], k, numbers[k], numbers[i] + numbers[j] + numbers[k]);
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    println!("Vysledek je: {}", numbers[i] * numbers[j] * numbers[k]);
                }
            }
        }
    }
}
