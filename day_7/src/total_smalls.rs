mod total_smalls_module;
use total_smalls_module::ProblemSolverPattern;
mod problem_solver;
use problem_solver::solve_problem;
mod fs_graph;
mod du_directories;


fn main() {
  solve_problem::<ProblemSolverPattern>();
}
