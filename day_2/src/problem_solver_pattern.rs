use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use problem_solver_module::{initialize, output, solve};


fn main() {
  let args: Vec<String> = env::args().collect();
  let input_filename = &args[1];
  if let Ok(lines) = read_lines(input_filename) {
    let input = initialize(lines);
    let solution = solve(input);
    output(solution);
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
