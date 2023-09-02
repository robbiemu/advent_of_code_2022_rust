use std::{collections::HashSet, rc::Rc};


const DATA: &str = include_str!("../input.txt");
const STORM_SYMBOLS: [char; 4] = ['<', '>', '^', 'v'];
const NEIGHBORS: [[isize; 2]; 5] = [[1, 0], [0, 1], [-1, 0], [0, -1], [0, 0]];
const ESCAPE_HATCH: usize = 10_000;
#[cfg(not(feature = "part2"))]
const REPEATED_SEARCH: bool = false;
#[cfg(feature = "part2")]
const REPEATED_SEARCH: bool = true;

type ProblemDescription = (Coord, Coord, Coord, Rc<Vec<Storm>>);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
  x: isize,
  y: isize,
}

#[derive(Debug)]
struct Storm {
  direction: char,
  coord: Coord,
}

impl Storm {
  fn offset(&self, t: isize, terminus: &Coord) -> Result<Coord, String> {
    match self.direction {
      '>' => {
        let x = ((self.coord.x - 1 + t) % terminus.x) + 1;

        Ok(Coord { x, y: self.coord.y })
      }
      '<' => {
        let x = (self.coord.x - 1 - t).rem_euclid(terminus.x) + 1;

        Ok(Coord { x, y: self.coord.y })
      }
      'v' => {
        let y = ((self.coord.y - 1 + t) % terminus.y) + 1;

        Ok(Coord { x: self.coord.x, y })
      }
      '^' => {
        let y = (self.coord.y - 1 - t).rem_euclid(terminus.y) + 1;

        Ok(Coord { x: self.coord.x, y })
      }
      _ => Err("Unimplemented direction at Storm.offset()".to_string()),
    }
  }
}

fn main() -> Result<(), String> {
  let (start, end, terminus, storms) = extract()?;
  let time = transform(&start, &end, &terminus, storms.clone())?;

  println!("time {time}");

  Ok(())
}

fn extract() -> Result<ProblemDescription, String> {
  let input: Vec<Vec<char>> = DATA
    .split('\n')
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();
  let last_row = input.len() - 1;
  let mut start = Coord::default();
  let mut end = Coord::default();
  let mut storms: Vec<Storm> = Vec::new();
  let terminus = Coord {
    x: input[0].len() as isize - 2,
    y: input.len() as isize - 2,
  };
  for (y, row) in input.iter().enumerate() {
    for (x, tile) in row.iter().enumerate() {
      if y == 0 {
        let x = row
          .iter()
          .position(|&c| c == '.')
          .ok_or("No starting element found.")?;
        start = Coord { x: x as isize, y: y as isize };
        continue;
      } else if y == last_row {
        let x = row
          .iter()
          .position(|&c| c == '.')
          .ok_or("No ending element found.")?;
        end = Coord { x: x as isize, y: y as isize };
        continue;
      }

      if STORM_SYMBOLS.contains(tile) {
        storms.push(Storm {
          direction: tile.to_owned(),
          coord: Coord { x: x as isize, y: y as isize },
        })
      }
    }
  }

  Ok((start, end, terminus, Rc::new(storms)))
}

fn transform(
  start: &Coord,
  end: &Coord,
  terminus: &Coord,
  storms: Rc<Vec<Storm>>,
) -> Result<usize, String> {
  if REPEATED_SEARCH {
    let mut t = search(start, end, terminus, storms.clone(), 0)?;
    t = search(end, start, terminus, storms.clone(), t)?;
    t = search(start, end, terminus, storms, t)?;

    Ok(t)
  } else {
    search(start, end, terminus, storms, 0)
  }
}

fn search(
  start: &Coord,
  end: &Coord,
  terminus: &Coord,
  storms: Rc<Vec<Storm>>,
  time: usize,
) -> Result<usize, String> {
  let mut q: HashSet<Coord> = HashSet::from_iter([*start]);
  let mut mapped_storms: Vec<Coord> = Vec::with_capacity(storms.len());
  for t in time + 1.. {
    mapped_storms.clear();
    for storm in storms.iter() {
      mapped_storms.push(storm.offset(t as isize, terminus)?);
    }
    q = q
      .iter()
      .flat_map(|coord| {
        NEIGHBORS.map(|offset| Coord {
          x: offset[0] + coord.x,
          y: offset[1] + coord.y,
        })
      })
      .filter(|coord| {
        (coord == end || coord == start)
          || (!mapped_storms.contains(coord)
            && coord.x > 0
            && coord.x <= terminus.x
            && coord.y > 0
            && coord.y <= terminus.y)
      })
      .collect();
    if q.contains(end) {
      return Ok(t);
    }
    if t == ESCAPE_HATCH {
      break;
    }
  }

  Err("No path to end".to_string())
}
