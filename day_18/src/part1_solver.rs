use std::collections::HashMap;

use ndarray::{Array, ArrayBase, Dim, OwnedRepr};

use super::problem_solver::ProblemSolver;
use crate::common::{get_neighbors, prelude::*};


const MAX_COORDS: (usize, usize, usize) = (22, 22, 22);

pub struct PSInput {
  three_d: ArrayBase<OwnedRepr<bool>, Dim<[usize; 3]>>,
}

pub struct PSSolution {
  externalized_points: HashMap<Position, usize>,
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let mut three_d = Array::from_elem(MAX_COORDS, false);

    lines.for_each(|l| {
      let coords: Vec<usize> =
        l.split(',').map(|w| w.parse::<usize>().unwrap()).collect();

      let x = coords[0];
      let z = coords[1];
      let y = coords[2];

      three_d[[x, y, z]] = true;
    });

    Self::Input { three_d }
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let (x_len, y_len, z_len) = input.three_d.dim();

    let mut externalized_points = HashMap::new();
    for x in 0..x_len {
      for y in 0..y_len {
        for z in 0..z_len {
          let position = (x, y, z);
          if input.three_d[position] {
            let neighbors = get_neighbors(position, &input.three_d);
            let n_len = neighbors.iter().filter(|p| input.three_d[**p]).count();
            externalized_points.entry(position).or_insert(6 - n_len);
          }
        }
      }
    }

    Self::Solution { externalized_points }
  }

  fn output(solution: Self::Solution) {
    println!(
      "total {}",
      solution.externalized_points.values().sum::<usize>()
    );
  }
}
