mod smallest_large_module;
use smallest_large_module::ProblemSolverPattern;
mod problem_solver;
use problem_solver::solve_problem;
mod du_directories;
mod fs_graph;


fn main() {
  solve_problem::<ProblemSolverPattern>();
}
