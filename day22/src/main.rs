use std::fs;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Failed");
    
    let step_re = Regex::new(r"(?P<state>\w+) x=(?P<xmin>[\d-]+)..(?P<xmax>[\d-]+),y=(?P<ymin>[\d-]+)..(?P<ymax>[\d-]+),z=(?P<zmin>[\d-]+)..(?P<zmax>[\d-]+)").unwrap();
    let mut steps = Vec::<Step>::new();
    for line in file_contents.split_terminator("\n") {
        if let Some(instr) = step_re.captures(line) {
            let step  = Step {
                on : instr.name("state").and_then(|m| Some(m.as_str()== "on") ).unwrap(),
                range: [ 
                    MinMax { min: instr.name("xmin").unwrap().as_str().parse::<i32>().unwrap(), max: instr.name("xmax").unwrap().as_str().parse::<i32>().unwrap() }, 
                    MinMax { min: instr.name("ymin").unwrap().as_str().parse::<i32>().unwrap(), max: instr.name("ymax").unwrap().as_str().parse::<i32>().unwrap() }, 
                    MinMax { min: instr.name("zmin").unwrap().as_str().parse::<i32>().unwrap(), max: instr.name("zmax").unwrap().as_str().parse::<i32>().unwrap() }, 
                ]
            };
            steps.push(step);
        }
    }
    // println!("Steps: {:?}", steps);

    let mut lit_cubes = 0;
    'step_loop: for i in 0..steps.len() {
        for coord in 0..3 {
            if steps[i].range[coord].max < -50 || steps[i].range[coord].min > 50 {
                // println!("Skipping {} (outside init zone)", fmt_cube(&steps[i].range));
                continue 'step_loop;
            }
        }
        if !steps[i].on { 
            // println!("Skipping {} (Turns off)", fmt_cube(&steps[i].range));
            continue;
        }
        let non_covered_cubes = get_non_covered_cubes(&steps[i].range, i, &steps); 
        // println!("From the step {}, the part not covered by later steps are {:?}", i, fmt_cubes(&non_covered_cubes));
        let affected_cubes:u128 = non_covered_cubes.iter().map(|x| count_cubes(x)).sum();
        lit_cubes += affected_cubes;
        
        println!("Lit cubes: {}", lit_cubes);
    }
    println!("Part 1: lit cubes {}", lit_cubes);
    println!();

    let mut lit_cubes = 0;
    print!("Lit cubes: " );
    for i in 0..steps.len() {
        if !steps[i].on { 
            // println!("Skipping {} (Turns off)", fmt_cube(&steps[i].range));
            continue;
        }
        let non_covered_cubes = get_non_covered_cubes(&steps[i].range, i, &steps); 
        // println!("From the step {}, the part not covered by later steps are {:?}", i, fmt_cubes(&non_covered_cubes));
        let affected_cubes:u128 = non_covered_cubes.iter().map(|x| count_cubes(x)).sum();
        lit_cubes += affected_cubes;
        
        print!("{}  ", lit_cubes);
    }
    println!();
    println!("Part 2: lit cubes {}", lit_cubes);

}

#[derive(Debug, Clone)]
struct MinMax {min: i32, max: i32 }

type Cube = [MinMax ;3];  // (min, max) from x to z 

#[derive(Debug, Clone)]
struct Step {
    on: bool,
    range: Cube
}

fn get_non_covered_cubes(cube: &Cube, step_no: usize, steps: &Vec<Step>) -> Vec<Cube>{
    // println!("Checking coverage of {:?} ({})", cube, step_no);
    if step_no < steps.len()-1 {
        let covering_cube = &(steps[step_no+1].range);
        for coord in 0..3 {
            if cube[coord].min > covering_cube[coord].max || cube[coord].max < covering_cube[coord].min {
                return get_non_covered_cubes(&cube, step_no+1, steps);
            }
        }
        let mut result = Vec::new();
        let mut remaining = cube.clone();
        for coord in 0..3 {
            if remaining[coord].min < covering_cube[coord].min {
                let mut un_covered_part = remaining.clone();
                if remaining[coord].max >= covering_cube[coord].min {
                    un_covered_part[coord].max = covering_cube[coord].min - 1;
                    remaining[coord].min = covering_cube[coord].min;
                    // println!("Partly uncovered (min) ({}) {:?}. remaining: {:?}", step_no, fmt_cube(&un_covered_part), fmt_cube(&remaining)); 
                    result.extend( get_non_covered_cubes(&un_covered_part, step_no+1, steps));
                } else {
                    panic!();   // Checked initially
                }
            }
            if remaining[coord].max > covering_cube[coord].max {
                let mut un_covered_part = remaining.clone();
                if remaining[coord].min <= covering_cube[coord].max {
                    un_covered_part[coord].min = covering_cube[coord].max + 1;
                    remaining[coord].max = covering_cube[coord].max;
                    // println!("Partly uncovered (max) ({}) {:?}. remaining: {:?}", step_no, fmt_cube(&un_covered_part), fmt_cube(&remaining)); 
                    result.extend( get_non_covered_cubes(&un_covered_part, step_no+1, steps));
                } else {
                    panic!();   // Checked initially
                }
            }
        }
        return result;
    } else {
        return vec![cube.clone()];
    }
}

