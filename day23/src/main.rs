use std::fmt::Display;
use std::fs;
use std::collections::HashSet;
use std::collections::BinaryHeap;
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
        let filecontents = fs::read_to_string("input_test.txt").expect("Something went wrong?");
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
            if !self.correct_room(i) { return false; }
        }
        true
    }

    fn correct_room(&self, amph: i8) -> bool {
        return State::correct_room_with_position(amph, self.positions[amph as usize])
    }

    fn correct_room_with_position(amph: i8, position: Position) -> bool {
        position/2 == 10 + (amph/2) * 5
    }
    
    fn room(position: &Position) -> bool {
        *position > 10
    }

    fn occupied(&self, pos: Position) -> bool {
        self.positions.iter().any(|&x| x == pos)
    }

    fn moves(&self) -> Vec<State> {
        static LINKS: [(Position,Position,bool);18] = [
            (0,1,true), (1,2,false), (2,20,true), (20,21,true),
            (2,3,true), (3,4,false), (4,30,true), (30,31,true),
            (4,5,true), (5,6,false), (6,40,true), (40,41,true),
            (6,7,true), (7,8,false), (8,50,true), (50,51,true),
            (8,9,true), (9,10,true) ];

        let mut moves = Vec::new();
        println!("Moves {}", self);

        for amph in 0..8 {
            let pos = self.positions[amph];
            let starting_in_room = State::room(&pos);
            println!("-- amph {} at {}({})", amph, pos, starting_in_room);

            let mut added = Vec::new();
            let mut stack = Vec::new();
            stack.push(pos);
            while let Some(pos) = stack.pop() {
                let possible_pos = LINKS.iter().filter(|&&l| l.0 == pos || l.1 == pos);
                for (to, from, stop) in possible_pos {
                    let next = if to == &pos {
                        *from
                    } else {
                        assert_eq!(*from, pos);
                        *to
                    };
                    if self.occupied(next) || added.contains(&next) { break; }
                    added.push(next);
                    // if stop && !incorrect_room(next) && ! ( corridor && started in corridor ) { push (next) }
                    if *stop && ( !State::room(&next) || State::correct_room_with_position(amph as i8, next)) && 
                        ( State::room(&next) || starting_in_room ) {
                            let mut positions = self.positions.clone();
                            positions[amph as usize] = next;
                            moves.push(State {
                                positions, energy: self.energy + 10usize.pow( (amph/2) as u32)
                            });
                        }
                    stack.push(next);
                }
            }
        }
        moves
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.energy.cmp(&self.energy).then_with(|| self.positions.cmp(&other.positions))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "\n#############\n#");
        for i in 0..11 {
            if let Some(amph) = self.positions.iter().position(|&x| x == i ) {
                write!(f, "{}", AMPHS[amph]);
            } else {
                write!(f, ".");
            }
        }
        write!(f, "#\n###");
        for i in 0..4 {
            if let Some(amph) = self.positions.iter().position(|&x| x == 20 + 10 * i ) {
                write!(f, "{}#", AMPHS[amph]);
            } else {
                write!(f, ".#");
            }
        }
        write!(f, "##\n  #");
        for i in 0..4 {
            if let Some(amph) = self.positions.iter().position(|&x| x == 21 + 10 * i ) {
                write!(f, "{}#", AMPHS[amph]);
            } else {
                write!(f, ".#");
            }
        }
        writeln!(f, "\n  #########")
     }
}

fn main() {
    let state = State::from_input();
    println!("{:?}", state);
    println!("State: {}", state);
    solve(state);
}

fn solve (state: State) {
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();

    queue.push(state);
    while let Some(state) = queue.pop() {
        if state.done() { return; }
        visited.insert(state.positions);

        // Possible moves
        for amph in 0..8 {
            let pos = state.positions[amph]; // Position of this amphipod
            if pos > 11 && pos % 10 == 1 {  // In lower part of room
                if state.correct_room(amph as i8) { continue; }
                // if 
            }
        }
    }


}