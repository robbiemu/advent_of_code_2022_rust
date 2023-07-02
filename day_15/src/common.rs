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

use sscanf::sscanf;
use std::cmp::{max, min};

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
) -> Option<(usize, usize)> {
  let (a, b) = entity.get_coord();
  let x: usize;
  let y: usize;
  if (a - bounds.0 .0 >= 0) && (b - bounds.0 .1 >= 0) {
    x = (a - bounds.0 .0) as usize;
    y = (b - bounds.0 .1) as usize;

    return Some((x, y));
  }

  None
}

pub fn solve_to(
  origin: (usize, usize),
  target_y: usize,
  path_length: usize,
  bounds: Bounds,
) -> Coord {
  let breadth = (origin.1 as isize - target_y as isize).unsigned_abs();
  let extent = path_length - breadth;
  log::info!(
    "{:?}",
    (origin.0 as isize - extent as isize + bounds.0 .0
      ..=origin.0 as isize + extent as isize + bounds.0 .0)
  );
  ((origin.0 - extent) as isize, (origin.0 + extent) as isize)
}

pub fn extend_coord_ranges(range: Coord, ranges: &mut [Coord]) -> Vec<Coord> {
  let mut left = usize::MAX;
  let mut right = usize::MAX;
  let mut current_merge = usize::MAX;
  let mut exclude: Vec<usize> = Vec::new();

  for (i, r) in ranges.iter().enumerate() {
    match check_overlap(*r, range) {
      Some(Overlap::Left) => {
        left = i;
        current_merge = i;
      }
      Some(Overlap::Right) => {
        right = i;
        current_merge = i;
      }
      Some(Overlap::Contains) => {
        right = i;
        break;
      }
      Some(Overlap::Contained) => {
        exclude.push(i);
      }
      None => (),
    }

    if current_merge != usize::MAX {
      for (j, r2) in ranges.iter().enumerate().skip(i + 1) {
        match check_overlap(*r2, range) {
          Some(Overlap::Left) => {
            left = j;
            break;
          }
          Some(Overlap::Right) => {
            right = j;
            break;
          }
          Some(Overlap::Contained) => {
            exclude.push(j);
          }
          _ => (),
        }
      }
      break;
    }
  }

  let mut merged_ranges: Vec<Coord> = Vec::new();
  let mut working_range = range;
  if left != usize::MAX {
    working_range = merge_coord_range(ranges[left], working_range);
  }
  if right != usize::MAX {
    working_range = merge_coord_range(ranges[right], working_range);
  }

  merged_ranges.push(working_range);
  merged_ranges.extend(
    ranges
      .iter()
      .enumerate()
      .filter(|(i, _)| *i != left && *i != right && !exclude.contains(i))
      .map(|(_, r)| *r),
  );

  merged_ranges
}

fn merge_coord_range(left: Coord, right: Coord) -> Coord {
  let x = if left.0 < right.0 { left.0 } else { right.0 };
  let y = if left.1 > right.1 { left.1 } else { right.1 };

  (x, y)
}

#[derive(Debug, PartialEq)]
enum Overlap {
  Left,
  Right,
  Contains,
  Contained,
}

fn check_overlap(
  origin: (isize, isize),
  target: (isize, isize),
) -> Option<Overlap> {
  if origin.0 <= target.0 && origin.1 >= target.1 {
    Some(Overlap::Contains)
  } else if target.0 <= origin.0 && target.1 >= origin.1 {
    Some(Overlap::Contained)
  } else if origin.1 >= target.0 && origin.0 < target.0 {
    Some(Overlap::Right)
  } else if origin.0 <= target.1 && origin.1 > target.1 {
    Some(Overlap::Left)
  } else {
    None
  }
}
