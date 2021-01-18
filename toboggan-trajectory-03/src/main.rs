use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let contents = fs::read_to_string("input.txt")?;
    let lines: Vec<&str> = contents.lines().collect();

    let tree_count_1_1 = count_collisions(1, 1, &lines);
    let tree_count_3_1 = count_collisions(3, 1, &lines);
    let tree_count_5_1 = count_collisions(5, 1, &lines);
    let tree_count_7_1 = count_collisions(7, 1, &lines);
    let tree_count_1_2 = count_collisions(1, 2, &lines);



    println!("Right: 1, down: 1 - tree count = {}", tree_count_1_1);
    println!("Right: 3, down: 1 - tree count = {}", tree_count_3_1);
    println!("Right: 5, down: 1 - tree count = {}", tree_count_5_1);
    println!("Right: 7, down: 1 - tree count = {}", tree_count_7_1);
    println!("Right: 1, down: 2 - tree count = {}", tree_count_1_2);

    println!("Multiplied is = {}", tree_count_1_1 * tree_count_3_1 * tree_count_5_1 * tree_count_7_1 * tree_count_1_2);

    Ok(())
}

fn count_collisions(right: usize, down: usize, map: &Vec<&str>) -> isize {
    let line_length = match map.first() {
        None => return 0,
        Some(length) => length.len()
    };

    let mut x = 0;
    let mut tree_count = 0;


    for i in (0..map.len() - 1).step_by(down) {
        let line = map[i];
        if line.len() < line_length {
            break;
        }

        let space = line.chars().nth(x % line_length).unwrap();

        if space == '#' {
            tree_count += 1;
        }

        x += right;
    }

    tree_count
}
