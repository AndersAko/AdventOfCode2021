use std::collections::HashMap;

fn main() {
    // let mut player_pos = [4,8]; 
    let mut player_pos = [7,4]; 

    let mut player_score = [0,0]; 
    
    let mut die = Die::new();
    let mut player_turn = 0;

    while player_score[0] < 1000 && player_score[1] < 1000 {
        let die_roll = die.roll() + die.roll() + die.roll();
        player_pos[player_turn] = player_pos[player_turn] + die_roll;
        while player_pos[player_turn] > 10 { player_pos[player_turn] -= 10 }

        player_score[player_turn] += player_pos[player_turn];
        println!("Player {} rolls {} landing on {} scoring {}", player_turn+1, die_roll, player_pos[player_turn], player_score[player_turn]);
        player_turn = (player_turn + 1)%2; 
    }
    println!("Part 1: Losing player has {} points and die is rolled {} times => {}", player_score[player_turn], die.count, player_score[player_turn]*die.count);

    let mut player_score = [Vecx::new(),Vec::new()]; 
    let mut player_turn = 0;
    let die_rolls = Die::dirac_probabilities();

    while player_score[0] < 21 && player_score[1] < 21 {

        let die_roll = die.roll() + die.roll() + die.roll();
        player_pos[player_turn] = player_pos[player_turn] + die_roll;
        while player_pos[player_turn] > 10 { player_pos[player_turn] -= 10 }

        player_score[player_turn] += player_pos[player_turn];
        println!("Player {} rolls {} landing on {} scoring {}", player_turn+1, die_roll, player_pos[player_turn], player_score[player_turn]);
        player_turn = (player_turn + 1)%2; 
    }
}

struct Die { 
    roll: i32,
    count : i32
}

impl Die {
    fn roll(&mut self) -> i32 {
        self.roll = (self.roll + 1)%100;
        self.count += 1;
        self.roll + 1
    }

    fn new() -> Self{
        Die{ roll: -1, count: 0}
    }

    fn dirac_probabilities() -> HashMap::<i32,i32> {
        let mut sums = HashMap::new();
        
        for roll1 in 1..=3 {
            for roll2 in 1..=3 {
                for roll3 in 1..=3 {
                    let e = sums.entry(roll1+roll1+roll3).or_insert(0);
                    *e += 1;
                }
            }
        }
        sums
    }
}
