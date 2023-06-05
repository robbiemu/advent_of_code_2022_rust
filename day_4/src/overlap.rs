  mod overlap_module;
  use overlap_module::ProblemSolverPattern;
  mod problem_solver;
  use problem_solver::solve_problem;

  fn main() {
    solve_problem::<ProblemSolverPattern>();
  }