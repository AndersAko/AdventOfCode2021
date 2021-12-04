use std::fs;

fn main() {
    println!("Hello, world!");

    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");
    
    let contents: Vec<i32> = filecontents.split_terminator('\n').map(|l| l.trim().parse::<i32>().unwrap() ).collect();

    println!("{:?}", contents);

    // Part 1
    let mut prev_value = 0;
    let mut increases = -1;
 
    for num_value in &contents {
        println!("{}", num_value);

        if num_value > &prev_value {
            increases = increases + 1;
        }
        prev_value = *num_value;
    }
    println!("Part1: Number of increases = {}", increases);


    // Part 2
    let mut previous_avg : Option<i32> = None;
    let mut increases_part2 = 0;

    for line_num in 2..contents.len() {
        let moving_avg  = contents[line_num-2]+contents[line_num-1]+contents[line_num];
        if let Some(prev) = previous_avg {
            if moving_avg > prev {
                increases_part2 += 1;
            }
        }
        previous_avg = Some(moving_avg);
    }
    println!("Part2: number of increases = {}", increases_part2);
}
