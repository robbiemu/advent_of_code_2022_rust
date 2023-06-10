use super::problem_solver_axum::ProblemSolver;
use super::common::{get_map, rotate_2d_collection};


pub struct PSInput {
  map: Vec<Vec<u32>>
}

pub struct PSSolution {
  score: usize
}

pub struct Part2Solver;

impl ProblemSolver for Part2Solver {
  type Input = PSInput;
  type Solution = PSSolution;
  
  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let map = get_map::<u32>(lines);
    Self::Input { map }
  }
  
  fn solve(input: Self::Input) -> Self::Solution {
    let rotated = rotate_2d_collection(&input.map);

    let score = calculate_best_score(&input.map, &rotated);

    PSSolution { score }
  }
  
  fn output(solution: Self::Solution) -> String {
    format!("{} visibility score", solution.score)
  }
}

fn calculate_best_score(map: &Vec<Vec<u32>>, rotated: &Vec<Vec<u32>>) -> usize {
  let last_y = map.len() - 1;
  let last_x = map[0].len() - 1;
  map.iter().enumerate()
    .map(|(i, row)| 
      if i == 0 || i == last_y {
        0
      } else {
        row.iter().enumerate().fold(0, |acc, (j, _)| {
          if j == 0 || j == last_x {
            acc
          } else {
            usize::max(acc, get_scenic_score((i, j), &map, &rotated))
          }
        })        
      }
    ).max().unwrap()
}

fn get_scenic_score(pos: (usize, usize), map: &Vec<Vec<u32>>, 
rotated: &Vec<Vec<u32>>) -> usize {
  let tree = map[pos.0][pos.1];
  let cardinal_view = |(index, &t)| {
    if tree <= t {
        Some(index + 1)
    } else {
        None
    }
  };
  let left = map[pos.0][..pos.1].iter().rev().enumerate().find_map(cardinal_view).unwrap_or(map[pos.0][..pos.1].len());
  let right = map[pos.0][pos.1 + 1..].iter().enumerate()
  .find_map(cardinal_view).unwrap_or(map[pos.0][pos.1 + 1..].len());
  let up = rotated[pos.1][..pos.0].iter().rev().enumerate()
  .find_map(cardinal_view).unwrap_or(rotated[pos.1][..pos.0].len());
  let down = rotated[pos.1][pos.0 + 1..].iter().enumerate()
  .find_map(cardinal_view).unwrap_or(rotated[pos.1][pos.0 + 1..].len());

  println!("{} {:?} => {} [{:?}:{} {:?}:{} {:?}:{} {:?}:{}]", tree, pos, left * right * up * down, map[pos.0][..pos.1].iter().rev().collect::<Vec<_>>(), left, map[pos.0][pos.1 + 1..].to_vec(), right, rotated[pos.1][..pos.0].iter().rev().collect::<Vec<_>>(), up, rotated[pos.1][pos.0 + 1..].to_vec(), down);

  left * right * up * down
}
