use axum::{
  Router, routing::post, 
};
use std::net::SocketAddr;
use std::env;

mod part1;
use part1::Part1Solver;
mod part2;
use part2::Part2Solver;
mod problem_solver_axum;
use problem_solver_axum::solve_problem;
mod common;


#[tokio::main]
async fn main() {
  if env::var("RUST_LOG").is_err() {
    // Set RUST_LOG to 'debug' if it's not already defined
    env::set_var("RUST_LOG", "debug");
  }
  tracing_subscriber::fmt::init(); // initialize tracing
  
  let app = Router::new()
  .route("/part_1", post(part_1))
  .route("/part_2", post(part_2));
  
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  tracing::info!("listening on {}", addr);
  axum::Server::bind(&addr)
  .serve(app.into_make_service())
  .await
  .unwrap();
}

async fn part_1(payload: String) -> String {
  solve_problem::<Part1Solver>(payload)
}

async fn part_2(payload: String) -> String {
  solve_problem::<Part2Solver>(payload)
}
