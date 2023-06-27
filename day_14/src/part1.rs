mod part1_module;
use part1_module::ProblemSolverPattern;
mod problem_solver_async;
use problem_solver_async::solve_problem;
mod common;
mod curses;


#[tokio::main]
async fn main() {
  solve_problem::<ProblemSolverPattern>().await;
}
