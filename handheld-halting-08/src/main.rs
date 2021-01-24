use std::fs;

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("input.txt")?;

    let instructions: Vec<&str> = contents.lines().collect();

    for j in 0..instructions.len() - 1 {
        let mut instructions_modified: Vec<&str> = instructions.clone();

        let old_instruction = instructions_modified[j];
        let modified_instruction = modify_instruction(instructions_modified[j]);
        instructions_modified[j] = &modified_instruction;
        println!("From {} to {}", old_instruction, modified_instruction);
        if old_instruction == modified_instruction {
            println!("Didn't modify, skiping.");
            continue;
        }

        let (success, accumulator) = evaluate_program(instructions_modified);
        if success {
            println!("Accumulator is {}", accumulator);
            break;
        }
    }

    Ok(())
}

fn modify_instruction(instruction: &str) -> String {
    let (instruction, value) = instruction.split_at(instruction.find(' ').unwrap());
    match instruction {
        "acc" => format!("{} {}", instruction, value.trim()),
        "jmp" => format!("nop {}", value.trim()),
        "nop" => format!("jmp {}", value.trim()),
        _ => format!("nop 1")
    }
}


fn evaluate_program(instructions: Vec<&str>) -> (bool, isize) {
    let mut instruction_occurence = vec![0; instructions.len()];
    let mut i = 0;
    let mut accumulator = 0;

    while i < instructions.len() {
        let (relative_jump, value) = evaluate_instruction(instructions[i]);
        instruction_occurence[i] += 1;
        if instruction_occurence[i] > 1 {
            return (false, accumulator);
        }

        if relative_jump < 0 {
            i -= relative_jump.abs() as usize;
        } else {
            i += relative_jump as usize;
        }

        accumulator += value;
    }

    (true, accumulator)
}

fn evaluate_instruction(instruction: &str) -> (isize, isize) {
    let (instruction, value) = instruction.split_at(instruction.find(' ').unwrap());
    let value = value.trim().parse::<isize>().unwrap();
    match instruction {
        "acc" => (1, value),
        "jmp" => (value, 0),
        _ => (1, 0)
    }
}
