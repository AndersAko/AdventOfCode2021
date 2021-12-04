use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");

    let filecontents = fs::read_to_string("input_test.txt").expect("Something went wrong?");
    
    let contents = filecontents.split_terminator('\n');
        
    let mut prev_value = 0;
    let mut increases = -1;

    for line in contents {
        println!("{}", line);

        let num_value : i64 = line.trim().parse().unwrap();
        if num_value > prev_value {
            increases = increases + 1;
        }
        prev_value = num_value;

    }
    println!("Number of increases {}", increases);
}
