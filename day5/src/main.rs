use std::fs;
use std::cmp;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");
    let lines = (&filecontents).split_terminator('\n');

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
    }

    let mut vent_lines = Vec::<(i32,i32,i32,i32)>::new();

    for line in lines {
        let captures = &RE.captures(line).unwrap();

        let x1 = captures.name("x1").and_then(|m| m.as_str().parse::<i32>().ok()).unwrap(); 
        let x2 = captures.name("x2").and_then(|m| m.as_str().parse::<i32>().ok()).unwrap(); 
        let y1 = captures.name("y1").and_then(|m| m.as_str().parse::<i32>().ok()).unwrap(); 
        let y2 = captures.name("y2").and_then(|m| m.as_str().parse::<i32>().ok()).unwrap(); 

        // println!("x1 {:?} x2 {:?} y1 {:?} y2 {:?}", x1, x2, y1, y2);
        vent_lines.push((x1,x2,y1,y2));
    }

    let mut crossings = HashMap::<(i32,i32),i32>::new();
    for vent_line in &vent_lines {
        if vent_line.0 == vent_line.1 { // Same x
            for y in cmp::min(vent_line.2,vent_line.3)..=cmp::max(vent_line.2,vent_line.3) {
                crossings.entry((vent_line.0, y)).and_modify(|x| *x+=1).or_insert(1);
            }
        } else if vent_line.2 == vent_line.3 {  // Same y
            for x in cmp::min(vent_line.0,vent_line.1)..=cmp::max(vent_line.0,vent_line.1) {
                crossings.entry((x, vent_line.2)).and_modify(|x| *x+=1).or_insert(1);
            }
        }
    }
    let result = crossings.values().filter(|&x| *x > 1 ).count(); 
    println!("Part 1: The lines overlap in {} points", result);

    let mut crossings = HashMap::<(i32,i32),i32>::new();
    for vent_line in vent_lines {
        if vent_line.0 == vent_line.1 { // Same x
            for y in cmp::min(vent_line.2,vent_line.3)..=cmp::max(vent_line.2,vent_line.3) {
                crossings.entry((vent_line.0, y)).and_modify(|x| *x+=1).or_insert(1);
            }
        } else if vent_line.2 == vent_line.3 {  // Same y
            for x in cmp::min(vent_line.0,vent_line.1)..=cmp::max(vent_line.0,vent_line.1) {
                crossings.entry((x, vent_line.2)).and_modify(|x| *x+=1).or_insert(1);
            }
        } else if (vent_line.0 - vent_line.1).abs() == (vent_line.2 - vent_line.3).abs() {  // Diagonal
            let d = (vent_line.0 - vent_line.1).abs();
            let dx = (vent_line.1 - vent_line.0) / d;
            let dy = (vent_line.3 - vent_line.2) / d;
            for i in 0..=d {
                crossings.entry((vent_line.0+i*dx, vent_line.2+i*dy)).and_modify(|x| *x+=1).or_insert(1);
            }
        }
    }
    for y in 0..=9 {
        for x in 0..=9 {
            print!("{}", crossings.get(&(x,y)).unwrap_or(&0));
        }
        println!();
    }


    let result = crossings.values().filter(|&x| *x > 1 ).count(); 
    println!("Part 2: The lines overlap in {} points", result);

}
