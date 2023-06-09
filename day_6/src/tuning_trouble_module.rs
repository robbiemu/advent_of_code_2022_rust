use std::collections::HashSet;

use super::problem_solver_shuttle_axum::ProblemSolver;


pub struct PSInput {
  stream: String
}

pub struct PSSolution {
  position: i16
}

pub struct PSContext {
  data: usize
}

impl PSContext {
  pub fn from(data: usize) -> PSContext {
      PSContext { data } 
  }
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;
  type Context = PSContext;
  
  fn initialize(
    lines: impl Iterator<Item = String>, 
    _: Option<PSContext>
  ) -> Self::Input {
    let mut lines = lines.peekable();
    match lines.next() {
      Some(stream) => {
        if lines.next().is_none() {
          Self::Input { stream }
        } else {
          panic!("Expected only one line, but found multiple lines");
        }
      }
      None => panic!("No input lines found"),
    }
  }
  
  fn solve(input: Self::Input, context: Option<PSContext>) 
  -> Self::Solution 
  {
    let context = context.expect("Context not provided");
    let window_size = context.data;

    let mut position: usize = input.stream.chars()
    .collect::<Vec<char>>()
    .windows(window_size)
    .position(|chars| has_n_unique_chars(window_size, chars))
    .unwrap_or(usize::MAX);
    if position < usize::MAX {
      position += window_size;
    }  
    let position: i16 = position.try_into().unwrap_or(-1);
    
    Self::Solution{ position }
  }
  
  fn output(solution: Self::Solution, context: Option<PSContext>) 
  -> String 
  {
    let context = context.expect("Context not provided");
    let window_size = context.data;

    match solution.position {
      -1 => format!(
        "no window of {} unique characrters found in input!", 
        window_size),
      n => format!(
        "window of {} unique characrters found in position {}", 
        window_size, n)
    }
  }
}
  
  fn has_n_unique_chars(n: usize, chars: &[char]) -> bool {
    let mut unique_chars = HashSet::new();
    
    for &ch in chars {
      unique_chars.insert(ch);
      
      if unique_chars.len() == n {
        return true;
      }
    }
    
    false
  }
  