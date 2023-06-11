use hyper::{
  body::to_bytes,
  server::conn::AddrStream,
  service::{make_service_fn, service_fn},
  Body, Request, Response, Server,
};
use std::convert::Infallible;

mod port_state_behavior;
use port_state_behavior::get_port;
mod problem_solver_service;
use problem_solver_service::solve_problem;
mod part_1_module;
use part_1_module::Part1Solver;
mod common;


#[tokio::main]
async fn main() {
  pretty_env_logger::init();

  let port = get_port();
  let addr = ([0, 0, 0, 0], port).into();

  let make_svc = make_service_fn(|_socket: &AddrStream| async move {
    Ok::<_, Infallible>(service_fn(move |req: Request<Body>| async move {
      let body_bytes = to_bytes(req.into_body()).await.unwrap();
      let raw_string = String::from_utf8_lossy(&body_bytes).to_string();
      let result = solve_problem::<Part1Solver>(raw_string);

      Ok::<_, Infallible>(Response::new(Body::from(result)))
    }))
  });

  let server = Server::bind(&addr).serve(make_svc);

  println!("Listening on http://{}", addr);
  if let Err(e) = server.await {
    eprintln!("server error: {}", e);
  }
}
