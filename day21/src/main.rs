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

    // let player_pos = [4,8]; 
    let player_pos = [7,4]; 
    let mut player_score = [HashMap::new(),HashMap::new()]; 
    player_score[0].insert((0,player_pos[0]), 1u64);   // 1 universe where player 0 has a score of 0 in position player_pos[0]
    player_score[1].insert((0,player_pos[1]), 1u64);
    
    let mut player_turn = 0;
    let die_rolls = Die::dirac_probabilities();
    let mut wins = [0u64,0u64];
    let mut round = 1; 
    while player_score[0].len() > 0 && player_score[1].len() > 0 {
        println!(" Round {}: {} {} ", round, player_score[0].len(), player_score[1].len());
        let mut new_player_score = HashMap::new();
        for (roll, &count_roll) in die_rolls.iter() {
            for ( (score, pos), count_pos) in &player_score[player_turn] {
                let mut new_pos = pos + roll;
                while new_pos >10 { new_pos -= 10 }
                let new_score = score + new_pos;

                if new_score >= 21 {
                    let other_player_options: u64 = player_score[1-player_turn].values().sum();
                    wins[player_turn] += count_pos * count_roll as u64 * other_player_options;
                    println!("Player {} wins in {} universes ( {} positions * {} rolls * {} opponent options )", 
                            player_turn, count_pos * count_roll as u64 * other_player_options, count_pos, count_roll, other_player_options);
                } else {
                    let e = new_player_score.entry((new_score, new_pos)).or_insert(0);
                    *e += count_pos * count_roll;
                }

            }
        }
        player_score[player_turn] = new_player_score;
        player_turn = 1- player_turn;
        round += 1;
    }
    println!("Part 2: Player 1 wins in {} universes. Player 2 wins in {}", wins[0], wins[1]);
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

    fn dirac_probabilities() -> HashMap::<i32,u64> {
        let mut sums = HashMap::new();
        
        for roll1 in 1..=3 {
            for roll2 in 1..=3 {
                for roll3 in 1..=3 {
                    let e = sums.entry(roll1+roll2+roll3).or_insert(0);
                    *e += 1;
                }
            }
        }
        sums
    }
}