fn count_cubes(c: &Cube) -> u128 {
    (c[0].max-c[0].min + 1) as u128 * (c[1].max-c[1].min + 1) as u128 * (c[2].max-c[2].min + 1) as u128
}

fn _fmt_cube(c: &Cube) -> String {
    format!("x={}..{},y={}..{},z={}..{} ({})",c[0].min,c[0].max,c[1].min,c[1].max,c[2].min,c[2].max, count_cubes(c) )
}
fn _fmt_cubes(cubes : &Vec<Cube>) -> String {
    cubes.iter().map(|x| _fmt_cube(x)).collect::<Vec<_>>().join("; ")
}
#[test]
fn test_count_cubes() {
    let cube = [MinMax { min: 10, max: 12}, MinMax { min: 10, max: 12}, MinMax { min: 10, max: 12}];
    assert_eq!(count_cubes(&cube), 27);

    let cube = [MinMax { min: 10, max: 10}, MinMax { min: 10, max: 10}, MinMax { min: 10, max: 10}];
    assert_eq!(count_cubes(&cube), 1);

    let cube =  [MinMax { min: 10, max: 11}, MinMax { min: 10, max: 10}, MinMax { min: 10, max: 10}];
    assert_eq!(count_cubes(&cube), 2);

    let cube = [MinMax { min: 10, max: 10}, MinMax { min: 10, max: 11}, MinMax { min: 10, max: 10}];
    assert_eq!(count_cubes(&cube), 2);

    let cube = [MinMax { min: 10, max: 10}, MinMax { min: 10, max: 10}, MinMax { min: 10, max: 11}];
    assert_eq!(count_cubes(&cube), 2);
}

#[test]
fn test_get_non_covered_min_side() {
    let cube1 = Step { on: true, range: [MinMax { min: 10, max: 12}, MinMax { min: 10, max: 12}, MinMax { min: 10, max: 12}]};
    let cube2 = Step { on: true, range: [MinMax { min: 11, max: 13}, MinMax { min: 11, max: 13}, MinMax { min: 11, max: 13}]};
    
    let steps = vec![cube1.clone(), cube2.clone()];

    let cube1_non_covered = get_non_covered_cubes(&cube1.range, 0, &steps);
    println!("Non covered parts: {:?}", _fmt_cubes(&cube1_non_covered));

    let no_cubes: u128= cube1_non_covered.iter().map(|c| count_cubes(c)).sum();

    println!("A total of {} cubes in step 1 was not covered by step 2", no_cubes);
    assert_eq!(no_cubes, 19);
}

#[test]
fn test_get_non_covered_max_side() {
    let cube1 = Step { on: true, range: [MinMax { min: 11, max: 13}, MinMax { min: 11, max: 13}, MinMax { min: 11, max: 13}]};
    let cube2 = Step { on: true, range: [MinMax { min: 10, max: 12}, MinMax { min: 10, max: 12}, MinMax { min: 10, max: 12}]};
    
    let steps = vec![cube1.clone(), cube2.clone()];

    let cube1_non_covered = get_non_covered_cubes(&cube1.range, 0, &steps);
    println!("Non covered parts: {:?}", _fmt_cubes(&cube1_non_covered));

    let no_cubes:u128 = cube1_non_covered.iter().map(|c| count_cubes(c)).sum();

    println!("A total of {} cubes in step 1 was not covered by step 2", no_cubes);
    assert_eq!(no_cubes, 19);
}

