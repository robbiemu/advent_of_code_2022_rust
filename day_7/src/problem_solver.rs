use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use std::process;


pub trait ProblemSolver {
  type Input;
  type Solution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input;
  fn solve(input: Self::Input) -> Self::Solution;
  fn output(solution: Self::Solution);
}

pub fn solve_problem<T: ProblemSolver>() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    eprintln!("must provide an input file (from context of current working directory)");
    process::exit(1);
  }
  let input_filename = &args[1];
  match read_lines(input_filename) {
    Ok(lines) => {
      let input = T::initialize(lines.map(|l| l.unwrap()));
      let solution = T::solve(input);
      T::output(solution);
    }
    Err(err) => {
      eprintln!("Failed to open input file: {}", err);
      process::exit(1);
    }
  }
}

fn read_lines<P>(filename: P) -> std::io::Result<Lines<BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(BufReader::new(file).lines())
}
