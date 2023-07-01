pub mod prelude {
  pub type Coord = (isize, isize);

  #[derive(Clone, Debug, Eq, Hash, PartialEq)]
  pub enum Entity {
    Beacon(Coord),
    Sensor(Coord),
  }

  impl Entity {
    pub fn get_coord(&self) -> Coord {
      match self {
        Entity::Beacon(coord) | Entity::Sensor(coord) => *coord,
      }
    }

    pub fn get_coord_indices(&self) -> (usize, usize) {
      match self {
        Entity::Beacon(coord) | Entity::Sensor(coord) => {
          (coord.0 as usize, coord.1 as usize)
        }
      }
    }
  }

  pub type Record = (Entity, Entity);
  pub type Bounds = (Coord, Coord);

  pub trait Dimensional {
    fn get_dims(&self) -> (isize, isize);
    fn get_width(&self) -> isize;
    fn get_height(&self) -> isize;
  }

  impl Dimensional for Bounds {
    fn get_dims(&self) -> (isize, isize) {
      let bounds_width = self.1 .0 - self.0 .0;
      let bounds_height = self.1 .1 - self.0 .1;

      (bounds_width, bounds_height)
    }
    fn get_width(&self) -> isize {
      self.1 .0 - self.0 .0
    }
    fn get_height(&self) -> isize {
      self.1 .1 - self.0 .1
    }
  }
}

use dioxus::html::p;
use sscanf::sscanf;
use std::{
  cmp::{max, min},
  collections::HashMap,
};

use prelude::*;


pub fn parse_line(line: String) -> Option<Record> {
  let Some((sensor_record, beacon_record)) = line.split_once(": ") else {
    return None
  };

  let sensor =
    match sscanf!(sensor_record.trim(), "Sensor at x={}, y={}", isize, isize) {
      Ok(coord) => Entity::Sensor(coord),
      Err(_) => return None,
    };

  let beacon = match sscanf!(
    beacon_record.trim(),
    "closest beacon is at x={}, y={}",
    isize,
    isize
  ) {
    Ok(coord) => Entity::Beacon(coord),
    Err(_) => return None,
  };

  Some((sensor, beacon))
}

pub fn validate_puzzle(lines: String) -> bool {
  lines
    .trim()
    .lines()
    .all(|l| parse_line(l.to_string()).is_some())
}

pub fn derive_bounds(records: &[Record]) -> Bounds {
  records.iter().fold(
    ((isize::MAX, isize::MAX), (isize::MIN, isize::MIN)),
    |acc, cur| {
      let (sensor, beacon) = match cur {
        (Entity::Sensor(coord1), Entity::Beacon(coord2)) => (coord1, coord2),
        _ => unreachable!(),
      };

      let manhattan_distance =
        (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

      let min_x = min(acc.0 .0, sensor.0 - manhattan_distance);
      let min_y = min(acc.0 .1, sensor.1 - manhattan_distance);
      let max_x = max(acc.1 .0, sensor.0 + manhattan_distance);
      let max_y = max(acc.1 .1, sensor.1 + manhattan_distance);

      ((min_x, min_y), (max_x, max_y))
    },
  )
}

pub fn manhattan_distance(record: &Record) -> usize {
  let sensor = record.0.get_coord();
  let beacon = record.1.get_coord();
  let dx = (beacon.0 - sensor.0).abs();
  let dy = (beacon.1 - sensor.1).abs();

  (dx + dy) as usize
}

pub fn get_bounded_coordinate_indices(
  bounds: &Bounds,
  entity: &Entity,
) -> (usize, usize) {
  let (mut x, mut y) = entity.get_coord_indices();
  if (x as isize - bounds.0 .0 >= 0) && (y as isize - bounds.0 .1 >= 0) {
    x = (x as isize - bounds.0 .0) as usize;
    y = (y as isize - bounds.0 .1) as usize;
  }

  (x, y)
}

// not quite right
pub fn _aco(
  origin: (usize, usize),
  path_length: usize,
  map: &mut Vec<Vec<bool>>,
) {
  let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
  let mut queue: Vec<(usize, usize, usize)> = Vec::new();
  let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
  queue.push((origin.0, origin.1, 0));
  while let Some((cur_x, cur_y, cur_dist)) = queue.pop() {
    if cur_dist > path_length
      || ((visited.contains_key(&(cur_x, cur_y)))
        && *visited.get(&(cur_x, cur_y)).unwrap() <= cur_dist)
    {
      continue;
    }
    //log::info!("{:?}", (cur_x, cur_y));
    visited.insert((cur_x, cur_y), cur_dist);
    map[cur_y][cur_x] = true;

    for &(dx, dy) in &directions {
      let next_x = cur_x as isize + dx;
      let next_y = cur_y as isize + dy;
      if next_x >= 0 && next_y >= 0 {
        let next_y: usize = next_y as usize;
        let next_x: usize = next_x as usize;
        if next_x < map[0].len()
          && next_y < map.len()
          && ((!visited.contains_key(&(next_x, next_y)))
            || *visited.get(&(next_x, next_y)).unwrap() <= cur_dist + 1)
        {
          queue.push((next_x, next_y, cur_dist + 1));
        }
      }
    }
  }
}

pub fn solve_to(
  origin: (usize, usize),
  target_y: usize,
  path_length: usize,
  bounds: Bounds,
  row: &mut [bool],
) {
  let breadth = (origin.1 as isize - target_y as isize).unsigned_abs();
  let extent = path_length - breadth;
  log::info!(
    "{:?}",
    (origin.0 as isize - extent as isize + bounds.0 .0
      ..=origin.0 as isize + extent as isize + bounds.0 .0)
  );
  (origin.0 - extent..=origin.0 + extent).for_each(|x| row[x] = true);
}
