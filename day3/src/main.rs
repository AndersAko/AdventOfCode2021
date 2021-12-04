use std::fs;

fn main() {
    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");
    let mut lines = filecontents.split_terminator('\n').collect::<Vec<&str>>();

    // println!("Input: {:?}", &lines);

    let wordsize = lines[0].len();
    println!("Wordsize {}", wordsize);

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..wordsize {
        gamma = gamma * 2;
        epsilon *= 2;

        if ones_most_common(&lines, i)=="1" {
            gamma += 1; 
        } else {
            epsilon += 1;
        }
    }
    println!("Part1: Gamma {} Eplsion {} Power consumption: {}", gamma, epsilon, gamma*epsilon);    

    // Oxygen: keep values with bit = most common
    for i in 0..wordsize {
        let most_common = ones_most_common(&lines, i);
        lines = lines.iter().filter(|&line| &line[i..i+1] == most_common).map(|x| *x).collect::<Vec<&str>>();

        let &count = &lines.len();

        if count == 1 {
            println!("Only one remaining at bit: {} {:?}", i, lines[0]);
            break;
        }
        println!("Bit {} lines remaining {} -- {:?}", i, count, &lines);
    }
    let oxygen = isize::from_str_radix(lines[0],2).unwrap();

    lines = filecontents.split_terminator('\n').collect::<Vec<&str>>();
    // CO2: keep values with bit = least common
    for i in 0..wordsize {
        let most_common = ones_most_common(&lines, i);
        lines = lines.iter().filter(|&line| &line[i..i+1] != most_common).map(|x| *x).collect::<Vec<&str>>();

        let &count = &lines.len();

        if count == 1 {
            println!("Only one remaining at bit: {} {:?}", i, lines[0]);
            break;
        }
        println!("Bit {} lines remaining {} -- {:?}", i, count, &lines);
    }
    let co2 = isize::from_str_radix(lines[0], 2).unwrap();

    println!("Oxygen {} CO2 {} => Life support {}", oxygen, co2, oxygen * co2);
}

fn ones_most_common<'a>(lines: &Vec<&str> , bitno: usize) -> &'a str {
    let mut more_ones=0;
    for line in lines {
        if &line[bitno..bitno+1] == "1" {
            more_ones+=1;
        } else {
            more_ones-=1;
        }
    }
    return if more_ones >= 0 {"1"} else {"0"};
}
