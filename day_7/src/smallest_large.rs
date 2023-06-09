mod smallest_large_module;
use smallest_large_module::ProblemSolverPattern;
mod problem_solver;
use problem_solver::solve_problem;
mod fs_graph;
mod du_directories;


fn main() {
  solve_problem::<ProblemSolverPattern>();
}
