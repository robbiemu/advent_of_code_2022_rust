use std::collections::HashMap;

use super::problem_solver::ProblemSolver;
use crate::common::{factory_system, prelude::*, score_system};


const TIME_STEPS: usize = 32;

pub struct PSInput {
  systems: Vec<System>,
}

pub struct PSSolution {
  scores: HashMap<usize, usize>,
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let mut systems: Vec<System> =
      lines.map(|l| factory_system(l, TIME_STEPS)).collect();
    systems.truncate(3);

    Self::Input { systems }
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let scores = input.systems.iter().map(score_system).collect();

    Self::Solution { scores }
  }

  fn output(solution: Self::Solution) {
    let mut order = solution.scores.keys().collect::<Vec<_>>();
    order.sort();
    let res = order
      .iter()
      .map(|key| {
        let value = solution.scores.get(key.to_owned()).unwrap();
        format!("{}:{}", key, value)
      })
      .collect::<Vec<_>>()
      .join("\n");

    println!(
      "geodes product: {}",
      solution.scores.values().product::<usize>(),
    );
    println!("{res}");
  }
}
