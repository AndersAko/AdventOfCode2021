use std::fs;

struct Mapping<'a> {
    from: &'a str,
    to: &'a str
}

fn main() {
    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");

    let lines: Vec<&str> = filecontents.split_terminator("\n").collect();

    let template = lines[0];

    let mapping = 
}
