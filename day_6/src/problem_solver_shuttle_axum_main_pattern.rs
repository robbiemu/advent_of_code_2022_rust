use axum::{ routing::post, Router };
mod problem_solver_shuttle_axum_module_pattern;
use problem_solver_shuttle_axum_module_pattern::ProblemSolverPattern;
mod problem_solver_shuttle_axum;
use problem_solver_shuttle_axum::solve_problem;


async fn problem_solver_axum(payload: String) 
-> &'static str 
{
  solve_problem::<ProblemSolverPattern>(payload, None)
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
  let router = Router::new().route("/", post(problem_solver_axum));
  
  Ok(router.into())
}
