mod part1_solver;
use part1_solver::ProblemSolverPattern;
mod problem_solver;
use problem_solver::solve_problem;
mod common;

fn main() {
  solve_problem::<ProblemSolverPattern>();
}
