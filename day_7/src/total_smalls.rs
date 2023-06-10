mod total_smalls_module;
use total_smalls_module::ProblemSolverPattern;
mod problem_solver;
use problem_solver::solve_problem;
mod du_directories;
mod fs_graph;


fn main() {
  solve_problem::<ProblemSolverPattern>();
}
