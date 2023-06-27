use linked_hash_set::LinkedHashSet;
use pancurses::{
  endwin, init_pair, initscr, COLOR_BLACK, COLOR_PAIR, COLOR_WHITE,
};
use rayon::prelude::*;
use std::{
  cmp::{max, min},
  collections::HashSet,
  ops::Deref,
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
};
use tokio::{sync::Mutex, time::Duration};

use super::common::prelude::*;


const FPS: u64 = 1000 / 60;
const AIR: char = '.';
const SAND_ENTRY: char = '+';
const SAND: char = 'o';
const ROCK: char = '#';

#[derive(Clone, Default)]
pub struct RenderMap {
  segments: Vec<PathSegment>,
  bounds: (usize, usize, usize),
  sand_entry: Option<Coord>,
  entities: LinkedHashSet<Entity>,
  limit: bool,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Entity {
  Rock(Coord),
  Sand(Coord),
  SandEntry(Coord),
}

impl Entity {
  pub fn position_from_entity(entity: &Entity) -> Option<(usize, usize)> {
    match entity {
      Entity::SandEntry(position) => Some(*position),
      Entity::Sand(position) => Some(*position),
      Entity::Rock(position) => Some(*position),
    }
  }

  pub fn identifier_from_entity(entity: &Entity) -> char {
    match entity {
      Entity::Rock(_) => ROCK,
      Entity::Sand(_) => SAND,
      Entity::SandEntry(_) => SAND_ENTRY,
    }
  }

  pub fn entity_is_at_coord(entity: &Entity, coord: Coord) -> bool {
    match entity {
      Entity::Rock(c) => *c == coord,
      Entity::Sand(c) => *c == coord,
      Entity::SandEntry(c) => *c == coord,
    }
  }
}

pub struct Curses {
  render_map: Arc<Mutex<RenderMap>>,
  terminate_render: Arc<AtomicBool>,
}

impl Default for Curses {
  fn default() -> Self {
    Self {
      render_map: Default::default(),
      terminate_render: Arc::new(AtomicBool::new(false)),
    }
  }
}

impl Curses {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn init(&self) {
    init_pair(1, COLOR_WHITE, COLOR_BLACK); // Air
    init_pair(2, COLOR_BLACK, COLOR_WHITE); // Rock
  }

  pub fn render(&self) {
    let terminate_render = Arc::clone(&self.terminate_render);
    let render_map_clone = Arc::clone(&self.render_map);

    let window = initscr();
    window.keypad(true);
    window.nodelay(true);
    window.timeout(0);

    let render_map_guard = tokio::task::block_in_place(|| {
      tokio::runtime::Handle::current().block_on(render_map_clone.lock())
    });
    let render_map = render_map_guard.deref();
    let max_y = if render_map.limit {
      render_map.bounds.2 + 1
    } else {
      render_map.bounds.2
    };
    for y in 0..=max_y {
      for x in render_map.bounds.0..=render_map.bounds.1 {
        let terminal_x = x - render_map.bounds.0;
        if let Some(entry) = render_map.sand_entry {
          let entry_entity = Entity::SandEntry(entry);
          if Entity::entity_is_at_coord(&entry_entity, (x, y)) {
            let identifier = Entity::identifier_from_entity(&entry_entity);
            window.attrset(COLOR_PAIR(2));
            window.mvaddch(y as i32, terminal_x as i32, identifier);
            continue;
          }
        }
        if let Some(entity) = render_map
          .entities
          .iter()
          .find(|&entity| Entity::entity_is_at_coord(entity, (x, y)))
        {
          let identifier = Entity::identifier_from_entity(entity);
          window.attrset(COLOR_PAIR(2));
          window.mvaddch(y as i32, terminal_x as i32, identifier);
        } else {
          window.attrset(COLOR_PAIR(1));
          window.mvaddch(y as i32, terminal_x as i32, AIR);
        }
      }
    }
    drop(render_map_guard);

    window.refresh();

    let mut previous_entities_length = 0;
    let mut previous_last_entity_position: Option<(usize, usize)> = None;

    loop {
      let render_map_guard = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(render_map_clone.lock())
      });
      let render_map = render_map_guard.deref();
      let min_x = render_map.bounds.0;

      let current_last_entity_position = render_map
        .entities
        .iter()
        .last()
        .map(|e| Entity::position_from_entity(e).unwrap());

      if render_map.entities.len() != previous_entities_length {
        // New render for each new sand entity
        for entity in render_map.entities.iter().skip(previous_entities_length)
        {
          if let Entity::Sand(position) = entity {
            let (x, y) = position;
            let terminal_x = x - min_x;
            window.attrset(COLOR_PAIR(1));
            window.mvaddch(*y as i32, terminal_x as i32, SAND);
          }
        }
      } else if let (Some(previous_position), Some(current_position)) =
        (previous_last_entity_position, current_last_entity_position)
      {
        // Check if the position of the last entity has changed
        if previous_position != current_position {
          let (previous_x, previous_y) = previous_position;
          let (current_x, current_y) = current_position;
          let previous_terminal_x = previous_x - min_x;
          let current_terminal_x = current_x - min_x;

          // Replace old position with previous value
          let identifier = match render_map.sand_entry {
            Some(entity) => {
              let entry = Entity::SandEntry(entity);
              if Entity::entity_is_at_coord(&entry, previous_position) {
                window.attrset(COLOR_PAIR(2));
                SAND_ENTRY
              } else {
                window.attrset(COLOR_PAIR(1));
                AIR
              }
            }
            None => AIR,
          };
          window.mvaddch(
            previous_y as i32,
            previous_terminal_x as i32,
            identifier,
          );

          // Update new position with the identifier for Sand
          window.attrset(COLOR_PAIR(1));
          window.mvaddch(current_y as i32, current_terminal_x as i32, SAND);
        }
      }

