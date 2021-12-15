use std::fs;
// use std::cmp;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

fn main() {
    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");
    let cavern: Vec<Vec<_>> = filecontents.split_terminator("\n")
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()).collect();


    let risk_level = solve(&cavern, false);

    println!("Part 1: risk level {}", risk_level);

    let risk_level = solve(&cavern, true);

    println!("Part 2: risk level {}", risk_level);

}

#[derive(Copy, Clone, Eq, PartialEq,Debug)]
struct State {
    pos: (i32, i32),
    risk: i32,
    cost: i32 
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve (cavern: &Vec<Vec<i32>>, part2: bool) -> i32 {
    let dir:[(i32,i32);4] = [(0,-1), (-1,0), (1, 0), (0,1)];

    let mut visited = HashMap::<(i32, i32), i32>::new();

    let mut queue = BinaryHeap::new();
    let start_pos = (0,0);
    visited.insert(start_pos, 0);
    let size = cavern.len();
    let target = if part2 { 
        (size as i32 * 5 - 1, size as i32 * 5 - 1)
    } else {
        (size as i32 - 1, size as i32 - 1)
    };
    let mut count = 0; 

    queue.push(State { pos: start_pos, risk: 0, cost: 0 });

    while let Some(next) = queue.pop() {
        count += 1; 
        if next.pos == target {
            println!("Found a way to {:?} with a risk level of {} after {} tested locations", target, next.risk, count);

            return next.risk;
        }
        for (dx, dy) in dir {
            let (x,y) = (next.pos.0 as i32+dx, next.pos.1 as i32+dy);
            if x >=0 && x<=target.0 as i32 && y >= 0 && y <= target.1 as i32 {
                let mut risk = cavern[y as usize % size][x as usize % size] + 
                    if part2 { x / size as i32 + y / size as i32 } else { 0 };
                if risk > 9 { risk -= 9 }
                risk = next.risk + risk;
                let cost = - y - x + risk as i32;
                if let Some(prev_cost) = visited.get(&(x,y)) {
                    if *prev_cost > cost {
                        // println!("Found lower cost to {:?} ({} vs {})", (x,y), cost, prev_cost);
                    } else {
                        continue;
                    }
                }
                let state = State{ pos: (x,y), risk, cost}; 
                // println!("Pushing state {:?}", &state);
                queue.push(state);
                visited.insert((x,y), cost);
            }
        }
    }
    0
}

