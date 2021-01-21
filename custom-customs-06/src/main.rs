use std::fs;

const ALPHABET_SIZE: usize = 'z' as usize - 'a' as usize + 1;
const START_VALUE: usize = 'a' as usize;

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("input.txt")?;
    println!("size: {}", ALPHABET_SIZE);

    let mut sum = 0;
    let mut answers: [usize; ALPHABET_SIZE] = [0; ALPHABET_SIZE];
    let mut group_size = 0;

    for line in contents.lines() {
        if line.is_empty() {
            for num in answers.iter() {
                if *num == group_size {
                    sum += 1;
                }
            }
            group_size = 0;
            answers = [0; ALPHABET_SIZE];
        } else {
            group_size += 1;
        }
        for c in line.chars() {
            answers[c as usize - START_VALUE] += 1;
        }
    }


    println!("Total sum is: {}", sum);

    Ok(())
}
