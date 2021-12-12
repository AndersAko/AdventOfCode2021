use std::fs;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use itertools::Itertools;

#[derive(Debug)]
struct Display {
    combinations : Vec<String>,
    output : Vec<String>
}

fn main() {
    let filecontents = fs::read_to_string("input_test.txt").expect("Something went wrong?");

    let lines = filecontents.split_terminator("\n");
    // .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut displays = Vec::<Display>::new();

    for line in lines {
        let parts = line.split("|").collect::<Vec<&str>>();

        let display = Display {
            combinations : parts[0].split_whitespace().map(|x| String::from(x)).collect(),
            output :  parts[1].split_whitespace().map(|x| String::from(x)).collect()
        };
        println!("{:?}", &display);
        displays.push(display);
    }

    let mut count = 0;
    for disp in &displays {
        for dig in &disp.output {
            if dig.len() == 2 || dig.len() == 3|| dig.len() == 4 || dig.len() == 7 {
                count += 1;
                println!("{}", dig);
            }
        }
    }
    println!("Part 1: A total of {} digits are 1, 7, 4 or 8", &count);

    solve(&"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab".split_whitespace()
        .map(|x| String::from(x)).collect::<Vec<String>>());
    // for disp in displays {
    //     let foo = solve(&disp.combinations);
    //     println!("Solution to {:?} is {:?}", &disp, foo);
    //     println!();
    // }

}

    // in 3 , but not 2 => seg1
    // count6 => seg2
    // count8 => seg3
    // count7 and in len4 =>  seg4
    // count4 => seg5
    // count 9 => seg6
    // count7 and not in len4 => seg7

/*  Segments:
     000 
    1   2
    1   2
     333
    4   5
    4   5
     666
*/

#[derive(Copy,Clone,Eq,PartialEq, Debug)]
struct State {
    index: usize,
    segments: [Option<char>; 7],
    used_digits: [bool; 10]
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.index.cmp(&self.index)
        .then(self.segments.iter().filter(|c| c.is_some()).count()
             .cmp(&other.segments.iter().filter(|c| c.is_some()).count()))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

}

#[derive(Clone, Debug)]
struct NumberDisplay {
    number: usize,
    segments : Vec<usize>
}

// Gives sorted vec, where [0] is combination for "0", etc
fn solve(combinations: &Vec<String>) -> Vec<char> {
    let digits:Vec<NumberDisplay> = vec!{ 
        NumberDisplay { number : 0, segments: vec!(0,1,2,4,5,6)},
        NumberDisplay { number : 1, segments: vec!(2,5) },
        NumberDisplay { number : 2, segments: vec!(0,2,3,4,6) },
        NumberDisplay { number : 3, segments: vec!(0,2,3,5,6) },
        NumberDisplay { number : 4, segments: vec!(1,2,3,5) },
        NumberDisplay { number : 5, segments: vec!(0,1,3,5,6) },
        NumberDisplay { number : 6, segments: vec!(0,1,3,4,5,6) },
        NumberDisplay { number : 7, segments: vec!(0,2,5) },
        NumberDisplay { number : 8, segments: vec!(0,1,2,3,4,5,6) },
        NumberDisplay { number : 9, segments: vec!(0,1,2,3,5,6) }
    };
    
    let mut sorted_by_len = combinations.clone();
    sorted_by_len.sort_by(|x, y| x.len().cmp(&y.len()) );
    
    let mut heap = BinaryHeap::new();

    heap.push( State {index: 0, segments: [None;7], used_digits: [false; 10] } );
    while let Some( State{index, segments, used_digits} ) = heap.pop() {
        if index == 10 {
            assert!(segments.iter().all(|s| s.is_some())); 
            println!("==> Found one solution: {:?} {:?}", segments, used_digits);
            // return Vec::from(segments.map(|c| c.unwrap() ));
            continue;
        }

        let comb:Vec<char> = sorted_by_len[index].chars().collect();
        let mut possible_digits:Vec<NumberDisplay> = digits.clone();
        possible_digits.retain(|d| d.segments.len() == comb.len() && !used_digits[d.number] );

        let remaining_letters:Vec<char> = comb.iter()
            .filter(|&&c| !segments.iter().any(|&s| s == Some(c)))
            .map(|c| *c )
            .collect();

        println!("-- For the combination {:?}, the possible digits are: {:?} and remaining letters after filtering out {:?} is {:?}",
            &comb, &possible_digits, segments, remaining_letters);

        for mut digit in possible_digits {
            print!("Digit {:?}", digit);
            digit.segments.retain(|&s| segments[s as usize].is_none());
            println!(" filtered to {:?}", digit);

            match remaining_letters.len() {
                0 if digit.segments.len() == 0 => {
                    let mut next_state = State { index: index+1, segments: segments, used_digits};
                    next_state.used_digits[digit.number] = true;
                    println!("Possible next state {:?}", next_state);
                    heap.push(next_state); 
                    },
                letters if letters == digit.segments.len() => {
                        for permutation in remaining_letters.iter().permutations(remaining_letters.len()) {
                            let mut next_state = State { index: index+1, segments: segments, used_digits};
                            next_state.used_digits[digit.number] = true;
                            for i in 0..permutation.len() {
                                next_state.segments[digit.segments[i] as usize] = Some(*permutation[i]);
                            }
                            println!("Possible next state {:?}", next_state);
                            heap.push(next_state);
                        }
                    }
                _ => {
                    println!("Trying to match digit {:?} with {:?} combination failed since {:?}",
                    digit, remaining_letters, segments);                    
                }
            }
        }
    }
    
    vec!('0')
}

// combinations: bdcfga fag dgafc fg dacbf bdafec cfbg fabdgec eabdfg eagcd 