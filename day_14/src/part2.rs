mod part2_module;
use part2_module::ProblemSolverPattern;
mod problem_solver_async;
use problem_solver_async::solve_problem;
mod common;
mod curses;


#[tokio::main]
async fn main() {
  solve_problem::<ProblemSolverPattern>().await;
}
