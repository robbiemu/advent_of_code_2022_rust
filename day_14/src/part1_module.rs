use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use crossterm::event::{self, Event, KeyCode};

use super::common::prelude::*;
use super::curses::Curses;
use super::problem_solver_async::ProblemSolver;


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
    let mut rock_structures: Vec<PathSegment> = Vec::new();
    for record in lines {
      rock_structures.extend::<Vec<PathSegment>>(
        record
          .split(" -> ")
          .collect::<Vec<_>>()
          .windows(2)
          .map(|w| {
            let left = w[0].split_once(',').expect("invalid format");
            let right = w[1].split_once(',').expect("invalid format");

            (
              (left.0.parse().unwrap(), left.1.parse().unwrap()),
              (right.0.parse().unwrap(), right.1.parse().unwrap()),
            )
          })
          .collect(),
      );
    }

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
    let curses = Arc::new(Curses::new());
    curses.init();
    curses.set_paths(solution.rock_structures).await;
    curses.set_sand_entry(solution.sand_entry).await;

    let curses_clone = Arc::clone(&curses);
    tokio::spawn(async move {
      tokio::time::sleep(Duration::from_millis(100)).await;
      for _ in 0..solution.units_of_sand {
        curses_clone.release_sand(STEP).await;
        if let Ok(Event::Key(key_event)) = event::read() {
          if key_event.code == KeyCode::Esc {
            curses_clone.stop_rendering();
          }
        }
      }
      tokio::time::sleep(Duration::from_secs(1)).await;

      curses_clone.stop_rendering();
    });

    curses.render();

    println!("units of sand: {}", solution.units_of_sand);
  }
}
