use std::fmt::Display;
use std::fs;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::cmp::Ordering;

static AMPHS: [&str;8] = ["A", "A", "B", "B", "C", "C", "D", "D" ];
type Position = i8; // positions: 0-10 = corridor, 20-21 = A cell , 30-31 = B cell etc

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct State {
    positions: [Position;8],   // Possitions in order AABBCCDD
    energy: usize
}

impl State {
    fn new() -> Self {
        State {
            positions: [-1;8],
            energy: 0
        }
    }
    fn from_input() -> Self {
        let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");
        let lines: Vec<&str> = filecontents.split_terminator("\n").collect();
        let mut state = State::new();
        for j in 0..2 {
            for i in 0..4 {
                let cell = &lines[2+j][3+i*2..3+i*2+1];
                if let Some(amph) = AMPHS.iter().position(|&x| cell == x) {
                    let mut amph = amph;
                    if state.positions[amph] != -1 { amph += 1 }
                    state.positions[amph] = (20 + j + i * 10) as Position;
                }
            }
        }
        state
    }
    fn done(&self) -> bool {
        for i in 0..8 {
            if !self.in_correct_room(i) { return false; }
        }
        true
    }
    fn estimated_energy(&self) -> usize {
        let mut min_remain = 0; 
        for amph in 0..8 {
            min_remain = min_remain + 10usize.pow( (amph/2) as u32) * 
                if self.in_correct_room(amph) && self.squatter(self.positions[amph]) { 4 }
                else if self.in_room(amph) { (2 + (self.correct_room(amph) - self.positions[amph]).abs()/5) as usize }
                else { ((self.positions[amph]/2 + 1 - self.correct_room(amph)/10).abs()  + 1)  as usize } ;
        }
        self.energy + min_remain
    }
    fn in_correct_room(&self, amph: usize) -> bool {
        return State::correct_room_with_position(amph, self.positions[amph as usize])
    }

    fn correct_room(&self, amph: usize) -> Position {
        (20 + (amph/2) * 10) as Position
    }

    fn correct_room_with_position(amph: usize, position: Position) -> bool {
        position/2 == (10 + (amph/2) * 5) as i8
    }
    fn in_room(&self, amph: usize) -> bool {
        State::room(&self.positions[amph])
    }
    fn room(position: &Position) -> bool {
        *position > 10
    }

    fn occupied(&self, pos: Position) -> bool {
        self.positions.iter().any(|&x| x == pos)
    }
    fn squatter(&self, room: Position) -> bool {
        if !State::room(&room) { return false };
        for amph in 0..8 {
            if self.positions[amph]/10 == room/10 && !State::correct_room_with_position(amph, room) {
                return true;
            }
        }
        false
    }

    fn moves(&self) -> Vec<State> {
        static LINKS: [(Position,Position,bool);36] = [
            (0,1,true), (1,2,false), (2,20,true), (20,21,true),
            (2,3,true), (3,4,false), (4,30,true), (30,31,true),
            (4,5,true), (5,6,false), (6,40,true), (40,41,true),
            (6,7,true), (7,8,false), (8,50,true), (50,51,true),
            (8,9,true), (9,10,true),
            (1,0,true), (2,1,true), (20,2,false), (21,20,true),
            (3,2,true), (4,3,true), (30,4,false), (31,30,true),
            (5,4,true), (6,5,true), (40,6,false), (41,40,true),
            (7,6,true), (8,7,true), (50,8,false), (51,50,true),
            (9,8,true), (10,9,true) ];

        let mut moves = Vec::new();
        // println!("Moves {}", self);

        for amph in 0..8 {
            let pos = self.positions[amph];
            let starting_in_room = State::room(&pos);
            // println!("-- amph {} at {}({})", amph, pos, starting_in_room);
            if self.in_correct_room(amph) && !self.squatter(self.positions[amph]) { continue; }
            let mut added = Vec::new();
            let mut stack = VecDeque::new();
            stack.push_back((pos, 1));
            while let Some((pos, no_moves)) = stack.pop_front() {
                let possible_pos = LINKS.iter().filter(|&&l| l.0 == pos);
                for (from, to, stop) in possible_pos {
                    let next = *to;
                    // print!(" {} ", next);
                    if self.occupied(next) || added.contains(&next) { continue; }
                    added.push(next);
                    // if stop && !incorrect_room(next) && ! ( corridor && started in corridor ) { push (next) }
                    if *stop && 
                        ( !State::room(&next) ||
                            (State::correct_room_with_position(amph, next) && !self.squatter(next))) && 
                        ( State::room(&next) || starting_in_room ) {
                            let mut positions = self.positions.clone();
                            positions[amph as usize] = next;
                            moves.push(State {
                                positions, energy: self.energy + 10usize.pow( (amph/2) as u32) * no_moves
                            });
                            // println!("   possible move to {}", next);
                        }
                    stack.push_back((next, no_moves + 1));
                }
            }
            // println!();
        }
        // println!("=> {:?}", moves);
        moves
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimated_energy().cmp(&self.estimated_energy()).then_with(|| self.positions.cmp(&other.positions))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "\n#############\n#").expect("Failed");
        for i in 0..11 {
            if let Some(amph) = self.positions.iter().position(|&x| x == i ) {
                write!(f, "{}", AMPHS[amph]).expect("Failed");
            } else {
                write!(f, ".").expect("Failed");
            }
        }
        write!(f, "#\n###").expect("Failed");
        for i in 0..4 {
            if let Some(amph) = self.positions.iter().position(|&x| x == 20 + 10 * i ) {
                write!(f, "{}#", AMPHS[amph]).expect("Failed");
            } else {
                write!(f, ".#").expect("Failed");
            }
        }
        write!(f, "##\n  #").expect("Failed");
        for i in 0..4 {
            if let Some(amph) = self.positions.iter().position(|&x| x == 21 + 10 * i ) {
                write!(f, "{}#", AMPHS[amph]).expect("Failed");
            } else {
                write!(f, ".#").expect("Failed");
            }
        }
        writeln!(f, "\n  #########      {} ({})", self.energy, self.estimated_energy())
     }
}

fn main() {
    let state = State::from_input();
    println!("{:?}", state);
    println!("State: {}", state);
    let solved_state = solve(state);
    println!("Found a solution with {} energy. {}", solved_state.energy, solved_state);
}

fn solve (state: State) -> State {
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();

    queue.push(state);
    while let Some(state) = queue.pop() {
        if state.done() { return state; }
        visited.insert(state.positions);
        if visited.len() < 10 || visited.len() % 1000 == 0 { 
            println!("{}", state);
        }
        // Possible moves
        for next_state in state.moves() {
            if !visited.contains(&next_state.positions) {
                queue.push(next_state);
            }
        }
    }
    panic!();
}