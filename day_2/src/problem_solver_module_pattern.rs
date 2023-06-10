use std::fs::File;
use std::io::BufRead;


pub mod problem_solver_module {
  struct PSInput {}

  pub fn initialize(lines: Lines<BufReader<File>>) -> PSInput {
    for line in lines {
      if let Ok(record) = line {}
    }
  }

  pub fn solve(input: PSInput) -> PSSolution {}

  pub fn output(solution: PSSolution) {
    println!("hi mom!");
  }
}
