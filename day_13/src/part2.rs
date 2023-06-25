mod part2_module;
use part2_module::ProblemSolverPattern;
mod problem_solver;
use problem_solver::solve_problem;
mod common;


fn main() {
  solve_problem::<ProblemSolverPattern>();
}
