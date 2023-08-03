mod part2_solver;
use part2_solver::ProblemSolverPattern;
mod problem_solver;
use problem_solver::solve_problem;
mod common;
mod cube;


fn main() {
  solve_problem::<ProblemSolverPattern>();
}
