use std::fs;

fn main() {
    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");

    let lines: Vec<Vec<char>> = filecontents.split_terminator("\n").map(|x| x.chars().collect::<Vec<char>>()).collect();

    let mut score = 0;
    'lines1: for line in &lines {
        let mut stack = Vec::new();
        for c in line {
            match c {
                '<' | '(' | '[' | '{' => stack.push(c),
                '}' | ']' | ')' | '>' => {
                    let match_ok = match stack.pop() {
                        Some('<') => *c=='>',
                        Some('(') => *c==')',
                        Some('[') => *c==']',
                        Some('{') => *c=='}',
                        _ => false
                    };
                    if !match_ok {
                        let error_score = match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => 0
                        }; 
                        score += error_score;
                        println!("Corrupted line, got unexpected {}, with score {} ({})", c, error_score, score);
                        continue 'lines1;
                    }
                }
                _ => ()
            }
        }
    }
    println!("Part 1: syntax error score: {} ", score);

    let mut scores = Vec::new();
    'lines2: for line in &lines {
        let mut stack = Vec::new();

        for c in line {
            match c {
                '<' | '(' | '[' | '{' => stack.push(c),
                '}' | ']' | ')' | '>' => {
                    let match_ok = match stack.pop() {
                        Some('<') => *c=='>',
                        Some('(') => *c==')',
                        Some('[') => *c==']',
                        Some('{') => *c=='}',
                        _ => false
                    };
                    if !match_ok {
                        // println!("Corrupted line, got unexpected {}", c);
                        continue 'lines2;
                    }
                }
                _ => ()
            }
        }
        let mut score:u128 = 0;
        while let Some(open) = stack.pop() {
            score = score * 5 + match open {
                '<' => 4,
                '(' => 1, 
                '[' => 2,
                '{' => 3,
                _ => 0
            };
        }
        scores.push(score);
        println!("Score for uncompleted line {}", score);
    }
    scores.sort();
    println!("Part2: Middle score {}", scores[scores.len()/2]);
}
