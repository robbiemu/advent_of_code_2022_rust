use super::problem_solver_axum::ProblemSolver;
use super::common::rotate_2d_collection;


pub struct PSInput {
  map: Vec<Vec<u8>>
}

pub struct PSSolution {
  visible: u32
}

pub struct Part1Solver;

impl ProblemSolver for Part1Solver {
  type Input = PSInput;
  type Solution = PSSolution;
  
  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let mut map: Vec<Vec<u8>> = Vec::new();
    for line in lines {
      if line.is_empty() {
        panic!(
          "invalid input format, as line is not composed of characters"
        );
      }
      map.push(line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect());
    }
    
    Self::Input { map }
  }
  
  fn solve(input: Self::Input) -> Self::Solution {      
    let rotated = rotate_2d_collection(&input.map);
    println!("Rotated array: {:?}", rotated);  // Print the rotated array
    let last_y = (input.map.len() - 1) as usize;
    let last_x = (input.map[0].len() - 1) as usize;
    
    let visible = input.map.iter().enumerate().map(|(i, row)| {      
      row.iter().enumerate().fold(0, |acc, (j, tree)| {
        let max_left = row[..j].iter().max().unwrap_or(&0);
        let max_right = row[j + 1..].iter().max().unwrap_or(&0);
        let max_up = rotated[j][..i].iter().max().unwrap_or(&0);
        let max_down = rotated[j][i + 1..].iter().max().unwrap_or(&0);
        
        tracing::debug!("{} | l{} r{} u{} d{}", tree, max_left, max_right, max_up, max_down);
        
        if (max_left < tree || max_right < tree 
          || max_up < tree || max_down < tree) 
          || i == 0 || i == last_y
          || j == 0 || j == last_x
        {
          acc + 1
        } else {
          tracing::debug!("found invisible tree {}", tree);
          acc
        }
      })
    }).sum();
    
    
    PSSolution { visible }
  }
  
  fn output(solution: Self::Solution) -> String {
    format!("{} trees visible", solution.visible)
  }
}
