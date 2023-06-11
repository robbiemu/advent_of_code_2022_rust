use super::common::*;
use super::problem_solver_service::ProblemSolver;


pub struct PSInput {
  moves: Vec<Move>,
}

pub struct PSSolution {
  number_of_positions: usize,
}

pub struct CloudRunSolver;

impl ProblemSolver for CloudRunSolver {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let moves: Vec<Move> = lines.map(factory_move).collect();

    Self::Input { moves }
  }


  fn solve(input: Self::Input) -> Self::Solution {
    let number_of_positions = get_tail_positions(input.moves, 10).len();

    Self::Solution { number_of_positions }
  }

  fn output(solution: Self::Solution) -> String {
    format!("Number of positions {}", solution.number_of_positions)
  }
}
