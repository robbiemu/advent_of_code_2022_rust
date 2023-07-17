use super::problem_solver::ProblemSolver;
use crate::common::get_coordinates;


const DECRYPTION_KEY: i64 = 811589153;
const MIXES: u8 = 10;

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
    let mut codex = input.codex.iter().map(|x| x * DECRYPTION_KEY).collect();
    let mut indices: Vec<usize> = (0..input.codex.len()).collect();
    for _ in 0..MIXES {
      (codex, indices) = move_items(&mut codex, &indices);
    }
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

fn move_items(
  numbers: &mut Vec<i64>,
  indices: &[usize],
) -> (Vec<i64>, Vec<usize>) {
  let len = numbers.len();
  let mut index = 0;
  let mut work: Vec<(usize, i64)> = indices
    .iter()
    .cloned()
    .zip(numbers.iter().cloned())
    .collect();

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

  (
    work.iter().map(|(_, m)| *m).collect(),
    work.iter().map(|(i, _)| *i).collect(),
  )
}
