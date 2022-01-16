use std::fs;
use regex::Regex;
extern crate nalgebra as na;
use na::{ Rotation3, Vector3 };
use std::collections::HashMap;
use std::f64;

fn main() {
    let filecontents = fs::read_to_string("input.txt").expect("Failed");
    
    let mut lines = filecontents.split_terminator("\n");

    let scanner_start_re = Regex::new(r"^--- scanner (\d+) ---$").unwrap();
    let reading_re = Regex::new(r"(-?\d+),(-?\d+),(-?\d+)").unwrap();
    let mut readings: Readings = Vec::new();
    while let Some(line) = lines.next() {
        if scanner_start_re.is_match(line) {
        // let Some(m) = scanner_start_re.captures(line) {
            // println!("Scanner {} start", m.get(1).map_or("", |x| x.as_str()) );
            readings.push(Vec::new());
            continue;
        }
        if let Some(coords) = reading_re.captures(line) {
            let reading  = Coord::new(coords.get(1).unwrap().as_str().parse::<f64>().unwrap(), 
                                        coords.get(2).unwrap().as_str().parse::<f64>().unwrap() ,
                                        coords.get(3).unwrap().as_str().parse::<f64>().unwrap() );
            readings.last_mut().unwrap().push(reading);
            // println!("{:?}", reading);
        } 
    }
    let scanners = find_scanners(&readings);
    println!("Identified scanner positions:");
    for scanner in &scanners {
        println!("{:?}", scanner);
    }

    let unified_readings = unify_readings(&scanners, &readings);
    println!("Part 1: There are a total of {} beacons", unified_readings.len());

    let mut max_dist = 0;
    for scanner_a in &scanners {
        for scanner_b in &scanners {
            let dist =  ((scanner_a.abs_position.unwrap()[0]-scanner_b.abs_position.unwrap()[0]).abs() +
                            (scanner_a.abs_position.unwrap()[1]-scanner_b.abs_position.unwrap()[1]).abs() + 
                            (scanner_a.abs_position.unwrap()[2]-scanner_b.abs_position.unwrap()[2]).abs()).round() as i32;
            if dist > max_dist { max_dist = dist; }
            
        }
    }
    println!("Part 2: Maximal distance between any beacons is {}", max_dist);
}

fn unify_readings(scanners: &Vec<Scanner>, readings: &Readings) -> Vec<(i32, i32, i32)> {
    let mut result = Vec::new();

    for scanner in scanners {
        for reading in &readings[scanner.num] {
            let beacon = scanner.rotation.unwrap() * reading + scanner.abs_position.unwrap();
            let beacon_int = (beacon[0].round() as i32, beacon[1].round() as i32, beacon[2].round() as i32);

            if !result.contains(&beacon_int) {
                result.push(beacon_int);
            }
        }
    }
    result
}

type Rotation = Rotation3<f64>;
type Readings = Vec::<Vec<Coord>>;
type Coord = Vector3<f64>;

#[derive(Debug, Clone)]
struct Scanner {
    num: usize,
    abs_position: Option<Coord>,
    rotation: Option<Rotation>
}

fn find_scanners (readings: &Readings) -> Vec<Scanner> {
    let mut stack = Vec::new();
    let scanner0 = Scanner { num: 0, abs_position: Some (Coord::new(0.0,0.0,0.0)), rotation: Some(Rotation::identity())  };   // Some(Matrix3::identity())

    let mut scanners:Vec<Scanner> = Vec::new();
    for i in 0..readings.len() {
        scanners.push(Scanner { num: i, abs_position: None, rotation: Some(Rotation3::identity()) });
    }
    scanners[0] = scanner0;

    stack.push(scanners[0].clone());
    
    while let Some(scanner_a) = stack.pop() {
        let mut found_scanners = Vec::new();
        for scanner_b in scanners.iter().clone().filter(|&x| x.abs_position.is_none()) {
            println!("Trying to match scanner {} with scanner {} ", scanner_a.num, scanner_b.num);
            let scanner_match = 
                try_find_match(&readings[scanner_a.num], &readings[scanner_b.num], scanner_a.rotation.unwrap());
            if let Some( ( rel_position, rotation)) = scanner_match {
                found_scanners.push( Scanner { 
                    abs_position: Some(scanner_a.abs_position.unwrap() + rel_position), 
                    rotation: Some(rotation), ..*scanner_b
                });
            }
        }
        // Copy found scanners from temporary vec to satisfy the borrow checker
        for found in &found_scanners {
            scanners[found.num] = found.clone();
            stack.push(found.clone());
        }
    }

    scanners
}

fn try_find_match(readings_a: &Vec<Coord>, readings_b: &Vec<Coord>, rotation_a: Rotation) -> Option<(Vector3<f64>, Rotation)> {
    // println!("-- Matching readings starting with {} with reading starting with {}", readings_a[0], readings_b[0]);
    for rotation in all_rotations() {
        // println!("Rotation: {}", rotation);
        let mut distances = HashMap::new();
        for beacon_a in readings_a {
            let rotated_point_a = rotation_a * beacon_a; 
            // println!("beacon_a {}", beacon_a);
            for beacon_b in readings_b {
                let rotated_point_b = rotation * beacon_b;
                let distance = rotated_point_b - rotated_point_a; 
                let key = (distance[0].round() as i32, distance[1].round() as i32, distance[2].round() as i32); 
                let e = distances.entry(key).or_insert(0);
                *e += 1;
            }
            // println!("Distances {:?}", distances);
        }
        let common_distances = distances.iter().filter(|&(_,c)| *c >=12 ).collect::<Vec<_>>();
        if common_distances.len() == 0 {
            // println!("No match found on this rotation");
        } else if common_distances.len() == 1 {
            let distance = common_distances[0].0;
            println!("Found one match at distance {:?}", distance);
            return Some(
                (
                    -Vector3::new(distance.0 as f64, distance.1 as f64, distance.2 as f64), 
                    rotation
                )
            );
        } else {
            println!("Found more than one possible match:");
            for (d,c) in common_distances {
                println!("{:?} = {}", d, c);
            }
            panic!();
        }
    }
    None
}

fn all_rotations() -> Vec<Rotation> {
    let  mut rotations = Vec::new();
    let degree_90 = std::f64::consts::FRAC_PI_2;
    for angle in [ 0.0, degree_90, degree_90*2.0, degree_90*3.0 ] {
        rotations.push(Rotation3::from_euler_angles(angle, 0.0,0.0));
        rotations.push(Rotation3::from_euler_angles(angle, degree_90*2.0,0.0));
        rotations.push(Rotation3::from_euler_angles(angle, degree_90,0.0));
        rotations.push(Rotation3::from_euler_angles(angle, -degree_90,0.0));
        rotations.push(Rotation3::from_euler_angles(angle, 0.0, degree_90));
        rotations.push(Rotation3::from_euler_angles(angle, 0.0, -degree_90));
    }
    // println!("Rotations: {:?}", rotations);
    rotations
}