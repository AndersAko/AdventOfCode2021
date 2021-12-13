use std::fs;

fn main() {
    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");

    let height_map:Vec<Vec<char>> = filecontents.split_terminator("\n").map(|x| x.chars().collect::<Vec<char>>()).collect();

    println!("{:?}", height_map);

    let mut risk_level = 0;
    for y in 0..height_map.len() {
        for x in 0..height_map[0].len() {
            if x > 0 &&                     height_map[y][x-1] <= height_map[y][x] { continue; }
            if x < height_map[0].len()-1 && height_map[y][x+1] <= height_map[y][x] { continue; }
            if y > 0 &&                     height_map[y-1][x] <= height_map[y][x] { continue; }
            if y < height_map.len()-1 &&    height_map[y+1][x] <= height_map[y][x] { continue; }
            println!("Found a low point at {},{}:{}", x, y, height_map[y][x]);
            risk_level += 1 + height_map[y][x].to_digit(10).unwrap();
        }
    }
    println!("Part 1: Risk level is {}", risk_level);
}
