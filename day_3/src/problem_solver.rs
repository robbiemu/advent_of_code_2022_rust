use std::fs::File;
use std::io::BufRead;

pub trait ProblemSolver {
  type Input;
  type Solution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input;
  fn solve(input: Self::Input) -> Self::Solution;
  fn output(solution: Self::Solution);
}

pub fn solve_problem<T: ProblemSolver>() {
  let args: Vec<String> = std::env::args().collect();
  let input_filename = &args[1];
  if let Ok(lines) = read_lines(input_filename) {
    let input = T::initialize(lines.map(|l| l.unwrap()));
    let solution = T::solve(input);
    T::output(solution);
  }
}

fn read_lines<P>(
  filename: P,
) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
where
  P: AsRef<std::path::Path>,
{
  let file = File::open(filename)?;
  Ok(std::io::BufReader::new(file).lines())
}
