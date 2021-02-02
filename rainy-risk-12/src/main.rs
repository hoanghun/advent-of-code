use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    North = 0,
    West = 90,
    South = 180,
    East = 270,
}

enum Instruction {
    North(i64),
    West(i64),
    South(i64),
    East(i64),
    RotateRight(i64),
    RotateLeft(i64),
    Forward(i64),
}



impl From<i64> for Direction {
    fn from(degree: i64) -> Self {
        match degree {
            0 => Direction::North,
            90 => Direction::West,
            180 => Direction::South,
            270 => Direction::East,
            _ => panic!("Invalid degree"),
        }
    }
}

impl Add<i64> for Direction {
    type Output = Direction;

    fn add(self, rhs: i64) -> Self::Output {
        Direction::from((self as i64 + rhs) % 360)
    }
}

impl Sub<i64> for Direction {
    type Output = Direction;

    fn sub(self, rhs: i64) -> Self::Output {
        Direction::from((self as i64 + 360 - (rhs % 360)) % 360)
    }
}

impl Direction {
    fn to_instruction(&self, value: i64) -> Instruction {
        match self {
            Direction::North => Instruction::North(value),
            Direction::West => Instruction::West(value),
            Direction::South => Instruction::South(value),
            Direction::East => Instruction::East(value),
        }
    }
}

struct Coordinates {
    x: i64,
    y: i64,
}

trait Follow {
    fn follow(&mut self, instruction: &Instruction);
}

struct Ship {
    coord: Coordinates,
    direction: Direction,
}

impl Ship {
    fn new() -> Self {
        Ship {
            coord: Coordinates { x: 0, y: 0 },
            direction: Direction::East,
        }
    }


    fn get_manhattan_distance_from_start(&self) -> i64 {
        self.coord.x.abs() + self.coord.y.abs()
    }
}

impl Follow for Ship {
    fn follow(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::North(value) => self.coord.y += value,
            Instruction::West(value) => self.coord.x += value,
            Instruction::South(value) => self.coord.y -= value,
            Instruction::East(value) => self.coord.x -= value,
            Instruction::RotateRight(value) => self.direction = self.direction - *value,
            Instruction::RotateLeft(value) => self.direction = self.direction + *value,
            Instruction::Forward(value) => self.follow(&self.direction.to_instruction(*value)),
        }
    }
}

fn part_1(instructions: &Vec<Instruction>) {
    let mut ship = Ship::new();

    instructions.iter().for_each(|instr| ship.follow(instr));

    println!("What is the Manhattan distance between that location and the ship's starting position? Answer: {}.", ship.get_manhattan_distance_from_start());
}

fn parse_instruction(line: &str) -> Instruction {
    let (instruction, value) = line.split_at(1);
    let value: i64 = value.parse().unwrap();
    match instruction {
        "N" => Instruction::North(value),
        "S" => Instruction::South(value),
        "E" => Instruction::East(value),
        "W" => Instruction::West(value),
        "L" => Instruction::RotateLeft(value),
        "R" => Instruction::RotateRight(value),
        "F" => Instruction::Forward(value),
        _ => panic!("Invalid parse"),
    }
}

fn main() -> Result<(), std::io::Error> {
    let contents = std::fs::read_to_string("input.txt")?;
    let instructions: Vec<Instruction> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_instruction(line))
        .collect();

    part_1(&instructions);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_left() {
        let east: Direction = Direction::East;
        assert_eq!(Direction::North, east + 90);
    }

    #[test]
    fn rotate_right() {
        let east: Direction = Direction::East;
        assert_eq!(Direction::South, east - 90);
    }

    #[test]
    fn test_part_1() {
        let mut ship = Ship::new();
        ship.follow(&Instruction::Forward(10));
        ship.follow(&Instruction::North(3));
        ship.follow(&Instruction::Forward(7));
        ship.follow(&Instruction::RotateRight(90));
        ship.follow(&Instruction::Forward(11));

        assert_eq!(Direction::South, ship.direction);
        assert_eq!(-17, ship.coord.x);
        assert_eq!(-8, ship.coord.y);
        assert_eq!(25, ship.get_manhattan_distance_from_start());
    }
}
