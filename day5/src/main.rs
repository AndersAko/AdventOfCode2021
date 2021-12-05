use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let filecontents = fs::read_to_string("input_test.txt").expect("Something went wrong?");
    let mut lines = (&filecontents).split_terminator('\n');

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<x0>\d+),(?P<y0>\d+) -> (?P<x1>\d+),(?P<y1>\d+)").unwrap();
    }

    for line in lines {
        let x = RE.captures(line).and_then(|cap| {
            cap.name("login").map(|login| login.as_str())
        });
    }
}
