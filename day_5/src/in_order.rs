mod in_order_module;
use in_order_module::ProblemSolverPattern;
mod problem_solver;
use problem_solver::solve_problem;


fn main() {
  solve_problem::<ProblemSolverPattern>();
}
