use std::fs;

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("input.txt")?;
    let numbers: Vec<usize> = contents.lines().map(|line| line.parse().unwrap()).collect();

    for i in 0..numbers.len() - 25 {
        let target = numbers[i + 25];
        if !validate(&numbers[i..i + 25], target) {
            println!("Invalid number: {}", target);
            find_sequence(&numbers, target);
            break;
        }
    }

    Ok(())
}

fn find_sequence(numbers: &Vec<usize>, target: usize) {
    let mut start = 0;
    let mut end = 1;
    let mut sum = numbers[start] + numbers[end];
    while end < numbers.len() {
        if sum == target {
            let sum = add_two_extremes(&numbers[start..=end]);
            println!("Sum is of lowest and highest value is {}", sum);
            return;
        }
        if sum < target {
            end += 1;
            sum += numbers[end];
        } else {
            sum -= numbers[start];
            start += 1;
        }
    }

    println!("Not found");
}

fn add_two_extremes(numbers: &[usize]) -> usize {
    numbers
        .iter()
        .min()
        .and_then(|min| numbers.iter().max().map(|max| min + max))
        .unwrap()
}

fn validate(slice: &[usize], value: usize) -> bool {
    for i in 0..slice.len() {
        for j in i..slice.len() {
            if slice[i] + slice[j] == value {
                return true;
            }
        }
    }
    false
}
