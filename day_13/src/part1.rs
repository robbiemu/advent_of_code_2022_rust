mod part1_module;
use part1_module::ProblemSolverPattern;
mod problem_solver;
use problem_solver::solve_problem;
mod common;

fn main() {
  solve_problem::<ProblemSolverPattern>();
}
