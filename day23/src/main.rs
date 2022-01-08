use std::fmt::Display;
use std::fs;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::cmp::Ordering;


static AMPHS: [&str;16] = ["A", "A", "A", "A", "B", "B", "B", "B",
                          "C", "C", "C", "C", "D", "D", "D", "D" ];


type Position = i8; // positions: 0-10 = corridor, 20-21 = A cell , 30-31 = B cell etc
const DEBUG:bool = false;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct State {
    positions: [Position;16],   // Positions in order AA..BB..CC..DD..
    energy: usize,
    no_amphs: usize
}

impl State {
    fn new(no_amphs: usize) -> Self {
        State {
            positions: [-1;16],
            energy: 0,
            no_amphs
        }
    }
    fn from_input(no_amphs: usize) -> Self {
        let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");
        let lines: Vec<&str> = filecontents.split_terminator("\n").collect();
        let mut state = State::new(no_amphs);
        let offset = if no_amphs == 8 { 1 } else { 3 };
        for j in 0..2 {
            for i in 0..4 {
                let cell = &lines[2+j][3+i*2..3+i*2+1];
                if let Some(amph) = AMPHS.iter().position(|&x| cell == x) {
                    let mut amph = amph;
                    if no_amphs == 8 { amph /=2 }
                    if state.positions[amph] != -1 { amph += 1 }
                    state.positions[amph] = (20 + j*offset + i * 10) as Position;
                }
            }
        }
        state
    }

    fn done(&self) -> bool {
        for i in 0..self.no_amphs {
            if !self.in_correct_room(i) { return false; }
        }
        true
    }
    fn estimated_energy(&self) -> usize {
        let mut min_remain = 0; 
        for amph in 0..self.no_amphs {
            min_remain = min_remain + 10usize.pow( (amph*4/self.no_amphs) as u32) * 
                if self.in_correct_room(amph) && !self.squatter(self.positions[amph]) { 0 }
                else if !self.in_room(amph) { 2 } 
                else { 4 } ;
        }
        self.energy + min_remain
    }

    fn in_correct_room(&self, amph: usize) -> bool {
        return self.correct_room_with_position(amph, self.positions[amph as usize])
    }

    fn correct_room(&self, amph: usize) -> Position {
        (20 + (amph*4/self.no_amphs) * 10) as Position
    }

    fn correct_room_with_position(&self, amph: usize, position: Position) -> bool {
        (position as usize) / 10 == 2 + (amph*4/self.no_amphs) 
    }
    fn in_room(&self, amph: usize) -> bool {
        State::is_room(&self.positions[amph])
    }
    fn is_room(position: &Position) -> bool {
        *position > 10
    }
    fn room_pos(&self, amph: usize) -> Position {
        (amph * 4 / self.no_amphs * 2 + 2) as Position
    }

    fn corridor_amphs(&self) -> usize {
        self.positions.iter().filter(|&p| State::is_room(&p)).count()
    }
    fn occupied(&self, pos: Position) -> bool {
        self.positions.iter().any(|&x| x == pos)
    }
    fn amph_at(&self, pos: Position) -> Option<usize> {
        self.positions.iter().position(|&x| x == pos)
    }
    // Count free spots from start, in specified direction, ignoring amphs that have a room_pos > x if going right
    fn free_space_from(&self, start:Position, right: bool, excluded: Position) -> usize {
        let mut i = start;
        let mut count = 0;
        while i > 0 && i <11 {
            i += if right { 1 } else { -1 };
            if let Some(a) = self.amph_at(i) { 
                if  right && (self.room_pos(a) < excluded || self.room_pos(a) > start)  ||
                    !right && (self.room_pos(a) > excluded || self.room_pos(a) < start)  {
                    break;
                }
             }
            if i != 2 && i != 4 && i != 6 && i != 8 { count += 1 }
        }
        count
    }

    fn squatter(&self, room: Position) -> bool {
        if !State::is_room(&room) { return false };
        for amph in 0..self.no_amphs {
            if self.positions[amph]/10 == room/10 && !self.correct_room_with_position(amph, room) {
                return true;
            }
        }
        false
    }

