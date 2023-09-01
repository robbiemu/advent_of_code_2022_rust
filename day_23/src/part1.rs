use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod common;
use common::*;

const DATA: &str = include_str!("../army_of_elves.txt");
const STEPS: usize = 10;

type Map = Rc<RefCell<Vec<Vec<char>>>>;
type Elves = Rc<RefCell<Vec<Coord>>>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CardinalDirection {
  North = 0,
  South = 1,
  West = 2,
  East = 3,
}

impl CardinalDirection {
  fn get_steps(
    step: CardinalDirection,
  ) -> impl Iterator<Item = CardinalDirection> {
    (0..4).map(move |i| CardinalDirection::from((step as usize + i) % 4))
  }
}

impl From<usize> for CardinalDirection {
  fn from(value: usize) -> Self {
    match value % 4 {
      0 => CardinalDirection::North,
      1 => CardinalDirection::South,
      2 => CardinalDirection::West,
      3 => CardinalDirection::East,
      _ => unreachable!(),
    }
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Coord {
  x: usize,
  y: usize,
}

impl Coord {
  fn propose(&self, step: CardinalDirection, elves: Elves) -> Option<Coord> {
    if !self.is_indicated_to_move(elves.clone()) {
      return None;
    }
    for cardinal_direction in CardinalDirection::get_steps(step) {
      if self.is_direction_clear(cardinal_direction, elves.clone()) {
        return Some(self.get_coord_at(cardinal_direction));
      }
    }
    None
  }

  fn is_indicated_to_move(&self, elves: Elves) -> bool {
    let elves_inner = elves.borrow();
    let coords = [
      Coord { x: self.x, y: self.y + 1 },
      Coord { x: self.x, y: self.y - 1 },
      Coord { x: self.x - 1, y: self.y - 1 },
      Coord { x: self.x - 1, y: self.y },
      Coord { x: self.x - 1, y: self.y + 1 },
      Coord { x: self.x + 1, y: self.y - 1 },
      Coord { x: self.x + 1, y: self.y },
      Coord { x: self.x + 1, y: self.y + 1 },
    ];

    elves_inner.iter().any(|e| coords.contains(e))
  }

  fn is_direction_clear(
    &self,
    cardinal_direction: CardinalDirection,
    elves: Elves,
  ) -> bool {
    let elves_inner = elves.borrow();
    match cardinal_direction {
      CardinalDirection::North => {
        let coords = [
          Coord { x: self.x - 1, y: self.y - 1 },
          Coord { x: self.x, y: self.y - 1 },
          Coord { x: self.x + 1, y: self.y - 1 },
        ];
        !elves_inner.iter().any(|e| coords.contains(e))
      }
      CardinalDirection::South => {
        let coords = [
          Coord { x: self.x - 1, y: self.y + 1 },
          Coord { x: self.x, y: self.y + 1 },
          Coord { x: self.x + 1, y: self.y + 1 },
        ];
        !elves_inner.iter().any(|e| coords.contains(e))
      }
      CardinalDirection::West => {
        let coords = [
          Coord { x: self.x - 1, y: self.y - 1 },
          Coord { x: self.x - 1, y: self.y },
          Coord { x: self.x - 1, y: self.y + 1 },
        ];
        !elves_inner.iter().any(|e| coords.contains(e))
      }
      CardinalDirection::East => {
        let coords = [
          Coord { x: self.x + 1, y: self.y - 1 },
          Coord { x: self.x + 1, y: self.y },
          Coord { x: self.x + 1, y: self.y + 1 },
        ];
        !elves_inner.iter().any(|e| coords.contains(e))
      }
    }
  }

  fn get_coord_at(&self, cardinal_direction: CardinalDirection) -> Coord {
    match cardinal_direction {
      CardinalDirection::North => Coord { x: self.x, y: self.y - 1 },
      CardinalDirection::South => Coord { x: self.x, y: self.y + 1 },
      CardinalDirection::West => Coord { x: self.x - 1, y: self.y },
      CardinalDirection::East => Coord { x: self.x + 1, y: self.y },
    }
  }
}

fn main() {
  let (map, elves) = extract();
  transform(map.clone(), elves.clone());
  readout(map.clone(), elves.clone());
}

fn extract() -> (Map, Elves) {
  let map = DATA
    .split('\n')
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();
  let elves = map
    .iter()
    .enumerate()
    .flat_map(|(y, row)| {
      row.iter().enumerate().filter_map(move |(x, &c)| {
        if c == '#' {
          Some(Coord { x, y })
        } else {
          None
        }
      })
    })
    .collect();
  (Rc::new(RefCell::new(map)), Rc::new(RefCell::new(elves)))
}

fn transform(map: Map, elves: Elves) {
  readout(map.clone(), elves.clone());
  for i in 0..STEPS {
    conditionally_expand_map(map.clone(), elves.clone());
    let propositions: HashMap<Coord, Vec<Coord>> =
      elves.borrow().iter().fold(HashMap::new(), |mut acc, elf| {
        if let Some(coord) =
          elf.propose(CardinalDirection::from(i), elves.clone())
        {
          if !acc.contains_key(&coord) {
            acc.insert(coord.clone(), vec![]);
          }
          acc.get_mut(&coord).unwrap().push(elf.clone());
        }
        acc
      });
    propositions.iter().for_each(|(destination, pedestrians)| {
      if pedestrians.len() == 1 {
        let peoton = pedestrians.last().unwrap();
        let index = elves.borrow().iter().position(|e| e == peoton).unwrap();
        elves.borrow_mut()[index] = destination.to_owned();
      }
    });
    readout(map.clone(), elves.clone());
  }
}

fn conditionally_expand_map(map: Map, elves: Elves) {
  let mut elves_inner = elves.borrow_mut();

  let last_y = map.borrow().len() - 1;
  let last_x = map.borrow()[0].len() - 1;

  if elves_inner.iter().any(|e| e.y == 0) {
    extend_vector_up(map.clone(), 1);
    *elves_inner = elves_inner
      .iter()
      .map(|e| Coord { x: e.x, y: e.y + 1 })
      .collect();
  }
  if elves_inner.iter().any(|e| e.x == 0) {
    extend_vector_left(map.clone(), 1);
    *elves_inner = elves_inner
      .iter()
      .map(|e| Coord { x: e.x + 1, y: e.y })
      .collect();
  }
  if elves_inner.iter().any(|e| e.y == last_y) {
    extend_vector_down(map.clone(), 1);
  }
  if elves_inner.iter().any(|e| e.x == last_x) {
    extend_vector_right(map.clone(), 1);
  }
}

fn readout(map: Map, elves: Elves) {
  let mut min_x = usize::MAX;
  let mut min_y = usize::MAX;
  let mut max_x = 0;
  let mut max_y = 0;
  // Find the bounding box of elves
  {
    let elves_inner = elves.borrow();
    for elf in elves_inner.iter() {
      min_x = min_x.min(elf.x);
      min_y = min_y.min(elf.y);
      max_x = max_x.max(elf.x);
      max_y = max_y.max(elf.y);
    }
  }
  let mut map_inner = map.borrow_mut();
  if max_y < map_inner.len() - 1 {
    for y in (max_y + 1..map_inner.len()).rev() {
      map_inner.remove(y);
    }
  }
  if min_y > 0 {
    for y in (0..=min_y - 1).rev() {
      map_inner.remove(y);
    }
  }
  let span_x = map_inner[0].len() - 1;
  for row in map_inner.iter_mut() {
    if max_x < span_x {
      for x in (max_x + 1..row.len()).rev() {
        row.remove(x);
      }
    }
    if min_x > 0 {
      for x in (0..=min_x - 1).rev() {
        row.remove(x);
      }
    }
  }
  let mut elves_inner = elves.borrow_mut();
  for elf in elves_inner.iter_mut() {
    elf.x -= min_x;
    elf.y -= min_y;
  }
  let mut total_period_count = 0;
  map_inner.iter().enumerate().for_each(|(y, row)| {
    let representation = row
      .iter()
      .enumerate()
      .map(|(x, _)| {
        if elves_inner.contains(&Coord { x, y }) {
          &'#'
        } else {
          total_period_count += 1;
          &'.'
        }
      })
      .collect::<String>();
    println!("{}", representation)
  });
  println!("Total Period Count: {}", total_period_count);
}
