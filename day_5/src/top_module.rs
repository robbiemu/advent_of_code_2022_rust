use lazy_static::lazy_static;

use regex::Regex;
use std::iter::empty;
use super::problem_solver::ProblemSolver;


struct Move {
  count: usize,
  from: usize,
  to: usize
}

lazy_static! {
  static ref RE: Regex = 
  Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

pub struct PSInput {
  stacks: Vec<Vec<char>>,
  moves: Vec<Move>
}

pub struct PSSolution {
  top_of_each_stack: Vec<char>
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;
  
  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut moves: Vec<Move> = vec![];
    let mut is_record_a_move = false;
    for record in lines {
      if is_record_a_move {
        if let Some(mv) = move_from_record(record) {
          moves.push(mv);
        }
      } else {
        let (crates, is_end_of_crates) = 
        crates_from_record(record.clone());
        if is_end_of_crates {
          is_record_a_move = true;
          
          continue;
        }
        if stacks.len() == 0 {
          for _ in 0..crates.len() {
            stacks.push(vec![] );
          }
        }
        crates.iter().enumerate().for_each(|(i, c)| {
          if let Some(&first_char) = c.first() {
            stacks
            .get_mut(i)
            .map(|stack| stack.insert(0, first_char));
          }
        });
      }
    }
    
    Self::Input { stacks, moves }
  }
  
  fn solve(input: Self::Input) -> Self::Solution {
    let mut stacks = input.stacks.clone();
    let mut lens: Vec<usize> = input.stacks.iter()
    .map(|v| v.len())
    .collect();
    for mv in input.moves {
      println!("moving {} from {:?} to {:?}", 
        mv.count, stacks[mv.from], stacks[mv.to]);
      let popped = stacks[mv.from]
      .splice(lens[mv.from] - mv.count .., empty())
      .rev()
      .collect::<Vec<_>>();
      stacks[mv.to].extend(popped); 
      lens[mv.from] -= mv.count;
      lens[mv.to] += mv.count;
    }
    
    let top_of_each_stack: Vec<char> = stacks.iter()
    .map(|stack| stack.last())
    .flatten()
    .copied()
    .collect::<Vec<char>>();
    
    Self::Solution { top_of_each_stack }
  }
  
  fn output(solution: Self::Solution) {
    println!("{:?}", 
      solution.top_of_each_stack.iter().collect::<String>())
  }
}

fn move_from_record(record: String) -> Option<Move> {
  if let Some(captures) = RE.captures(&record) {
    let (count, from, to) = (
      captures[1].parse::<usize>().unwrap(),
      captures[2].parse::<usize>().unwrap() - 1,
      captures[3].parse::<usize>().unwrap() - 1,
    );
    
    return Some(Move { count, from, to });
  }
  None
}

fn crates_from_record(record: String) -> (Vec<Vec<char>>, bool) {
  let crates: Vec<String> = record
  .chars()
  .collect::<Vec<char>>()
  .chunks(4)
  .map(|chunk| chunk.iter().collect())
  .collect();
  
  if !crates[0].starts_with('[') {
    return (vec![], true);
  }
  
  let rotated: Vec<Vec<char>> = crates
  .into_iter()
  .map(|c| {
    let trimmed = c.trim_start_matches('[')
    .trim_end_matches("] ")
    .trim();
    trimmed.chars().collect()
  })
  .collect();
  
  (rotated, false)
}
