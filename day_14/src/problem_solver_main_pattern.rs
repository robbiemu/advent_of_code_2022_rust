mod problem_solver_module_pattern;
use problem_solver_module_pattern::ProblemSolverPattern;
mod problem_solver_async;
use problem_solver_async::solve_problem;


fn main() {
  solve_problem::<ProblemSolverPattern>();
}
