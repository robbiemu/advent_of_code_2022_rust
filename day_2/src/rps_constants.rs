use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Moves {
  Rock = 1,
  Paper = 2,
  Scissors = 3,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Outcomes {
  Loss = 0,
  Draw = 3,
  Win = 6,
}


lazy_static! {
  pub static ref OPPONENT_MOVES: HashMap<&'static str, Moves> = {
    let mut map = HashMap::with_capacity(3);
    map.insert("A", Moves::Rock);
    map.insert("B", Moves::Paper);
    map.insert("C", Moves::Scissors);
    map
  };
  
  pub static ref SUBJECT_MOVES: HashMap<&'static str, Moves> = {
    let mut map = HashMap::with_capacity(3);
    map.insert("X", Moves::Rock);
    map.insert("Y", Moves::Paper);
    map.insert("Z", Moves::Scissors);
    map
  };
  
  pub static ref SCORES: HashMap<(Moves, Moves), i32> = {
    let mut map = HashMap::with_capacity(9);
    map.insert((Moves::Rock, Moves::Rock), 
      Outcomes::Draw as i32 + Moves::Rock as i32);
    map.insert((Moves::Paper, Moves::Paper), 
      Outcomes::Draw as i32 + Moves::Paper as i32);
    map.insert((Moves::Scissors, Moves::Scissors), 
      Outcomes::Draw as i32 + Moves::Scissors as i32);
    map.insert((Moves::Rock, Moves::Paper), 
      Outcomes::Win as i32 + Moves::Paper as i32);
    map.insert((Moves::Rock, Moves::Scissors), 
      Outcomes::Loss as i32 + Moves::Scissors as i32);
    map.insert((Moves::Paper, Moves::Scissors), 
      Outcomes::Win as i32 + Moves::Scissors as i32);
    map.insert((Moves::Paper, Moves::Rock), 
      Outcomes::Loss as i32 + Moves::Rock as i32);
    map.insert((Moves::Scissors, Moves::Rock), 
      Outcomes::Win as i32 + Moves::Rock as i32);
    map.insert((Moves::Scissors, Moves::Paper), 
      Outcomes::Loss as i32 + Moves::Paper as i32);
    map
  };
}
