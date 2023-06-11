use super::common::*;
use super::problem_solver_service::ProblemSolver;


pub struct PSInput {
  moves: Vec<Move>,
}

pub struct PSSolution {
  number_of_positions: usize,
}

pub struct Part1Solver;

impl ProblemSolver for Part1Solver {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let moves: Vec<Move> = lines.map(factory_move).collect();

    Self::Input { moves }
  }


  fn solve(input: Self::Input) -> Self::Solution {
    let tail_positions = get_tail_positions(&input.moves);
    let number_of_positions = tail_positions.len();

    Self::Solution {
      number_of_positions,
    }
  }

  fn output(solution: Self::Solution) -> String {
    format!("Number of positions {}", solution.number_of_positions)
  }
}
