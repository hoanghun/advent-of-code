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
    part_2(&adapters);

    Ok(())
}

fn part_1(adapters: &Vec<usize>) {
    let mut counts = vec![0, 1, 0, 1];


    for i in 0..adapters.len() - 1 {
        let diff = adapters[i + 1] - adapters[i];
        counts[diff] += 1;
    }

    println!("What is the number of 1-jolt differences multiplied by the number of 3-jolt differences? Answer: {}.", counts[1] * counts[3]);
}

fn part_2(adapters: &Vec<usize>) -> u128 {
    let max = adapters.iter().max().unwrap().to_owned();
    let mut costs: Vec<u128> = vec![0; max + 1];

    let possible_jolts: [usize; 3] = [1, 2, 3];

    costs[0] = 1;

    for adapter in adapters {
        costs[*adapter] = possible_jolts.iter()
            .filter(|jolt| adapter.ge(jolt))
            .map(|jolt| costs[adapter - jolt])
            .sum();
    }


    let distinct_arrangements = costs.iter().max().unwrap();
    println!("What is the total number of distinct ways you can arrange the adapters to connect the charging outlet to your device? Answer: {}.", distinct_arrangements);

    *distinct_arrangements
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        adapters.sort();

        let distinct_arrangements = part_2(&adapters);
        assert_eq!(distinct_arrangements, 8);
    }
}
