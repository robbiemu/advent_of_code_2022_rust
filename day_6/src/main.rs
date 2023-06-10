use axum::{routing::post, Router};
mod tuning_trouble_module;
use tuning_trouble_module::{PSContext, ProblemSolverPattern};
mod problem_solver_shuttle_axum;
use problem_solver_shuttle_axum::{solve_problem, ProblemContexts};


const PACKET_WINDOW_SIZE: usize = 4;
const MESSAGE_WINDOW_SIZE: usize = 14;

async fn start_of_packet(payload: String) -> String {
  solve_problem::<ProblemSolverPattern>(
    payload,
    Some(ProblemContexts {
      solve: Some(PSContext::from(PACKET_WINDOW_SIZE)),
      output: Some(PSContext::from(PACKET_WINDOW_SIZE)),
      ..Default::default()
    }),
  )
}

async fn start_of_message(payload: String) -> String {
  solve_problem::<ProblemSolverPattern>(
    payload,
    Some(ProblemContexts {
      solve: Some(PSContext::from(MESSAGE_WINDOW_SIZE)),
      output: Some(PSContext::from(MESSAGE_WINDOW_SIZE)),
      ..Default::default()
    }),
  )
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
  let router = Router::new()
    .route("/start-of-packet", post(start_of_packet))
    .route("/start-of-message", post(start_of_message));

  Ok(router.into())
}
