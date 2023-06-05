use std::collections::HashMap;

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
    for record in lines {
      let mut interim: HashMap<char, bool> = HashMap::new();
        let i = record.len() / 2;
        let first = &record[..i].chars().collect::<Vec<_>>();
        let rest = &record[i..];
        for c in first {
            if rest.contains(*c) && !interim.contains_key(&c) {
              interim.insert(*c, true);
            }
        }
        common.extend(interim.keys().cloned());
    }
    PSInput { common }
  }
  
  fn solve(input: Self::Input) -> Self::Solution {
    let score=input.common.iter()
      .map(|c| {
        let mut priority = *c as u32 - 64;
        if priority < 27 {
          priority += 26;
        } else {
          priority -= 32;
        }
        println!("{} -> {}", *c, priority);
        priority
      })
      .sum();
    PSSolution { score }
  }
  
  fn output(solution: Self::Solution) {
    println!("score: {}", solution.score);
  }
}

