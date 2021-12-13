use std::fs;
use std::collections::HashSet;

fn main() {
    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");

    let height_map:Vec<Vec<char>> = filecontents.split_terminator("\n").map(|x| x.chars().collect::<Vec<char>>()).collect();

    println!("{:?}", height_map);

    let mut risk_level = 0;
    let mut low_points = Vec::<(usize, usize)>::new();

    for y in 0..height_map.len() {
        for x in 0..height_map[0].len() {
            if x > 0 &&                     height_map[y][x-1] <= height_map[y][x] { continue; }
            if x < height_map[0].len()-1 && height_map[y][x+1] <= height_map[y][x] { continue; }
            if y > 0 &&                     height_map[y-1][x] <= height_map[y][x] { continue; }
            if y < height_map.len()-1 &&    height_map[y+1][x] <= height_map[y][x] { continue; }
            println!("Found a low point at {},{}:{}", x, y, height_map[y][x]);
            low_points.push((x,y));
            risk_level += 1 + height_map[y][x].to_digit(10).unwrap();
        }
    }
    println!("Part 1: Risk level is {}", risk_level);

    let mut basin_sizes = Vec::new();
    for basin in low_points {
        let size = get_basin_size(basin.0,basin.1, &height_map);
        println!("Basin {:?} = {}", basin, size);
        basin_sizes.push(size);
    }

    basin_sizes.sort_by(|a,b| b.cmp(a));
    let total_size:usize = basin_sizes.iter().take(3).fold(1, |acc, x| acc * x );
    println!("Part2: Total product of size of three largest basins = {}", total_size);

}

fn get_basin_size(x: usize, y: usize, height_map: &Vec<Vec<char>> ) -> usize {
    let mut size = 0;

    let mut visited = HashSet::new();
    let mut to_visit = Vec::new();

    visited.insert((x, y)); to_visit.push((x,y));

    while let Some((x,y)) = to_visit.pop() {
        if height_map[y][x] == '9' { continue; }
        size += 1;
        if x > 0                        && !visited.contains(&(x-1,y))  { visited.insert((x-1, y)); to_visit.push((x-1, y)); }
        if x < height_map[0].len()-1    && !visited.contains(&(x+1,y))  { visited.insert((x+1, y)); to_visit.push((x+1, y)); }
        if y > 0                        && !visited.contains(&(x,y-1))  { visited.insert((x, y-1)); to_visit.push((x, y-1)); }
        if y < height_map.len()-1       && !visited.contains(&(x,y+1))  { visited.insert((x, y+1)); to_visit.push((x, y+1)); }
    }

    size
}
