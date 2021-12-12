use std::fs;
use std::cmp::Ordering;
use itertools::Itertools;

#[derive(Debug)]
struct Display {
    combinations : Vec<String>,
    output : Vec<String>
}

fn main() {
    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");

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

    let mut sum = 0;
    for disp in displays {
        let solution = solve(&disp.combinations);
        println!("Solution to {:?} is {:?}", &disp, solution);
        let mut number_result = Vec::<char>::new();
        for out in disp.output {
            for digit in digits() {
                if digit.segments.iter().all(|&s| out.contains(solution.segments[s].unwrap()) ) 
                    && digit.segments.len() == out.len() {
                    number_result.push(std::char::from_digit(digit.number as u32,10).unwrap());
                    print!("{}", digit.number);
                }
            }
        }
        println!();
        let number_result = String::from_iter(number_result.iter()).parse::<u32>().unwrap();
        println!("{:?} ", number_result);
        sum += number_result;
    }
    println!("Part2: answer = {} ", sum);
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

fn digits() -> Vec<NumberDisplay> {
    return vec!{ 
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
}

fn solve_for_state (state: State, combinations:  &Vec<String>) -> Option<State> {
    println!("Evaluating state {:?}", state);

    if state.index == combinations.len() {
        assert!(state.segments.iter().all(|s| s.is_some())); 
        println!("=Found a solution: {:?}", state);
        return Some(state);
    }
    let combination = &combinations[state.index];
    let mut possible_digits:Vec<NumberDisplay> = digits().clone();
    possible_digits.retain(|d| d.segments.len() == combination.len() && !state.used_digits[d.number] );

    if possible_digits.len() == 0 { return None ; }

    let unmatched_letters:Vec<char> = combination.chars()
        .filter(|&c| !state.segments.iter().any(|&s| s == Some(c)))
        .collect();
        
    // println!("-- For the combination {:?}, the possible digits are: {:?} and remaining letters after filtering out {:?} is {:?}",
    //         &combination, &possible_digits, state.segments, unmatched_letters);

    for mut digit in possible_digits {
        if digit.segments.iter().any(|&s| {
                if let Some(letter) = state.segments[s] {
                    !combination.chars().contains(&letter)
                } else { 
                    false 
                }
             })  {
                // // println!("Mismatched segments: {:?}", digit.segments.iter().filter(|&s| {
                //     if let Some(letter) = state.segments[*s] {
                //         !combination.chars().contains(&letter)
                //     } else { 
                //         false 
                //     }
                //  }));
                continue;
            }
        
        digit.segments.retain(|&s| {
            if let Some(letter) = state.segments[s as usize] {
                !combination.chars().contains(&letter) 
            } else { true }
        });
        // println!("---- Combination {}, trying digit {} matching digit segments {:?} to {:?}",
                // combination, digit.number, digit.segments, unmatched_letters);

        match unmatched_letters.len() {
            0 if digit.segments.len() == 0 => {
                    // println!("all segments matched already");
                    let mut next_state = State { index: state.index+1, ..state };
                    next_state.used_digits[digit.number] = true;
                    if let Some(solution) = solve_for_state(next_state, combinations) {
                        return Some(solution);
                    }
                },
            letters if letters == digit.segments.len() => {
                    // println!("generating new");
                    for permutation in unmatched_letters.iter().permutations(unmatched_letters.len()) {
                        let mut next_state = State { index: state.index+1, ..state};
                        next_state.used_digits[digit.number] = true;
                        for i in 0..permutation.len() {
                            next_state.segments[digit.segments[i] as usize] = Some(*permutation[i]);
                        }
                        if let Some(solution) = solve_for_state(next_state, combinations) {
                            return Some(solution);
                        }
                    }
                },
            _ => {
                // println!("Trying to match digit {:?} with {:?} combination failed since {:?}",
                // digit, unmatched_letters, state.segments);                    
                }
        }
    }
    return None;
} 
// Gives sorted vec, where [0] is combination for the digit "0", etc
fn solve(combinations: &Vec<String>) -> State {
    
    let mut sorted_by_len = combinations.clone();
    sorted_by_len.sort_by(|x, y| x.len().cmp(&y.len()) );
    
    let state = State {index: 0, segments: [None;7], used_digits: [false; 10] };
    let solution = solve_for_state (state, &sorted_by_len).unwrap();

    println!("===> Found a solution: {:?}", solution);

    return solution;
}

// combinations: bdcfga fag dgafc fg dacbf bdafec cfbg fabdgec eabdfg eagcd 