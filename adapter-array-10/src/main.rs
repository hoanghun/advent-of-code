use std::fs;

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("input.txt")?;

    let mut adapters: Vec<usize> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    adapters.sort();

    part_1(&adapters);

    Ok(())
}

fn part_1(input: &Vec<usize>) {
    let mut counts = vec![0, 1, 0, 1];


    for i in 0..input.len() - 1 {
        let diff = input[i + 1] - input[i];
        counts[diff] += 1;
    }

    println!("{:?}", counts);
    println!("Answer: {}", counts[1] * counts[3]);
}
