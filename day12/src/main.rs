use std::fs;
use itertools::Itertools;

fn main() {
    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");
    let paths = filecontents
        .split_terminator("\n")
        .map(|x| { 
            let mut r = x.split("-");
            (r.next().unwrap(),r.next().unwrap())
        }).collect::<Vec<(&str,&str)>>();
  
    println!("Paths: {:?} ", paths);

    let mut stack = Vec::new();
    let mut count = 0;
    stack.push(vec!["start"]);
    while let Some(node) = stack.pop() {
        let current_cave = *node.last().unwrap();
        if current_cave == "end" {
            println!("Found a path {:?}", node);
            count += 1;
        } else {
            print!("Cave: {}", current_cave);
            let connecting_caves = 
            paths.iter().filter(|&&p| p.0 == current_cave)
                        .map(|&p| p.1)
            .chain(paths.iter().filter(|&&p| p.1 == current_cave)
                        .map(|&p| p.0));
            for c in connecting_caves {
                print!(" {},", c);

                if c.chars().all(|c| c.is_ascii_uppercase() ) || !node.contains(&c) {
                    let mut next = node.clone();
                    next.push(c);
                    stack.push(next);
                }
            }
            println!();
        }
    }
    println!("Part 1:  A total of {} paths\n", count);

    let mut stack = Vec::new();
    let mut count = 0;
    stack.push(vec!["start"]);
    while let Some(node) = stack.pop() {
        let current_cave = *node.last().unwrap();
        if current_cave == "end" {
            println!("Found a path {:?}", node);
            count += 1;
        } else {
            print!("Cave: {}", current_cave);
            let connecting_caves = 
            paths.iter().filter(|&&p| p.0 == current_cave)
                        .map(|&p| p.1)
            .chain(paths.iter().filter(|&&p| p.1 == current_cave)
                        .map(|&p| p.0));
            for cave in connecting_caves {
                print!(" {},", cave);
                if cave == "start" { continue; }
                if cave.chars().all(|c| c.is_ascii_uppercase() ) || !node.contains(&cave) || 
                    node.iter().filter(|&x| x.chars().all(|c| c.is_ascii_lowercase())).all_unique()
                {
                    let mut next = node.clone();
                    next.push(cave);
                    stack.push(next);
                }

            }
            println!();
        }
    }
    println!("Part 2:  A total of {} paths", count);


}