#[test]
fn test_small_example () {
    let cube1 = Step { on: true, range: [MinMax { min: 11, max: 13}, MinMax { min: 11, max: 13}, MinMax { min: 11, max: 13}]};
    let cube2 = Step { on: true, range: [MinMax { min: 10, max: 12}, MinMax { min: 10, max: 12}, MinMax { min: 10, max: 12}]};
    let cube3 = Step { on: false, range: [MinMax { min: 9, max: 11}, MinMax { min: 9, max: 11}, MinMax { min: 9, max: 11}]};
    let cube4 = Step { on: true, range: [MinMax { min: 10, max: 10}, MinMax { min: 10, max: 10}, MinMax { min: 10, max: 10}]};
    
    let steps = vec![cube1.clone(), cube2.clone(), cube3.clone(), cube4.clone()];

    let mut lit_cubes:u128 = 0;
    for step in 0..steps.len() {
        if !steps[step].on { continue }
        let non_covered = get_non_covered_cubes(&steps[step].range, step, &steps);
        lit_cubes = lit_cubes + non_covered.iter().map(|x| count_cubes(x)).sum::<u128>();
    }
    assert_eq!(lit_cubes, 39);
}

fn _alt_count_uncovered_cubes(step_no: usize, steps: &Vec<Step>) -> u128 {
    let mut cubes = HashSet::new();

    let cube = &steps[step_no].range;
    for x in cube[0].min..=cube[0].max {
        for y in cube[1].min..=cube[1].max {
            for z in cube[2].min..=cube[2].max {
                cubes.insert((x,y,z));
            }
        }
    }
    assert_eq!(cubes.len(), count_cubes(&cube) as usize);
    print!("{} ", cubes.len());
    if step_no < steps.len()-1 {
        for step in (step_no+1)..steps.len() {
            let cube = &steps[step].range;
            for x in cube[0].min..=cube[0].max {
                for y in cube[1].min..=cube[1].max {
                    for z in cube[2].min..=cube[2].max {
                        cubes.remove(&(x,y,z));
                    }
                }
            }
            print!("{} ", cubes.len());
        }
    }
    cubes.len() as u128
}

#[test]
fn test_larger_example() {

    let step_re = Regex::new(r"(?P<state>\w+) x=(?P<xmin>[\d-]+)..(?P<xmax>[\d-]+),y=(?P<ymin>[\d-]+)..(?P<ymax>[\d-]+),z=(?P<zmin>[\d-]+)..(?P<zmax>[\d-]+)").unwrap();
    let mut steps = Vec::<Step>::new();
    let file_contents = fs::read_to_string("input_test.txt").expect("Failed");
    for line in file_contents.split_terminator("\n") {
        if let Some(instr) = step_re.captures(line) {
            let step  = Step {
                on : instr.name("state").and_then(|m| Some(m.as_str()== "on") ).unwrap(),
                range: [ 
                    MinMax { min: instr.name("xmin").unwrap().as_str().parse::<i32>().unwrap(), max: instr.name("xmax").unwrap().as_str().parse::<i32>().unwrap() }, 
                    MinMax { min: instr.name("ymin").unwrap().as_str().parse::<i32>().unwrap(), max: instr.name("ymax").unwrap().as_str().parse::<i32>().unwrap() }, 
                    MinMax { min: instr.name("zmin").unwrap().as_str().parse::<i32>().unwrap(), max: instr.name("zmax").unwrap().as_str().parse::<i32>().unwrap() }, 
                ]
            };
            if step.range[0].max < -50 || step.range[0].min > 50 { continue; }
            steps.push(step);
        }
    }

    for step_no in 0..steps.len() {
        let non_covered = get_non_covered_cubes(&steps[step_no].range, step_no, &steps);
        let method1_count = non_covered.iter().map(|x| count_cubes(x)).sum::<u128>();

        let method2_count = _alt_count_uncovered_cubes(step_no, &steps);
        // assert_eq!(method1_count, method2_count);
        println!("Method 1: {} Method2 {}", method1_count, method2_count);
    }

}

#[test]
fn test_part_of_larger_example () {
    let steps = vec![
        Step { on: true, range: [MinMax { min: -49, max: -5 }, MinMax { min: -3, max: 45 }, MinMax { min: -29, max: 18 }] },
        Step { on: true, range: [MinMax { min: 18, max: 30 }, MinMax { min: -20, max: -8 }, MinMax { min: -3, max: 13 }] },
        Step { on: true, range: [MinMax { min: -41, max: 9 }, MinMax { min: -7, max: 43 }, MinMax { min: -33, max: 15 }]}
    ];
    
    let non_covered = get_non_covered_cubes(&steps[0].range, 0, &steps);
    let method1_count = non_covered.iter().map(|x| count_cubes(x)).sum::<u128>();
    let method2_count = _alt_count_uncovered_cubes(0, &steps);

    println!("Method 1: {} Method2 {}", method1_count, method2_count);
    assert_eq!(method1_count, method2_count);
}