    fn moves(&self) -> Vec<State> {
        static LINKS: [(Position,Position,bool);52] = [
            (0,1,true), (1,2,false), (2,20,true), (20,21,true),
            (2,3,true), (3,4,false), (4,30,true), (30,31,true),
            (4,5,true), (5,6,false), (6,40,true), (40,41,true),
            (6,7,true), (7,8,false), (8,50,true), (50,51,true),
            (8,9,true), (9,10,true),
            (1,0,true), (2,1,true), (20,2,false), (21,20,true),
            (3,2,false), (4,3,true), (30,4,false), (31,30,true),
            (5,4,false), (6,5,true), (40,6,false), (41,40,true),
            (7,6,false), (8,7,true), (50,8,false), (51,50,true),
            (9,8,false), (10,9,true),
            (21,22,true),(22,23,true),(22,21,true),(23,22,true),
            (31,32,true),(32,33,true),(32,31,true),(33,32,true),
            (41,42,true),(42,43,true),(42,41,true),(43,42,true),
            (51,52,true),(52,53,true),(52,51,true),(53,52,true),
        ];

        let mut moves = Vec::new();
        if DEBUG { println!("Moves {}", self); }

        for amph in 0..self.no_amphs {
            let pos = self.positions[amph];
            let starting_in_room = State::is_room(&pos);
            if DEBUG { println!("-- amph {} at {}({})", amph, pos, starting_in_room); } 
            if self.in_correct_room(amph) && !self.squatter(self.positions[amph]) { continue; }
            let mut added = vec![pos];
            let mut stack = VecDeque::new();
            stack.push_back((pos, 1));
            while let Some((pos, no_moves)) = stack.pop_front() {
                let possible_pos = LINKS.iter().filter(|&&l| l.0 == pos);
                for (_from, to, stop) in possible_pos {
                    let next = *to;     
                    if self.no_amphs == 8 && next >10 && (next % 10) > 1 { continue; }  // Ignore bottom half of rooms
                    if self.occupied(next) || added.contains(&next) { continue; } 
                    if DEBUG { print!(" {} ", next); }
                    
                    added.push(next);
                    if *stop {
                        if self.correct_room_with_position(amph, next) {
                            if DEBUG { println!("Correct room {} with squatter {} and potential {} ", self.correct_room_with_position(amph, next), 
                                self.squatter(next), ( next % 10 == 1 || self.occupied(next + 1))); }
                            let bottom_room = if self.no_amphs == 8 { 1 } else { 3 };
                            if !self.squatter(next) && ( next % 10 == bottom_room || self.occupied(next + 1)) {
                                let mut positions = self.positions.clone();
                                positions[amph as usize] = next;
                                if DEBUG { println!("   only move to {}", next); }
                                return vec![ State {
                                    positions, energy: self.energy + 10usize.pow( (amph*4/self.no_amphs) as u32) * no_moves, no_amphs: self.no_amphs
                                }];
                            }
                        }
                        if !State::is_room(&next) && starting_in_room {
                                let mut positions = self.positions.clone();
                                positions[amph as usize] = next;
                                let new_state = State {
                                    positions, energy: self.energy + 10usize.pow( (amph*4/self.no_amphs) as u32) * no_moves, no_amphs: self.no_amphs
                                };
                                if DEBUG { println!("   possible move to {}", next); } 

                                /* Moves 
                                    #############
                                    #...C.....D.#
                                    ###B#.#B#.###
                                      #D#C#B#A#
                                      #D#B#A#C#
                                      #A#D#C#A#
                                      ######### 

                                    (Anta C sista som flyttade och står till vänster om sitt rum )
                                    => Omöjlig om Room(C) innehåller de som ska till vänster om C är fler än lediga platser till höger  (Motsvarande åt höger)

                                    räkna inte de som har ett hem mellan next och Room(C)
                                */
                                let squatters = self.positions.iter().enumerate()
                                    .filter(|(a, &p)| (p/10 == (amph*4/self.no_amphs) as i8 + 2) && a*4/self.no_amphs != amph*4/self.no_amphs )
                                    .map(|(i,_)| i );
                                if next > self.room_pos(amph) &&          // 'next' är till höger om sitt rum
                                    squatters.clone().filter(|&a| self.room_pos(a) > next).count() > self.free_space_from(self.room_pos(amph), false, next) {
                                        if DEBUG { println!("Detected impossible state due to squatters {:?} destined to the right of {} will not fit in the {} spaces to the left ",
                                                 squatters.collect::<Vec<_>>(), next, self.free_space_from(self.room_pos(amph), false, next)); }
                                } 
                                else if next < self.room_pos(amph) &&     // next är till vänster om sitt rum
                                    squatters.clone().filter(|&a| self.room_pos(a) < next).count() > self.free_space_from(self.room_pos(amph), true, next)  {    
                                        if DEBUG { println!("Detected impossible state due to squatters {:?} destined to the left of {} will not fit in the {} spaces to the right ",
                                                 squatters.collect::<Vec<_>>(), next, self.free_space_from(self.room_pos(amph), true, next)); }
                                } else {
                                    moves.push(new_state);
                                }
                            }
                    }
                    stack.push_back((next, no_moves + 1));
                }
            }
            if DEBUG { println!(); }
        }
        // for m in &moves {
        //     println!("{} ({:?})", m, m);
        // }
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
        let scale = if self.no_amphs == 8 { 2 } else { 1 };
        for i in 0..11 {
            if let Some(amph) = self.positions.iter().position(|&x| x == i ) {
                write!(f, "{}", AMPHS[amph*scale]).expect("Failed");
            } else {
                write!(f, ".").expect("Failed");
            }
        }
        write!(f, "#\n").expect("Failed");
        for j in 0..self.no_amphs/4 {
            if j == 0 { write!(f, "###").expect("Failed"); } else { write!(f, "  #").expect("Failed"); }
            for i in 0..4 {
                if let Some(amph) = self.positions.iter().position(|&x| x as usize == 20 + j + 10 * i ) {
                    write!(f, "{}#", AMPHS[amph*scale]).expect("Failed");
                } else {
                    write!(f, ".#").expect("Failed");
                }
            }
            if j == 0 { write!(f, "##").expect("Failed"); }
            writeln!(f).expect("Failed");
        }
        writeln!(f, "  #########      {} ({})", self.energy, self.estimated_energy())
     }
}

