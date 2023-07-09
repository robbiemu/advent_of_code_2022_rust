use super::problem_solver::ProblemSolver;
use crate::common::simulate;


const SHAPES_COUNT: u64 = 1_000_000_000_000;

pub struct PSInput {
  air_flow: Vec<char>,
}

pub struct PSSolution {
  height: u64,
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(mut lines: impl Iterator<Item = String>) -> Self::Input {
    let air_flow = lines.next().unwrap().chars().collect();

    Self::Input { air_flow }
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let (_chamber, height) = simulate(&input.air_flow, SHAPES_COUNT, true);

    Self::Solution { height }
  }

  fn output(solution: Self::Solution) {
    println!("height: {}", solution.height);
  }
}
