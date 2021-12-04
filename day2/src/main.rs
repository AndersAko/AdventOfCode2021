use std::fs;

#[derive(Debug)]
enum Instruction {
    Forward(i32),
    Up (i32),
    Down (i32),
    None
}

#[derive(Debug)]
struct Position {horizontal: i32, depth: i32}

fn main() {
    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");
    

    let instructions: Vec<Instruction> = filecontents
        .split_terminator('\n')
        .map(|l| {
            let words : Vec<&str> = l.trim().split(" ").collect();
            let steps = words[1].parse::<i32>();
            match words[0] {
                "forward" => Instruction::Forward (steps.unwrap()),
                "up" => Instruction::Up (steps.unwrap()),
                "down" => Instruction::Down (steps.unwrap()),
                _ => Instruction::None
            }
        })
        .collect();
    
    println!("{:?}", instructions);
    part1(&instructions);

    part2(&instructions);
}

fn part1(instructions : &Vec<Instruction> ) -> () {
    let mut position = Position {horizontal: 0, depth: 0};

    for instr in instructions {
        match instr {
            Instruction::Forward(steps) =>  position.horizontal += steps,
            Instruction::Down(steps) => position.depth += steps,
            Instruction::Up(steps) => position.depth -= steps,
            _ => ()
        }        
    }
    println!("Part1: Ended up at {:?} giving the answer: {}", position, position.depth*position.horizontal);
}

fn part2(instructions : &Vec<Instruction> ) -> () {
    let mut position = Position {horizontal: 0, depth: 0};
    let mut aim = 0;

    for instr in instructions {
        match instr {
            Instruction::Forward(steps) =>  { 
                position.horizontal += steps;
                position.depth += aim * steps;
            }
            Instruction::Down(steps) => aim += steps,
            Instruction::Up(steps) => aim -= steps,
            _ => ()
        }        
    }
    println!("Part2: Ended up at {:?} giving the answer: {}", position, position.depth*position.horizontal);
}
