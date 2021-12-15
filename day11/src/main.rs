use std::fs;

fn main() {
    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");
    let mut octopi = filecontents.split_terminator('\n')
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap() as u8).collect::<Vec<u8>>())
        .collect::<Vec<_>>();

    println!("Starting");
    // print_octopi(&octopi);

    let mut flashes = 0;
    for step in 1..=100 {
        for row in &mut octopi {
            for oct in row {
                *oct += 1;
            }
        }
        // println!("--");
        // print_octopi(&octopi);

        for row in 0..octopi.len() {
            for col in 0..octopi[0].len() {
                if octopi[row][col] == 10 {  // Flashing octopus
                    flash_octopus(col as i32, row as i32, &mut octopi, &mut flashes);
                }
            }
        }
        // println!("--");
        // print_octopi(&octopi);

        for row in &mut octopi {
            for oct in row {
                if *oct>9 { *oct = 0 }
            }
        }
        // println!();
        // println!("After step {}", step);
        // print_octopi(&octopi);
        // println!();
    }
    println!("Part1 {} flashes", flashes);

    let mut octopi = filecontents.split_terminator('\n')
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap() as u8).collect::<Vec<u8>>())
        .collect::<Vec<_>>();
    let mut flashes = 0;
    for step in 1..=1000 {
        for row in &mut octopi {
            for oct in row {
                *oct += 1;
            }
        }
        for row in 0..octopi.len() {
            for col in 0..octopi[0].len() {
                if octopi[row][col] == 10 {  // Flashing octopus
                    flash_octopus(col as i32, row as i32, &mut octopi, &mut flashes);
                }
            }
        }
        for row in &mut octopi {
            for oct in row {
                if *oct>9 { *oct = 0 }
            }
        }
        if octopi.iter().all(|row| row.iter().all(|&o| o == 0 )) {
            println!("Part 2: Synchronized flash at step {}", step);
            // print_octopi(&octopi);
            break;
        }
    }


}

// Flash the optopus at x,y
fn flash_octopus(x: i32, y :i32, octopi: &mut Vec<Vec<u8>>, flashes: & mut u128) {
    let dir:[(i32,i32);8] = [(-1,-1), (0,-1), (1,-1), (-1,0), (1,0), (-1,1), (0,1), (1,1)];
    // print!("[{},{}: ",x,y);
    *flashes += 1;
    octopi[y as usize][x as usize] = 11;
    for (dx,dy) in dir {
        flash_octopi( x + dx, y + dy, octopi, flashes);
    }
    // print!("]");
}


fn flash_octopi(x: i32, y :i32, octopi: &mut Vec<Vec<u8>>, flashes: &mut u128) {
    if  x >= 0 && (x as usize) < octopi[0].len() && 
        y >= 0 && (y as usize) < octopi.len() && 
        octopi[y as usize][x as usize] < 10 {

        // print!("{},{} ",x,y);
        octopi[y as usize][x as usize] += 1;
        if octopi[y as usize][x as usize] == 10 {
            flash_octopus(x,y, octopi, flashes)
        }
    }
}

fn print_octopi(octopi : &Vec<Vec<u8>>) {
    for row in octopi {
        for oct in row {
            print!("{}", oct);
        }
        println!();
    }
}
