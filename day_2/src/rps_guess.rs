use std::fs::File;
use std::io::{Lines, BufReader};

use crate::rps_constants::{Moves, OPPONENT_MOVES, SUBJECT_MOVES, SCORES};


pub struct RPSInput {
  move_pairs: Vec<(Moves, Moves)>
}
pub struct RPSSolution {
  scores: Vec<i32>
}

pub fn initialize(lines: Lines<BufReader<File>>) -> RPSInput {
  let mut move_pairs: Vec<(Moves, Moves)> = Vec::new(); 
  for line in lines {
    if let Ok(record) = line {
      if record.is_empty() {
        dbg!("empty record!");
        continue;
      }
      
      let tokens: Vec<&str> = record.split_whitespace().collect(); 
      if tokens.len() == 2 {
        let (opponent, subject) = 
          (tokens[0].to_string(), tokens[1].to_string());
        
        if let Some(opponent_move) = OPPONENT_MOVES.get(&*opponent) {
            if let Some(subject_move) = SUBJECT_MOVES.get(&*subject) {
                move_pairs.push((opponent_move.clone(), subject_move.clone()));
            } else {
                dbg!("Subject move not found for record: {}", record);
            }
        } else {
            dbg!("Opponent move not found for record: {}", record);
        }
      } else {
        dbg!("irregular record!: {}", record);
      }
    }
  }
  RPSInput { move_pairs }
}

pub fn solve(input: RPSInput) -> RPSSolution {
  let scores: Vec<i32> = input.move_pairs.iter()
    .map(|(opponent, subject)| {
      println!("{:?} -> {}", (opponent, subject), 
        SCORES.get(&(*opponent, *subject)).unwrap().clone());
      SCORES.get(&(*opponent, *subject)).unwrap().clone()
    })
    .collect();
  
  RPSSolution { scores }
}

pub fn output(solution: RPSSolution) {
  println!("{}", solution.scores.iter().sum::<i32>());
}
