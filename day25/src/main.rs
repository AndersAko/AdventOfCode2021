use std::fs;


fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Failed");
    let mut sea_floor: SeaFloor = file_contents.split_terminator("\n")
        .map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let width = sea_floor[0].len();
    let height = sea_floor.len();
    println!("Read {} lines of {} each", height, width);
    sea_floor.print();
    let mut count = 0;
    loop {
        let (moved, next) =  sea_floor.next_step();
        count += 1;
        if !moved { break;}
        sea_floor = next;
    }
    println!("After {} moves, they come to rest at:", count);
    sea_floor.print();
}

type SeaFloor = Vec<Vec<char>>;
trait SeaFloorTrait {
    fn print(&self);
    fn next_step(&self) -> (bool, Self);
}
impl SeaFloorTrait for SeaFloor {
    fn print(&self) {
        for l in self {
            println!("{}", l.iter().collect::<String>());
        }
        println!();
    }
    fn next_step(&self) -> (bool, Self) {
        let width = self[0].len();
        let height = self.len();
        let mut moved = false;
        let mut next_state = self.clone();
        for r in 0..height {
            for c in 0..width {
                if  self[r][c] == '>' && self[r][(c+1)%width] == '.' {
                    next_state [r][(c+1)%width] = '>';
                    next_state [r][c] = '.';
                    moved = true;
                }
            }
        }
        let mut next_state_d = next_state.clone();
        for r in 0..height {
            for c in 0..width {
                if self[r][c] == 'v' && next_state[(r+1)%height][c] == '.' {
                    next_state_d [(r+1)%height][c] = 'v';
                    next_state_d [r][c] = '.';
                    moved = true;
                }
            }
        }
        (moved, next_state_d)
    }
}

