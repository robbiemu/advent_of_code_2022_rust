use regex::Regex;
use std::collections::HashSet;


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

#[derive(Debug, Clone, Copy)]
pub struct Move {
  pub direction: Directions,
  pub noncardinal_ternary: Option<Directions>,
  pub distance: u8,
}

pub fn factory_move(record: String) -> Move {
  let re: Regex = Regex::new(r"([UDLR]) (\d+)").unwrap();

  if let Some(captures) = re.captures(&record) {
    let dir_str = captures[1].parse::<String>().unwrap();
    let distance = captures[2].parse::<u8>().unwrap();
    let direction = Directions::from(&dir_str);

    Move { direction, distance, noncardinal_ternary: None }
  } else {
    panic!("invalid format for line:\n {}", record);
  }
}

pub fn get_tail_positions(
  moves: Vec<Move>,
  positions: usize,
) -> HashSet<(i32, i32)> {
  let mut knots: Vec<(i32, i32)> = vec![(0, 0); positions];

  let mut visited: HashSet<(i32, i32)> = HashSet::new();
  visited.insert(
    knots
      .last()
      .cloned()
      .unwrap_or_else(|| panic!("unreachable")),
  );

  for mv in moves {
    for _ in 0..mv.distance {
      #[allow(unreachable_patterns)]
      match mv.direction {
        Directions::Up => knots[0].1 -= 1,
        Directions::Down => knots[0].1 += 1,
        Directions::Left => knots[0].0 -= 1,
        Directions::Right => knots[0].0 += 1,
        _ => {
          println!("Invalid direction");
          unreachable!();
        }
      }

      for position in 0..positions - 1 {
        let diff_x = knots[position].0 - knots[position + 1].0;
        let diff_y = knots[position].1 - knots[position + 1].1;

        if diff_x.abs() > 1 || diff_y.abs() > 1 {
          knots[position + 1].0 += diff_x.signum();
          knots[position + 1].1 += diff_y.signum();
        }

        visited.insert(
          knots
            .last()
            .cloned()
            .unwrap_or_else(|| panic!("unreachable")),
        );
      }
    }
  }

  visited
}
