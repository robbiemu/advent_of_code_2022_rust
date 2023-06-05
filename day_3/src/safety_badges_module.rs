//use std::collections::HashMap;

use super::problem_solver::ProblemSolver;


pub struct PSInput {
  common: Vec<char>,
}

pub struct PSSolution {
  score: u32
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;
  
  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let mut common: Vec<char> = vec![];
    let lines_vec: Vec<_> = lines.collect();
    let mut line_chunks = lines_vec.chunks(3);
    while let Some(group) = line_chunks.next() {
      for c in group[0].chars() {
        if group[1].contains(c) && group[2].contains(c) {
          common.push(c);
          break;
        }
      }
    }

    PSInput { common }
}
  
  fn solve(input: Self::Input) -> Self::Solution {
    let score=input.common.iter()
      .enumerate()
      .map(|(i, c)| {
        let mut priority = *c as u32 - 64;
        if priority < 27 {
          priority += 26;
        } else {
          priority -= 32;
        }
        println!("[{}] {} -> {}", i, *c, priority);
        priority
      })
      .sum();

    PSSolution { score }
  }
  
  fn output(solution: Self::Solution) {
    println!("score: {}", solution.score);
  }
}

