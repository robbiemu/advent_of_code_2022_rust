pub mod prelude {
  pub type Coord = (usize, usize);
  pub type PathSegment = (Coord, Coord);
}

use std::{sync::Arc, time::Duration};

use crossterm::event::{self, poll, Event, KeyCode};
use prelude::*;

use crate::curses::Curses;

pub fn read_paths(lines: impl Iterator<Item = String>) -> Vec<PathSegment> {
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

  rock_structures
}

pub async fn render_solution(
  rock_structures: Vec<PathSegment>,
  sand_entry: Coord,
  units_of_sand: u32,
  step: u64,
  limit: bool,
) {
  let curses = Arc::new(Curses::new());
  curses.init();
  if limit {
    curses.set_limit(true).await;
  }
  curses.set_paths(rock_structures).await;
  curses.set_sand_entry(sand_entry).await;

  let curses_clone = Arc::clone(&curses);
  tokio::spawn(async move {
    tokio::time::sleep(Duration::from_millis(step)).await;
    for _ in 0..units_of_sand {
      curses_clone.release_sand(0).await;
      if poll(Duration::from_millis(step)).ok().unwrap() {
        if let Ok(Event::Key(key_event)) = event::read() {
          if key_event.code == KeyCode::Esc {
            curses_clone.stop_rendering();
          }
        }
      }
    }
    tokio::time::sleep(Duration::from_secs(1)).await;

    curses_clone.stop_rendering();
  });

  curses.render();
}
