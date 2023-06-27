use async_trait::async_trait;

use super::common::{prelude::*, read_paths};
use super::curses::Curses;
use super::problem_solver_async::ProblemSolver;
use crate::common::render_solution;


const SAND_ENTRY: (usize, usize) = (500, 0);
const STEP: u64 = 10 / 6;

pub struct PSInput {
  rock_structures: Vec<PathSegment>,
  sand_entry: Coord,
}

pub struct PSSolution {
  rock_structures: Vec<PathSegment>,
  sand_entry: Coord,
  units_of_sand: u32,
}

pub struct ProblemSolverPattern;

#[async_trait]
impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let rock_structures = read_paths(lines);
    PSInput { rock_structures, sand_entry: SAND_ENTRY }
  }

  async fn solve_async(input: Self::Input) -> Self::Solution {
    let curses = Curses::new();
    curses.set_paths(input.rock_structures.clone()).await;
    curses.set_sand_entry(input.sand_entry).await;
    let mut i = 0;
    while curses.release_sand(0).await.is_some() {
      i += 1;
    }

    Self::Solution {
      rock_structures: input.rock_structures,
      sand_entry: input.sand_entry,
      units_of_sand: i,
    }
  }

  async fn output_async(solution: Self::Solution) {
    render_solution(
      solution.rock_structures,
      solution.sand_entry,
      solution.units_of_sand,
      STEP,
      false,
    )
    .await;

    println!("units of sand: {}", solution.units_of_sand);
  }
}
