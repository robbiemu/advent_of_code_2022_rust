use std::cmp::Ordering;

use super::common::*;
use super::problem_solver::ProblemSolver;


pub struct PSInput {
  pairs: Vec<(String, String)>,
}

pub struct PSSolution {
  ordered_indices: Vec<usize>,
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let pairs = lines
      .filter(|l| !l.is_empty())
      .collect::<Vec<String>>()
      .chunks(2)
      .map(|str_slices| (str_slices[0].to_string(), str_slices[1].to_string()))
      .collect();

    Self::Input { pairs }
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let ordered_indices = input
      .pairs
      .iter()
      .enumerate()
      .filter_map(|(i, (left, right))| {
        ordering(i + 1, left.into(), right.into())
      })
      .collect();
    Self::Solution { ordered_indices }
  }

  fn output(solution: Self::Solution) {
    println!(
      "{:?}\nsum of indices: {}",
      solution.ordered_indices,
      solution.ordered_indices.iter().sum::<usize>()
    );
  }
}

fn ordering(i: usize, left: String, right: String) -> Option<usize> {
  // we're making parentheses into numbers to ease comparison.
  if compare(left.as_bytes(), right.as_bytes()) != Ordering::Greater {
    Some(i)
  } else {
    None
  }
}
