use ndarray::{Array, ArrayBase, Dim, OwnedRepr};
use std::collections::{HashMap, HashSet};

use super::problem_solver::ProblemSolver;
use crate::common::{get_neighbors, prelude::*};


const MAX_COORDS: (usize, usize, usize) = (24, 24, 24);

pub type ReachableMap = HashMap<Position, HashSet<Position>>;

pub struct PSInput {
  three_d: ArrayBase<OwnedRepr<bool>, Dim<[usize; 3]>>,
}

pub struct PSSolution {
  externalized_points: ReachableMap,
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

      let x = coords[0] + 1;
      let z = coords[1] + 1;
      let y = coords[2] + 1;

      three_d[[x, y, z]] = true;
    });

    Self::Input { three_d }
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let externalized_points = dfs(&input.three_d, (0, 0, 0));

    Self::Solution { externalized_points }
  }

  fn output(solution: Self::Solution) {
    solution
      .externalized_points
      .iter()
      .for_each(|((x, y, z), v)| {
        println!("{:?}:{}", (x - 1, y - 1, z - 1), v.len())
      });

    println!(
      "total {}",
      solution
        .externalized_points
        .values()
        .map(|v| v.len())
        .sum::<usize>()
    );
  }
}

pub fn dfs(
  space: &ArrayBase<OwnedRepr<bool>, Dim<[usize; 3]>>,
  position: (usize, usize, usize),
) -> ReachableMap {
  let (x_len, y_len, z_len) = space.dim();

  let mut visited = Array::from_elem(space.dim(), false);
  let mut reachable_points = HashMap::new();

  let mut stack = vec![position];

  while let Some(position) = stack.pop() {
    let (x, y, z) = position;
    if x >= x_len || y >= y_len || z >= z_len || visited[position] {
      continue;
    }
    visited[position] = true;

    let neighbors = get_neighbors(position, space);
    for neighbor in neighbors {
      if space[neighbor] {
        reachable_points
          .entry(neighbor)
          .or_insert_with(HashSet::new)
          .insert(position);
      } else {
        stack.push(neighbor);
      }
    }
  }

  reachable_points
}
