use super::common::{get_map, rotate_2d_collection};
use super::problem_solver_service::ProblemSolver;


pub struct PSInput {
  map: Vec<Vec<u8>>,
}

pub struct PSSolution {
  visible: u32,
}

pub struct Part1Solver;

impl ProblemSolver for Part1Solver {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let map = get_map::<u8>(lines);
    Self::Input { map }
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let rotated = rotate_2d_collection(&input.map);
    let last_y = input.map.len() - 1;
    let last_x = input.map[0].len() - 1;

    let visible = calculate_visible(&input.map, &rotated, last_x, last_y);

    PSSolution { visible }
  }

  fn output(solution: Self::Solution) -> String {
    format!("{} trees visible", solution.visible)
  }
}

fn calculate_visible(
  map: &[Vec<u8>],
  rotated: &[Vec<u8>],
  last_x: usize,
  last_y: usize,
) -> u32 {
  map
    .iter()
    .enumerate()
    .map(|(i, row)| {
      row.iter().enumerate().fold(0, |acc, (j, tree)| {
        let max_left = row[..j].iter().max().unwrap_or(&0);
        let max_right = row[j + 1..].iter().max().unwrap_or(&0);
        let max_up = rotated[j][..i].iter().max().unwrap_or(&0);
        let max_down = rotated[j][i + 1..].iter().max().unwrap_or(&0);

        tracing::debug!(
          "{} | l{} r{} u{} d{}",
          tree,
          max_left,
          max_right,
          max_up,
          max_down
        );

        if (max_left < tree
          || max_right < tree
          || max_up < tree
          || max_down < tree)
          || i == 0
          || i == last_y
          || j == 0
          || j == last_x
        {
          acc + 1
        } else {
          acc
        }
      })
    })
    .sum()
}
