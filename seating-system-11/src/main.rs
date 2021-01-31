use std::fs;

type Seats = Vec<Vec<char>>;

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("input.txt")?;

    let seats: Seats = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    part_1(&seats);
    Ok(())
}

fn part_1(seats: &Seats) {
    let mut seats_read = seats.clone();
    let mut seats_write = seats_read.clone();

    loop {
        let mut changed_seats = false;
        for (i, row) in seats_read.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                let old_seat_symbol = seats_read[i][j];
                let new_seat_symbol = change_seat(i, j, old_seat_symbol, &seats_read);
                seats_write[i][j] = new_seat_symbol;
                if old_seat_symbol != new_seat_symbol {
                    changed_seats = true;
                }
            }
        }

        std::mem::swap(&mut seats_read, &mut seats_write);

        if !changed_seats {
            break;
        }
    }

    println!(
        "How many seats end up occupied? Answer: {}.",
        count_occupied_seats(&seats_read)
    );
}

fn count_occupied_seats(seats: &Seats) -> usize {
    return seats
        .iter()
        .flat_map(|row| row.iter().filter(|seat| **seat == '#'))
        .count();
}

fn change_seat(row_index: usize, column_index: usize, symbol: char, seats: &Seats) -> char {
    if symbol == '.' {
        return symbol;
    }

    let start = if column_index > 0 {
        column_index - 1
    } else {
        0
    };

    let end = if column_index < seats.iter().nth(0).unwrap().len() - 1 {
        column_index + 1
    } else {
        column_index
    };

    let (return_symbol, search_symbol, limit) = if symbol == '#' {
        ('L', '#', 5)
    } else {
        ('L', '#', 1)
    };

    let row_index_start = if row_index > 0 {
        row_index - 1
    } else {
        row_index
    };

    let mut sum = 0;
    for i in row_index_start..=row_index + 1 {
        if let Some(row) = seats.get(i) {
            for j in start..=end {
                if row[j] == search_symbol {
                    sum += 1;
                }
            }
        }
    }

    if sum >= limit {
        return_symbol
    } else {
        search_symbol
    }
}
