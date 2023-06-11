use super::problem_solver::ProblemSolver;


pub struct PSInput {
  left_ranges: Vec<(u32, u32)>,
  right_ranges: Vec<(u32, u32)>,
}

pub struct PSSolution {
  containments: u32,
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let mut left_ranges: Vec<(u32, u32)> = vec![];
    let mut right_ranges: Vec<(u32, u32)> = vec![];
    for record in lines {
      let (left, right) = record.split_once(',').unwrap_or_else(|| {
        panic!(
          "precondition failure! lines must be ranges of positive integers of \
           the form x-y,a-b, found (record):\n{}",
          record
        )
      });
      let fmt_error = &format!(
        "precondition failure! lines must be ranges of positive integers of \
         the form x-y,a-b, found:\n{}",
        record
      );
      let (x, y) = left.split_once('-').expect(fmt_error);
      let left_range =
        (x.parse().expect(fmt_error), y.parse().expect(fmt_error));
      left_ranges.push(left_range);
      let (a, b) = right.split_once('-').expect(fmt_error);
      let right_range =
        (a.parse().expect(fmt_error), b.parse().expect(fmt_error));
      right_ranges.push(right_range);
    }

    PSInput { left_ranges, right_ranges }
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let containments =
      input
        .left_ranges
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (x, y))| {
          let (a, b) = input.right_ranges[i];
          #[allow(clippy::nonminimal_bool)]
          if (&a <= y && &b >= x) || (x <= &b && y >= &a) {
            println!("overlap found for {:?} and {:?}", (a, b), (x, y));
            acc + 1
          } else {
            acc
          }
        });

    PSSolution { containments }
  }

  fn output(solution: Self::Solution) {
    println!("containments found: {}", solution.containments);
  }
}
