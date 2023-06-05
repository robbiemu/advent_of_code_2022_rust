use super::problem_solver::ProblemSolver;

pub struct PSInput {
  left_ranges: Vec<(u32, u32)>,
  right_ranges: Vec<(u32, u32)>
}

pub struct PSSolution {
  containments: u32
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;
  
  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let mut left_ranges: Vec<(u32, u32)> = vec![];
    let mut right_ranges: Vec<(u32, u32)> = vec![];
    for record in lines {
      let Some((left, right)) = record.split_once(",") else {
        println!("precondition failure! lines must be ranges of positive integers of the form x-y,a-b, found (record):\n{}", record);
        unreachable!()
      };
      let Some((x,y)) = left.split_once("-") else {
        println!("precondition failure! lines must be ranges of positive integers of the form x-y,a-b, found (left):\n{}", record);
        unreachable!()
      };
      let left_range = (x.parse().unwrap(), y.parse().unwrap());
      let Some((a,b)) = right.split_once("-") else {
        println!("precondition failure! lines must be ranges of positive integers of the form x-y,a-b, found (right):\n{}", record);
        unreachable!()
      };
      let right_range = (a.parse().unwrap(), b.parse().unwrap());
      left_ranges.push(left_range);
      right_ranges.push(right_range);
    }

    PSInput { left_ranges, right_ranges }
  }
  
  fn solve(input: Self::Input) -> Self::Solution {
    let containments = input.left_ranges.iter().enumerate().fold(0, |acc, (i, (x, y))| {
      let (a, b) = input.right_ranges[i];
      if (&a >= x && &b <= y) || (x >= &a && y <= &b) {
        println!("containment found for {:?} and {:?}", (a,b), (x,y));
        acc + 1
      } else {
        acc
      }
    });

    PSSolution { containments}
  }
  
  fn output(solution: Self::Solution) {
    println!("containments found: {}", solution.containments);
  }
}

