use std::cmp::Ordering;

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

fn compare(left: &[u8], right: &[u8]) -> Ordering {
  let mut li = 0;
  let mut ri = 0;
  let mut l_comp = left[0];
  let mut r_comp = right[0];
  if left.get(0..2) == Some(&[49, 48]) {
    li = 1;
    l_comp = b'a'; // placeholder for b"10", which is the maximum in the input
  }
  if right.get(0..2) == Some(&[49, 48]) {
    ri = 1;
    r_comp = b'a';
  }
  match (l_comp, r_comp) {
    (l, r) if l == r => compare(&left[li + 1..], &right[ri + 1..]),
    // now a & b are never equal:
    (l, b'[') => {
      let converted_left = [&[l, b']'], &left[li + 1..]].concat();

      compare(&converted_left, &right[ri + 1..])
    }
    (_, b']') => Ordering::Greater,
    (b'[', r) => {
      let converted_right = [&[r, b']'], &right[ri + 1..]].concat();

      compare(&left[li + 1..], &converted_right)
    }
    (b']', _) => Ordering::Less,
    (l, r) => l.cmp(&r), // a comma is 4 less than b'0'
  }
}
