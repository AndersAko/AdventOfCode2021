use std::fs;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone, Debug)]
struct PolymerResult {
    polymer: String,
    elements: HashMap<char, usize>    // element, count
}

fn main() {
    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");

    let lines: Vec<&str> = filecontents.split_terminator("\n").collect();

    let template = lines[0];
    let mut rules = HashMap::<String, String>::new();

    for i in 2..lines.len() {
        let parsed: Vec<&str> = lines[i].split(" -> ").collect();
        rules.insert(String::from(parsed[0]), 
            format!("{}{}{}", &parsed[0][..1], String::from(parsed[1]), &parsed[0][1..] ));    
    }
    println!("Rules: {:?}", rules);
    let solver = Solver { rules, 
        cache: Mutex::new(HashMap::new()) };
    
    println!("=== 1 step ===");
    println!("{:?}", solver.solve(template, 1));

    println!("=== 2 step ===");
    println!("{:?}", solver.solve(template, 2));

    println!("=== 3 step ===");
    println!("{:?}", solver.solve(template, 3));

    println!("=== 4 step ===");
    println!("{:?}", solver.solve(template, 4));

    println!("=== 10 steps ===");
    let result = solver.solve(template, 10);
    println!("{:?}", result.elements);
    let max = *result.elements.iter().max_by(|(k0,v0), (k1,v1)| v0.cmp(&v1) ).unwrap().1;
    let min = *result.elements.iter().min_by(|(k0,v0), (k1,v1)| v0.cmp(&v1) ).unwrap().1;
    println!("Part 1 answer: {}  (Max: {} Min: {}) ", max - min , max, min);

    println!("=== 40 steps ===");
    let result = solver.solve(template, 40);
    println!("{:?}", result.elements);
    let max = *result.elements.iter().max_by(|(k0,v0), (k1,v1)| v0.cmp(&v1) ).unwrap().1;
    let min = *result.elements.iter().min_by(|(k0,v0), (k1,v1)| v0.cmp(&v1) ).unwrap().1;
    println!("Part 2 answer: {}  (Max: {} Min: {}) ", max - min , max, min);

}

struct Solver {
    rules: HashMap<String, String>,
    cache: Mutex<HashMap<(String, isize), PolymerResult>>
}

impl Solver {
    fn solve (&self, template: &str, steps: isize) -> PolymerResult {
        let mut result = PolymerResult { polymer: String::from(&template[0..1]), elements: HashMap::new()};
        result.elements.insert(template.chars().next().unwrap(), 1);

        for i in 0..template.len()-1 {
            let pair = &template[i..i+2];
            let solved = self.solve_pair(pair, steps);
            // result.polymer += &solved.polymer;
            for e in solved.elements {
                let elem = result.elements.entry(e.0).or_insert(0);
                *elem += e.1;
            }
        }
        result
    }
    fn count_elements (&self, polymer: &str) -> HashMap<char, usize> {
        let mut elements = HashMap::<char, usize>::new();
        for c in polymer.chars() {
            let count = elements.entry(c).or_insert(0);
            *count += 1;
        }
        elements
    }

    fn solve_pair (&self, pair: &str, steps: isize) -> PolymerResult {
        if let Some(cached_result) = self.cache.lock().unwrap().get(&(String::from(pair), steps)) {
            // println!("cached {:?}", *cached_result);
            return (*cached_result).clone();
        }
        let mut result = PolymerResult { polymer: String::from(""), elements: HashMap::new()};

        if steps == 0 {
            result.polymer = String::from(&pair[1..]);
            result.elements = self.count_elements(&result.polymer)
        } else {
            let polymer = &self.rules[&String::from(pair)];
            let first_part = self.solve_pair(&polymer[0..2], steps - 1 );
            let second_part = self.solve_pair(&polymer[1..3], steps -1 );
            // result.polymer = first_part.polymer;
            // result.polymer.push_str(&second_part.polymer);
            result.elements = first_part.elements;
            for e in second_part.elements {
                let elem = result.elements.entry(e.0).or_insert(0);
                *elem += e.1;
            }
        }
        self.cache.lock().unwrap().insert((String::from(pair), steps), result.clone());
        result
    }
}

