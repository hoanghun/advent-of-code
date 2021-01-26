use std::fs;

fn main() -> Result<(), std::io::Error> {

    let contents = fs::read_to_string("input.txt")?;
    let numbers: Vec<usize> = contents.lines()
        .map(|line| line.parse().unwrap())
        .collect();

    println!("{:?}", numbers);
    Ok(())
}

fn foo() {

}
