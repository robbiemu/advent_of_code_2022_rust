use super::problem_solver::ProblemSolver;
use crate::common::{prelude::*, simulate};


const SHAPES_COUNT: u64 = 2022;

pub struct PSInput {
  air_flow: Vec<char>,
}

pub struct PSSolution {
  chamber: Chamber,
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
    let (chamber, _) = simulate(&input.air_flow, SHAPES_COUNT, false);

    Self::Solution { chamber }
  }

  fn output(solution: Self::Solution) {
    //println!("{:?}", solution.chamber);
    let height = solution.chamber.0.len();
    println!("height: {}", height);
  }
}