      window.refresh();
      if terminate_render.load(Ordering::SeqCst) {
        break;
      }
      std::thread::sleep(Duration::from_millis(FPS));

      // Update the previous state for the next frame
      previous_entities_length = render_map.entities.len();
      previous_last_entity_position = current_last_entity_position;
    }

    endwin();
  }

  pub async fn set_paths(&self, paths: Vec<PathSegment>) {
    let render_map = Arc::clone(&self.render_map);
    let bounds = get_render_bounds(&paths);
    let entities = compute_entities(&paths);

    let mut render_map_guard = render_map.lock().await;
    render_map_guard.segments = paths;
    render_map_guard.bounds = bounds;
    render_map_guard.entities = entities;
  }

  pub async fn set_sand_entry(&self, coord: Coord) {
    let mut render_map_guard = self.render_map.lock().await;
    render_map_guard.sand_entry = Some(coord);
  }

  pub async fn set_limit(&self, value: bool) {
    let mut render_map_guard = self.render_map.lock().await;
    render_map_guard.limit = value;
  }

  pub fn stop_rendering(&self) {
    self.terminate_render.store(true, Ordering::SeqCst);
  }

  pub async fn release_sand(&self, delay: u64) -> Option<Coord> {
    let render_map_guard = self.render_map.lock().await;

    let Some(original_position) = render_map_guard.sand_entry else {
      return None;
    };

    let entities: HashSet<Entity> =
      HashSet::from_iter(render_map_guard.entities.clone().into_iter());
    let is_valid_move = |coord| {
      entities
        .par_iter()
        .all(|entity| !Entity::entity_is_at_coord(entity, coord))
    };
    if !is_valid_move(original_position) {
      return None;
    }
    let mut current_position = Some(original_position);
    let bounds = render_map_guard.bounds;
    let max_y = if render_map_guard.limit {
      bounds.2 + 1
    } else {
      bounds.2
    };
    drop(render_map_guard);

    loop {
      let mut render_map_guard = self.render_map.lock().await;
      let position = match current_position {
        Some(c) if is_valid_move((c.0, c.1 + 1)) => (c.0, c.1 + 1),
        Some(c) if is_valid_move((c.0 - 1, c.1 + 1)) => (c.0 - 1, c.1 + 1),
        Some(c) if is_valid_move((c.0 + 1, c.1 + 1)) => (c.0 + 1, c.1 + 1),
        _ => current_position.unwrap(),
      };
      if position == current_position.unwrap() {
        if render_map_guard.limit && position.1 == 0 {
          current_position = None;
        }
        break;
      }
      if render_map_guard.limit {
        if position.1 > max_y {
          // Sand reached the last valid y position
          current_position = Some(position);
          break;
        }
      } else if position.0 < bounds.0
        || position.0 > bounds.1
        || position.1 > max_y
      {
        render_map_guard
          .entities
          .remove(&Entity::Sand(current_position.unwrap()));
        current_position = None;
        break;
      }

      // Update RenderMap with the new sand position
      render_map_guard
        .entities
        .remove(&Entity::Sand(current_position.unwrap()));
      render_map_guard.entities.insert(Entity::Sand(position));

      current_position = Some(position);

      if delay > 0 {
        tokio::time::sleep(Duration::from_millis(delay)).await;
      }
    }

    current_position
  }
}

fn get_render_bounds(paths: &[PathSegment]) -> (usize, usize, usize) {
  let mut min_x = usize::MAX;
  let mut max_x = usize::MIN;
  let mut max_y = usize::MIN;

  for path in paths {
    for (x, y) in [path.0, path.1] {
      min_x = min(min_x, x);
      max_x = max(max_x, x);
      max_y = max(max_y, y);
    }
  }

  (min_x, max_x, max_y)
}

fn compute_entities(paths: &[PathSegment]) -> LinkedHashSet<Entity> {
  let mut entities = LinkedHashSet::new();

  for path in paths {
    let &((x1, y1), (x2, y2)) = path;

    if x1 == x2 {
      let start_y = min(y1, y2);
      let end_y = max(y1, y2);

      for new_y in start_y..=end_y {
        entities.insert(Entity::Rock((x1, new_y)));
      }
    } else if y1 == y2 {
      let start_x = min(x1, x2);
      let end_x = max(x1, x2);

      for new_x in start_x..=end_x {
        entities.insert(Entity::Rock((new_x, y1)));
      }
    }
  }

  entities
}
