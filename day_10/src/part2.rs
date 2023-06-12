mod part2_mod;
use part2_mod::ProblemSolverPattern;
mod problem_solver;
use problem_solver::solve_problem;
mod common;


fn main() {
  solve_problem::<ProblemSolverPattern>();
}
