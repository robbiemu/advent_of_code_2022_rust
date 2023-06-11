use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]

pub enum Directions {
  Up,
  Down,
  Left,
  Right,
}

impl Directions {
  fn from(s: &str) -> Directions {
    match s {
      "U" => Directions::Up,
      "D" => Directions::Down,
      "L" => Directions::Left,
      "R" => Directions::Right,
      _ => unreachable!(),
    }
  }
}

#[derive(Debug)]
pub struct Move {
  direction: Directions,
  distance: u8,
}

pub fn factory_move(record: String) -> Move {
  let re: Regex = Regex::new(r"([UDLR]) (\d+)").unwrap();

  if let Some(captures) = re.captures(&record) {
    let dir_str = captures[1].parse::<String>().unwrap();
    let distance = captures[2].parse::<u8>().unwrap();
    let direction = Directions::from(&dir_str);

    Move {
      direction,
      distance,
    }
  } else {
    panic!("invalid format for line:\n {}", record);
  }
}

pub fn get_tail_positions(moves: &[Move]) -> HashSet<(i32, i32)> {
  let mut y: i32 = 0;
  let mut x: i32 = 0;
  let mut w: i32 = 0;
  let mut v: i32 = 0;
  let mut tail_positions = HashSet::from([(w, v)]);

  for mv in moves {
    println!("{:?}", mv);
    for _step in 0..mv.distance {
      let prev_y = y;
      let prev_x = x;

      #[allow(unreachable_patterns)] // future-proof for additional variants
      match mv.direction {
        Directions::Down => {
          y += 1;
        }
        Directions::Right => {
          x += 1;
        }
        Directions::Up => {
          y -= 1;
        }
        Directions::Left => {
          x -= 1;
        }
        _ => unreachable!(),
      };

      match (w, v) {
        (_, _) if (w, v) == (prev_y, prev_x) || (w, v) == (y, x) => {
          println!(
            "{:?} : {:?} (from {:?}) ",
            (w, v),
            (y, x),
            (prev_y, prev_x)
          );
          continue;
        }
        (_, _)
          if has_cardinal_adjacency((w, v), (y, x))
            || has_noncardinal_adjacency((w, v), (y, x)) =>
        {
          println!(
            "{:?} : {:?} (from {:?}) ",
            (w, v),
            (y, x),
            (prev_y, prev_x)
          );
          continue;
        }
        (_, _) if has_distant_cardinal_adjacency((w, v), (y, x)) => {
          println!(
            "cardinal {:?} : {:?} (from {:?}) ",
            (w, v),
            (y, x),
            (prev_y, prev_x)
          );

          (w, v) = (prev_y, prev_x);
        }
        (_, _) if has_distant_noncardinal_adjacency((w, v), (y, x)) => {
          println!(
            "noncardinal {:?} : {:?} (from {:?}) ",
            (w, v),
            (y, x),
            (prev_y, prev_x)
          );

          (w, v) = (prev_y, prev_x);
        }
        _ => {
          panic!(
            "error: {:?} : {:?} (from {:?}) ",
            (w, v),
            (y, x),
            (prev_y, prev_x)
          );
        }
      }

      tail_positions.insert((w, v));
    }
  }

  tail_positions
}

fn has_cardinal_adjacency(from: (i32, i32), to: (i32, i32)) -> bool {
  let dy = (from.0 - to.0).abs();
  let dx = (from.1 - to.1).abs();
  (dy == 0 && dx == 1) || (dy == 1 && dx == 0)
}

fn has_noncardinal_adjacency(from: (i32, i32), to: (i32, i32)) -> bool {
  let dy = (from.0 - to.0).abs();
  let dx = (from.1 - to.1).abs();
  dy == 1 && dx == 1
}

fn has_distant_cardinal_adjacency(from: (i32, i32), to: (i32, i32)) -> bool {
  let (x1, y1) = from;
  let (x2, y2) = to;

  let distance = (x2 - x1).abs() + (y2 - y1).abs();

  distance == 2
    && ((x2 - x1).abs() == 2 && y2 == y1 || (y2 - y1).abs() == 2 && x2 == x1)
}

fn has_distant_noncardinal_adjacency(from: (i32, i32), to: (i32, i32)) -> bool {
  let (x1, y1) = from;
  let (x2, y2) = to;

  let distance = (x2 - x1).abs() + (y2 - y1).abs();

  distance == 3
    && !((x2 == x1 + 3 || x2 == x1 - 3) && y2 == y1)
    && !(x2 == x1 && (y2 == y1 + 3 || y2 == y1 - 3))
}
