use std::fs::File;
use std::io::{Lines, BufReader};
use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::rps_constants::{Moves, OPPONENT_MOVES, SCORES, Outcomes};

lazy_static! {
  pub static ref STRATEGY: HashMap<&'static str, Outcomes> = {
    let mut map = HashMap::with_capacity(3);
    map.insert("X", Outcomes::Loss);
    map.insert("Y", Outcomes::Draw);
    map.insert("Z", Outcomes::Win);
    map
  };
}

fn get_move_for_outcome(outcome: Outcomes, opponent_move: Moves) -> Moves {
  match outcome {
    Outcomes::Draw => opponent_move.clone(),
    Outcomes::Loss => get_move_from_i32(1 + ((opponent_move as i32 + 1) % 3)),
    Outcomes::Win => get_move_from_i32(1 + (opponent_move as i32 % 3)),
  }
}

fn get_move_from_i32(i: i32) -> Moves {
  match i {
    1 => Moves::Rock,
    2 => Moves::Paper,
    3 => Moves::Scissors,
    _ => unreachable!()
  }
}

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
        let (opponent, strategy) = 
          (tokens[0].to_string(), tokens[1].to_string());
        
        if let Some(opponent_move) = OPPONENT_MOVES.get(&*opponent) {
            if let Some(strategy) = STRATEGY.get(&*strategy) {
                let subject_move = 
                  get_move_for_outcome(*strategy, *opponent_move);
                println!("{:?} {:?} -> {:?}", strategy, 
                  opponent_move, subject_move);
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
      //println!("{:?} -> {}", (opponent, subject), 
      //  SCORES.get(&(*opponent, *subject)).unwrap().clone());
      SCORES.get(&(*opponent, *subject)).unwrap().clone()
    })
    .collect();
  
  RPSSolution { scores }
}

pub fn output(solution: RPSSolution) {
  println!("{}", solution.scores.iter().sum::<i32>());
}