fn main() {
    // let state = State::from_input(8);
    // println!("Starting state: {:?} {}", state, state);
    // let solved_state = solve(state);
    // println!("Part1: Found a solution with {} energy. {}", solved_state.energy, solved_state);

    let mut state = State::from_input(16);
    state.positions[2]= 51; state.positions[3]= 42;
    state.positions[6]= 41; state.positions[7]= 32;
    state.positions[10]= 31; state.positions[11]= 52;
    state.positions[14]= 21; state.positions[15]= 22;

    println!("Part2: Starting state: {}, {:?}", state, state);
    let solved_state = solve(state);
    println!("Part2: Found a solution with {} energy. {}", solved_state.energy, solved_state);


}

fn solve (state: State) -> State {
    let mut visited = HashMap::new();
    let mut queue = BinaryHeap::new();
    let mut solutions = Vec::new();
    let mut lowest_energy_found : Option<usize> = None;

    queue.push(state);
    while let Some(state) = queue.pop() {
        if lowest_energy_found.is_some() && state.estimated_energy() > lowest_energy_found.unwrap() * 8 / 5 {
            break;
        }
        if state.done() { 
            println!("Found a solution {} {}", visited.len(), state);
            lowest_energy_found = Some(state.energy);
            solutions.push(state);
            // return state;
         }
        if visited.len() % 1000 == 0  { 
            println!("{}", state);
        }
        // Possible moves
        for next_state in state.moves() {
            let lowest_cost_here = visited.get(&next_state.positions);
            if lowest_cost_here.is_none() || *lowest_cost_here.unwrap() > next_state.energy {
                visited.insert(next_state.positions, next_state.energy);
                queue.push(next_state);
            }
        }
    }
    println!("Searched {} positions", visited.len());
    return *solutions.iter().min_by(|&x,&y| x.energy.cmp(&y.energy)).unwrap();
}

#[test]
fn test_correct_room8 () {
    let state = State::new(8);
    assert_eq!(state.correct_room_with_position(0, 20), true);
    assert_eq!(state.correct_room_with_position(0, 30), false);
    assert_eq!(state.correct_room_with_position(0, 21), true);
    assert_eq!(state.correct_room_with_position(1, 20), true);
    assert_eq!(state.correct_room_with_position(1, 21), true);
    assert_eq!(state.correct_room_with_position(2, 20), false);
    assert_eq!(state.correct_room_with_position(2, 30), true);
    assert_eq!(state.correct_room_with_position(3, 30), true);
    assert_eq!(state.correct_room_with_position(4, 40), true);
    assert_eq!(state.correct_room_with_position(5, 40), true);
    assert_eq!(state.correct_room_with_position(6, 50), true);
    assert_eq!(state.correct_room_with_position(7, 50), true);
}

#[test]
fn test_correct_room16() {
    let state = State::new(16);
    assert_eq!(state.correct_room_with_position(0, 20), true);
    assert_eq!(state.correct_room_with_position(0, 30), false);
    assert_eq!(state.correct_room_with_position(0, 21), true);
    assert_eq!(state.correct_room_with_position(1, 20), true);
    assert_eq!(state.correct_room_with_position(1, 21), true);
    assert_eq!(state.correct_room_with_position(2, 20), true);
    assert_eq!(state.correct_room_with_position(3, 20), true);
    assert_eq!(state.correct_room_with_position(4, 20), false);
    assert_eq!(state.correct_room_with_position(4, 40), false);
    assert_eq!(state.correct_room_with_position(4, 30), true);
    assert_eq!(state.correct_room_with_position(4, 31), true);
    assert_eq!(state.correct_room_with_position(4, 32), true);
    assert_eq!(state.correct_room_with_position(4, 33), true);
    assert_eq!(state.correct_room_with_position(15, 53), true);
}

