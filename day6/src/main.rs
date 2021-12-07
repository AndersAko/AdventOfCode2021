use std::fs;

fn main() {
    
    let filecontents = fs::read_to_string("input_test.txt").expect("Something went wrong?");
    println!("{}", &filecontents.len());
    
    let mut fishes = filecontents.split_terminator(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    for day in 1..=256 {
        for i in 0..fishes.len() {
            if fishes[i] > 0 {
                fishes[i] -= 1;
            } else {
                fishes[i] = 6;
                fishes.push(8);
            }
        }
        if day <= 18 { println!("After day {} ({}) {:?}", day, fishes.len(), fishes ); }
        else { println!("After day {} ({}) ", day, fishes.len() ); }
    }
}
