mod part2_solver;
use part2_solver::ProblemSolverPattern;
mod problem_solver;
use problem_solver::solve_problem;
pub mod common;
mod ru_symbols_simple_parser;

fn main() {
  solve_problem::<ProblemSolverPattern>();
}
