use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Failed to read input file!");

    let (dots, instructions) = read_paper(&file_contents);
    println!("{:?} ({})", dots, dots.len());
    println!("Instructions {:?}", instructions);

    let folded = fold_paper(&dots, instructions[0]);

    println!("Part 1: {} dots visible after one fold", folded.len());

    let mut folded = dots;
    for instr in instructions { 
        folded = fold_paper(&folded, instr);
    }
    println!("{:?} ({})", folded, folded.len());
    print_paper(&folded);
}

fn read_paper(input: &str) -> (Vec<(isize, isize)>, Vec<(isize,isize)>) {
    let mut lines = input.split_terminator("\n");

    let mut dots = Vec::new();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() { break;}

        let parsed_line = line.split(",").map(|x| x.trim().parse::<isize>().unwrap()).collect::<Vec<_>>();
        dots.push((parsed_line[0], parsed_line[1]));
    }
    let mut instructions = Vec::new();
    while let Some(line) = lines.next() {
        let mut instr_split = line.split("=");
        let instr;
        if instr_split.next().unwrap().chars().last().unwrap() == 'x' {
            instr = (instr_split.next().unwrap().parse::<isize>().unwrap(), 0);
        } else {
            instr = (0, instr_split.next().unwrap().parse::<isize>().unwrap());
        }
        instructions.push(instr);
    }
    (dots, instructions)
}

fn fold_paper(dots: &Vec<(isize, isize)>, instruction: (isize,isize)) -> Vec<(isize, isize)> {
    let mut result = Vec::<(isize, isize)>::new();
    for dot in dots {
        let new = if instruction.1 == 0 && dot.0 > instruction.0 {    // fold along x
                (instruction.0 * 2 - dot.0, dot.1)
            } else if instruction.0 == 0 && dot.1 > instruction.1 {     // fold along y
                (dot.0, instruction.1 * 2 - dot.1)
            } else {
                (dot.0, dot.1)
            };
        println!("{:?} {:?} => {:?}", instruction, dot, new);
        if !result.contains(&new) { result.push(new);  }
    }
    result
}

fn print_paper(dots: &Vec<(isize, isize)>) {
    let max_x = dots.iter().map(|x| x.0).max().unwrap();
    let max_y = dots.iter().map(|x| x.1).max().unwrap();

    for row in 0..=max_y {
        for col in 0..=max_x {
            if dots.contains(&(col,row)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[test]
fn test_read_paper() {
    let (starting_dots, instructions) = read_paper(
        "6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0
        
        fold along y=7
        fold along x=5");

    assert_eq!(starting_dots, vec![(6,10),(0,14),(9,10),(0,3),(10,4),(4,11),(6,0),(6,12),(4,1),(0,13),(10,12),(3,4),
                (3,0),(8,4),(1,10),(2,14),(8,10),(9,0)]);

    assert_eq!(instructions[0], (0,7));
    assert_eq!(instructions[1], (5,0));
}

#[test]
fn test_fold() {
    let dots = vec![(6,10),(0,14),(9,10),(0,3),(10,4),(4,11),(6,0),(6,12),(4,1),(0,13),(10,12),(3,4),
    (3,0),(8,4),(1,10),(2,14),(8,10),(9,0)];
    let folded = fold_paper(&dots, (0,7));

    println!("Fold at y=7 => {:?} ({})", folded, folded.len());
    print_paper(&folded);
    assert!(folded.iter().all(|x| x.1 < 7));
    assert_eq!(folded.len(), 17);

    let folded = fold_paper(&folded, (5,0));

    println!("Fold at x=5 => {:?} ({})", folded, folded.len());
    print_paper(&folded);
    assert!(folded.iter().all(|x| x.0 < 5));

}

#[test]
fn test_print_paper() {
    let dots = vec![(4, 4), (0, 0), (1, 4), (0, 3), (0, 4), (4, 3), (4, 0), (4, 2), (4, 1), (0, 1), (0, 2), (3, 4), (3, 0), (2, 4), (2, 0), (1, 0)];
    print_paper(&dots);
}