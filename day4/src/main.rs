use std::fs;
use std::collections::HashMap;

fn main() {
    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");
    let mut lines = (&filecontents).split_terminator('\n');

    let numbers = lines.next().expect("No first line!!").split_terminator(",").map(|n| n.parse::<i32>().unwrap());

    // test();
    let mut bingo_boards = setup(lines);
 
    'outer: for i in numbers {
        println!("Calling number {}", i);
        for board_no in 0..bingo_boards.len() {
            if let Some(matching_ix) = bingo_boards[board_no].iter().position(|&x| x == i)
            {
                bingo_boards[board_no][matching_ix] = -1;
            }
            if winner(&bingo_boards[board_no]) {
                let sum_unmarked :i32 = bingo_boards[board_no].iter().filter(|&&x| x>=0 ).sum();
                println!("We have a winner, with score {} on number {} => {}", sum_unmarked, i, i*sum_unmarked);
                break 'outer;
            }
        }
    }

    println!("Part 2");
    let mut lines = filecontents.split_terminator('\n');

    let numbers = lines.next().expect("No first line!!").split_terminator(",").map(|n| n.parse::<i32>().unwrap());

    let mut bingo_boards = setup(lines);

    let mut winners  = HashMap::<usize, i32>::new();

    'outer2: for i in numbers {
        println!("Calling number {}", i);
        for board_no in 0..bingo_boards.len() {
            if winners.contains_key(&board_no) {
                continue;
            }
            if let Some(matching_ix) = bingo_boards[board_no].iter().position(|&x| x == i)
            {
                bingo_boards[board_no][matching_ix] = -1;
            }
            if winner(&bingo_boards[board_no]) {
                let sum_unmarked :i32 = bingo_boards[board_no].iter().filter(|&&x| x>=0 ).sum();
                println!("We have a winner, with score {} on number {} => {}", sum_unmarked, i, i*sum_unmarked);
                winners.insert(board_no, i*sum_unmarked);
            }
        }
    }

}

fn setup(mut lines: std::str::SplitTerminator<char>) -> Vec<Vec<i32>> {
    let mut bingo_boards:Vec<Vec<i32>> = Vec::new();

    while let Some(_blank) = lines.next() {
        let mut board: Vec<i32> = vec!();

        for i in 0..5 {
            let line = lines.next().expect("Expected to find a bingoboard line here!");
            // println!("Line: {}: {:?}", i, line);

            board.extend(line.split_whitespace()
                .map(|l| l.parse::<i32>().unwrap())
                );  
        }
        // println! ("{:?}", board);
        bingo_boards.push(board);
    }
    bingo_boards
}

fn winner(board : &Vec<i32>) -> bool {
    for i in 0..5 {
        if board[i*5..i*5+5].iter().all(|&c| c == -1) {
            println!("Board {:?} is a winner, row {}", board, i);
            return true;
        }
        if board.iter().skip(i).step_by(5).all(|&c| c == -1) {
            println!("Board {:?} is a winner, col {}", board, i);
            return true;
        }
    }
    false
} 

fn test() -> () {
    let board = vec![1,2,3,4,5,-1,-1,-1,-1,-1,1,2,3,4,5,1,2,3,4,5,1,2,3,4,5];
    assert_eq!(winner(&board), true) ;

    assert_eq!(winner(&vec![1,2,3,4,5,-1,-1,0,-1,-1,-1,-1,-1,4,5,1,2,3,4,5,1,2,3,4,5]), false) ;
    assert_eq!(winner(&vec![1,2,-1,4,5,1,2,-1,4,5,1,2,-1,4,5,1,2,-1,4,5,1,2,-1,4,5]), true) ;
    assert_eq!(winner(&vec![1,2,-1,4,5,1,2,-1,4,5,-1,-1,0,-1,-1,1,2,-1,4,5,1,2,-1,4,5]), false) ;

}