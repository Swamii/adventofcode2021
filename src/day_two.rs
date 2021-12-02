fn first_pass(instructions: &Vec<Instruction>) -> (usize, usize) {
    let mut horizontal = 0;
    let mut vertical = 0;

    for step in instructions {
        match step {
            Instruction::Down(value) => vertical += value,
            Instruction::Up(value) => vertical -= value,
            Instruction::Forward(value) => horizontal += value,
        }
    }
    return (horizontal, vertical);
}

fn second_pass(instructions: &Vec<Instruction>) -> (usize, usize) {
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;

    for step in instructions {
        match step {
            Instruction::Down(value) => aim += value,
            Instruction::Up(value) => aim -= value,
            Instruction::Forward(value) => {
                horizontal += value;
                vertical += value * aim;
            }
        }
    }
    return (horizontal, vertical);
}

#[derive(Debug)]
enum Instruction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl Instruction {
    fn from_str(input: &str, value: usize) -> Option<Instruction> {
        match input {
            "Forward" => Some(Instruction::Forward(value)),
            "Down" => Some(Instruction::Down(value)),
            "Up" => Some(Instruction::Up(value)),
            _ => None,
        }
    }
}

pub fn run() {
    let contents = crate::utils::read_input("day2.txt");
    let instructions: &Vec<Instruction> = &contents
        .split("\n")
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let instruction_name = crate::utils::capitalize_string(parts[0]);
            let steps = parts[1].parse::<usize>().unwrap();
            return Instruction::from_str(&instruction_name, steps).unwrap();
        })
        .collect();

    let (horizontal, vertical) = first_pass(instructions);
    println!(
        "First pass: {} * {} = {}",
        horizontal,
        vertical,
        horizontal * vertical
    );
    let (horizontal, vertical) = second_pass(instructions);
    println!(
        "Second pass: {} * {} = {}",
        horizontal,
        vertical,
        horizontal * vertical
    );
}
