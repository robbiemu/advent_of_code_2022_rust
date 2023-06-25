mod problem_solver_module_pattern;
use problem_solver_module_pattern::ProblemSolverPattern;
mod problem_solver;
use problem_solver::solve_problem;


fn main() {
  solve_problem::<ProblemSolverPattern>();
}
