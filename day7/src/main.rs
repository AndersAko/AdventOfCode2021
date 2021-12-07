use std::fs;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Mutex;

fn main() {
    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");
    let mut crabs = filecontents.split_terminator(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    crabs.sort();

    let midpoint = crabs[crabs.len()/2];
    let mut fuel = 0;
    for crab in &crabs {
        fuel += (midpoint - crab).abs();
    }
    println!("Part 1: Fuel consumption to reach {} = {}", midpoint, fuel);

    let mut point = midpoint;
    let mut fuel = fuel_part2(&crabs, point);
    let mut dir = 1;
    let mut visited = HashSet::<i32>::new();
    loop {
        println!("{} {}", point, fuel);
        point += dir;
        if visited.contains(&point) { break; }
        let new_fuel = fuel_part2(&crabs, point);
        if new_fuel > fuel {dir = -dir;}
        fuel = new_fuel;
        visited.insert(point);
    }
    println!("Found a nice spot at {} costing {}", point, fuel_part2(&crabs, point));
}

fn fuel_cost(steps : i32) -> i32 {
    lazy_static! {
        static ref COSTS: Mutex<HashMap::<i32, i32>> = Mutex::new(HashMap::<i32, i32>::new());
    }
    if !COSTS.lock().unwrap().contains_key(&steps) {
        if steps == 0 { 
            COSTS.lock().unwrap().insert(0, 0);
        } else if steps == 1 { 
            COSTS.lock().unwrap().insert(1, 1);
        } else {
            let cost_one_step_less = fuel_cost(steps-1);
            COSTS.lock().unwrap().insert(steps, steps + cost_one_step_less );
        }
        println!("Cost of {} steps: {}", steps, COSTS.lock().unwrap()[&steps]);
    }
    COSTS.lock().unwrap()[&steps]
}

fn fuel_part2(crabs:&Vec<i32>, point: i32) -> i32 {
    let mut fuel = 0;

    for crab in crabs {
        fuel += fuel_cost((point - crab).abs())
    }
    println!("Fuel to point {} = {}", point, fuel);
    fuel
}
