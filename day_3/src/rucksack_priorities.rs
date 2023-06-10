mod rucksack_priorities_module;
use rucksack_priorities_module::ProblemSolverPattern;
mod problem_solver;
use problem_solver::solve_problem;


fn main() {
  solve_problem::<ProblemSolverPattern>();
}
