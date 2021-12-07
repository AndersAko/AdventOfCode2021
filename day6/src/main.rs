use std::fs;

fn main() {
    
    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");
    println!("{}", &filecontents.len());
    
    let fishes = filecontents.split_terminator(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut fish_ages = [0;9];
    for &f in &fishes {
        fish_ages[f as usize] += 1;
    }
    println!("Fishes: {:?} Fish_ages {:?}", &fishes, fish_ages);
    
    for day in 1..=256 {
        let mut next_day_ages = [0;9];

        for i in 0..fish_ages.len() {
            if i==0 {
                next_day_ages[6] += fish_ages[0];
                next_day_ages[8] += fish_ages[0];
            } else {
                next_day_ages[i-1] += fish_ages[i];
            }
        }
        fish_ages = next_day_ages;
        let sum: u128 = fish_ages.iter().sum();
        println!("After day {} ({}) {:?}", day, sum , fish_ages );
    }
}