#[test]
fn test_moves_16() {
    let move_sequence = [ 
        State { positions: [23, 53, 51, 42, 20, 40, 41, 32, 30, 43, 31, 52, 50, 33, 21, 22], energy: 0, no_amphs: 16 },
        State { positions: [23, 53, 51, 42, 20, 40, 41, 32, 30, 43, 31, 52, 10, 33, 21, 22], energy: 3000, no_amphs: 16 },
        State { positions: [23, 53, 0, 42, 20, 40, 41, 32, 30, 43, 31, 52, 10, 33, 21, 22], energy: 3010, no_amphs: 16 },
        State { positions: [23, 53, 0, 42, 20, 9, 41, 32, 30, 43, 31, 52, 10, 33, 21, 22], energy: 3050, no_amphs: 16 },
        State { positions: [23, 53, 0, 42, 20, 9, 7, 32, 30, 43, 31, 52, 10, 33, 21, 22], energy: 3080, no_amphs: 16 },
        State { positions: [23, 53, 0, 1, 20, 9, 7, 32, 30, 43, 31, 52, 10, 33, 21, 22], energy: 3088, no_amphs: 16 },

        State { positions: [23, 53, 0, 1, 20, 9, 7, 32, 42, 43, 31, 52, 10, 33, 21, 22], energy: 3088, no_amphs: 16 },
        State { positions: [23, 53, 0, 1, 20, 9, 7, 32, 42, 43, 41, 52, 10, 33, 21, 22], energy: 3088, no_amphs: 16 },
        State { positions: [23, 53, 0, 1, 20, 9, 7, 5, 42, 43, 41, 52, 10, 33, 21, 22], energy: 4328, no_amphs: 16 },
        State { positions: [23, 53, 0, 1, 20, 9, 7, 5, 42, 43, 41, 52, 10, 3, 21, 22], energy: 9328, no_amphs: 16 },

        State { positions: [23, 53, 0, 1, 20, 9, 7, 33, 42, 43, 41, 52, 10, 3, 21, 22], energy: 9328, no_amphs: 16 },
        State { positions: [23, 53, 0, 1, 20, 9, 32, 33, 42, 43, 41, 52, 10, 3, 21, 22], energy: 9328, no_amphs: 16 },
        State { positions: [23, 53, 0, 1, 20, 31, 32, 33, 42, 43, 41, 52, 10, 3, 21, 22], energy: 9328, no_amphs: 16 },
        State { positions: [23, 53, 0, 1, 20, 31, 32, 33, 42, 43, 41, 40, 10, 3, 21, 22], energy: 9328, no_amphs: 16 },
        State { positions: [23, 9, 0, 1, 20, 31, 32, 33, 42, 43, 41, 40, 10, 3, 21, 22], energy: 9328, no_amphs: 16 },
        State { positions: [23, 9, 0, 1, 20, 31, 32, 33, 42, 43, 41, 40, 10, 53, 21, 22], energy: 9328, no_amphs: 16 },
        State { positions: [23, 9, 0, 1, 30, 31, 32, 33, 42, 43, 41, 40, 10, 53, 21, 22], energy: 9328, no_amphs: 16 },
        State { positions: [23, 9, 0, 1, 30, 31, 32, 33, 42, 43, 41, 40, 10, 53, 52, 22], energy: 9328, no_amphs: 16 },
        State { positions: [23, 9, 0, 1, 30, 31, 32, 33, 42, 43, 41, 40, 10, 53, 52, 51], energy: 9328, no_amphs: 16 },
        State { positions: [23, 22, 0, 1, 30, 31, 32, 33, 42, 43, 41, 40, 10, 53, 52, 51], energy: 9328, no_amphs: 16 },
        State { positions: [23, 22, 0, 21, 30, 31, 32, 33, 42, 43, 41, 40, 10, 53, 52, 51], energy: 9328, no_amphs: 16 },
        State { positions: [23, 22, 20, 21, 30, 31, 32, 33, 42, 43, 41, 40, 10, 53, 52, 51], energy: 9328, no_amphs: 16 },
        State { positions: [23, 22, 20, 21, 30, 31, 32, 33, 42, 43, 41, 40, 50, 53, 52, 51], energy: 9328, no_amphs: 16 },
    ];
    for seq in 0..move_sequence.len()-1 {
        let state = move_sequence[seq];
        let moves = state.moves();

        assert!(moves.iter().find(|m| m.positions == move_sequence[seq+1].positions ).is_some(), "{}{:?} not found from {}{:?} ", 
            move_sequence[seq+1], move_sequence[seq+1], move_sequence[seq], move_sequence[seq]);
    }
}

#[test]
fn test_room_pos16() {
    let state = State::new(16);

    for i in 0..16 {
        println!("Amph {} roompos {} room {}", i, state.room_pos(i), state.correct_room(i));

        assert_eq!(state.room_pos(i) as usize, 2 + (i / 4)*2, "Amph: {}'s room is not {}", i, 2 + (i / 4)*2);
    }
}
