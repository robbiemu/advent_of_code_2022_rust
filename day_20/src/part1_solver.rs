use super::problem_solver::ProblemSolver;
use crate::common::get_coordinates;


pub struct PSInput {
  codex: Vec<i64>,
}

pub struct PSSolution {
  coordinates: [i64; 3],
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let codex = lines
      .map(|s| {
        str::parse::<i64>(s.as_str())
          .unwrap_or_else(|_| panic!("invalid input format"))
      })
      .collect();

    Self::Input { codex }
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let mut source = input.codex;
    let codex = move_items(&mut source);
    // println!("{:?}", codex);

    let coordinates = get_coordinates(codex);

    Self::Solution { coordinates }
  }

  fn output(solution: Self::Solution) {
    println!(
      "sum {} {:?}",
      solution.coordinates.iter().sum::<i64>(),
      solution.coordinates
    );
  }
}

fn move_items(numbers: &mut Vec<i64>) -> Vec<i64> {
  let len = numbers.len();
  let mut index = 0;
  let mut work: Vec<(usize, i64)> =
    numbers.iter().enumerate().map(|(i, v)| (i, *v)).collect();

  loop {
    let i = work.iter().position(|(i, _x)| *i == index).unwrap();
    let (mi, m) = work[i];
    if m != 0 {
      let new_index = ((i as i64 + m).rem_euclid(len as i64 - 1)) as usize;

      work.remove(i);
      work.insert(new_index, (mi, m));
    }

    // println!("{:?}", work); // Debug print

    if index == len - 1 {
      break;
    }
    index += 1;
  }

  work.iter().map(|(_, m)| *m).collect()
}
