use std::fs;


fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("input.txt")?;
    let mut highest_id = 0;

    let mut ids = Vec::new();

    for line in contents.lines() {
        let mut row_lower = 0;
        let mut row_higher = 127;
        let mut column_lower = 0;
        let mut column_higher = 7;

        for direction in line.chars() {
            let row_range = row_higher - row_lower + 1;
            let column_range = column_higher - column_lower + 1;
            match direction {
                'F' => row_higher -= row_range / 2,
                'B' => row_lower += row_range / 2,
                'R' => column_lower += column_range / 2,
                'L' => column_higher -= column_range / 2,
                _ => ()
            }
        }

        let id = row_lower * 8 + column_lower;
        ids.push(id);

        if highest_id < id {
            highest_id = id;
        }
    }

    println!("Highest id is: {}", highest_id);
    ids.sort();
    process_ids(&ids);

    Ok(())
}

fn process_ids(ids: &Vec<i32>) {
    let mut value = *ids.first().unwrap();

    for i in 0..ids.len() - 1 {
        if value != ids[i] {
            println!("Found gap {}", value);
            break;
        }

        value += 1;
    }
}
