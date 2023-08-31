use regex::Regex;
use std::{
  borrow::Cow,
  collections::HashMap,
  fmt::{self, Display},
  ops::{Index, IndexMut},
};


const DATA: &str = include_str!("../input.txt");
const SIZE: usize = 50;

#[derive(Clone, Copy, Default, Debug)]
enum Tile {
  #[default]
  Space,
  Wall,
}

impl Tile {
  pub fn from_char(c: char) -> Result<Self, String> {
    match c {
      '.' => Ok(Tile::Space),
      '#' => Ok(Tile::Wall),
      _ => Err(format!("tile not implemented for character {c}")),
    }
  }
}

type CubeFace = [[Tile; SIZE]; SIZE];

#[derive(Clone, Copy, Debug)]
enum Heading {
  Right = 0,
  Down = 1,
  Left = 2,
  Up = 3,
}

impl Heading {
  pub fn headings<'a>() -> impl Iterator<Item = &'a Heading> {
    [Heading::Right, Heading::Down, Heading::Left, Heading::Up].iter()
  }

  pub fn relative_coordinate(&self, coord: Coord) -> Coord {
    match self {
      Heading::Right => Coord::from((coord.x + 1, coord.y)),
      Heading::Down => Coord::from((coord.x, coord.y + 1)),
      Heading::Left => Coord::from((coord.x - 1, coord.y)),
      Heading::Up => Coord::from((coord.x, coord.y - 1)),
    }
  }
}

impl From<usize> for Heading {
  fn from(value: usize) -> Self {
    match value {
      0 => Heading::Right,
      1 => Heading::Down,
      2 => Heading::Left,
      3 => Heading::Up,
      _ => unimplemented!(),
    }
  }
}

impl Index<Heading> for [Option<FacePositioning>] {
  type Output = Option<FacePositioning>;

  fn index(&self, heading: Heading) -> &Self::Output {
    &self[heading as usize]
  }
}

impl IndexMut<Heading> for [Option<FacePositioning>] {
  fn index_mut(&mut self, heading: Heading) -> &mut Self::Output {
    &mut self[heading as usize]
  }
}

impl Display for Heading {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let representation = match self {
      Heading::Right => ">",
      Heading::Down => "v",
      Heading::Left => "<",
      Heading::Up => "^",
    };

    write!(f, "{representation}")
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord {
  x: usize,
  y: usize,
}

impl Coord {
  pub fn from(tuple: (usize, usize)) -> Self {
    Coord { x: tuple.0, y: tuple.1 }
  }
}

impl Display for Coord {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "(x{},y{})", self.x, self.y)
  }
}

#[derive(Clone)]
struct Face {
  data: CubeFace,
  coord: Coord,
  side: [Option<FacePositioning>; 4],
}

impl fmt::Debug for Face {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "Face {{")?;
    writeln!(f, "    coord: {:?}", self.coord)?;

    writeln!(f, "    side: [")?;
    for (i, side) in self.side.iter().enumerate() {
      write!(f, "        ")?;
      match side {
        Some(positioning) => write!(f, "{:?}", positioning)?,
        None => write!(f, "None")?,
      }
      if i < self.side.len() - 1 {
        writeln!(f, ",")?;
      }
    }
    writeln!(f, "\n    ],")?;

    writeln!(f, "    data: [")?;
    for row in &self.data {
      write!(f, "        [")?;
      for (i, tile) in row.iter().enumerate() {
        write!(f, "{:?}", tile)?;
        if i < row.len() - 1 {
          write!(f, ", ")?;
        }
      }
      writeln!(f, "],")?;
    }
    writeln!(f, "    ],")?;
    write!(f, "}}")
  }
}

#[derive(Clone, Debug)]
struct FacePositioning {
  coord: Coord,
  rotation: isize,
}

#[derive(Clone)]
struct Position {
  coord: Coord,
  face: Coord,
  rotation: Heading,
}

fn main() -> Result<(), String> {
  let mut grid: Vec<&str> =
    DATA.split('\n').take_while(|l| !l.is_empty()).collect();
  let face_data = {
    let mut collection: Vec<Vec<Option<String>>> = vec![vec![None; 7]; 7];
    grid.as_slice().chunks(SIZE).enumerate().for_each(|(y, v)| {
      v.iter().for_each(|line| {
        for x in 0..line.len() / SIZE {
          let row = &line[x * SIZE..(x + 1) * SIZE].to_string();
          if !row.trim().is_empty() {
            if collection[y + 1][x + 1].is_none() {
              collection[y + 1][x + 1] = Some("".to_string())
            }
            if let Some(ref mut face) = &mut collection[y + 1][x + 1] {
              *face += row;
            }
          }
        }
      })
    });

    collection
  };

  let face_coords: [Coord; 6] = (0..7)
    .flat_map(|y| {
      (0..7)
        .filter_map(|x| {
          if face_data[y][x].is_some() {
            Some(Coord::from((x, y)))
          } else {
            None
          }
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>()
    .try_into()
    .map_err(|err| format!("Error building face_coords: {:?}", err))?;

  let mut faces: HashMap<Coord, Face> = face_coords.iter().try_fold(
    HashMap::new(),
    |mut acc: HashMap<Coord, Face>, cur| {
      let data: CubeFace = {
        let input: [Tile; SIZE * SIZE] = face_data[cur.y][cur.x]
          .as_ref()
          .unwrap()
          .chars()
          .map(|c| Tile::from_char(c).map_err(|e| e.to_string()))
          .collect::<Result<Vec<_>, _>>()?
          .try_into()
          .map_err(|_| {
            "Error converting characters to tiles, are rows of the same number \
             of characters?"
          })?;

        input
          .chunks(SIZE)
          .map(|chunk| {
            let mut row = [Tile::default(); SIZE];
            row.copy_from_slice(chunk);
            row
          })
          .collect::<Vec<_>>()
          .try_into()
          .map_err(|_| {
            "Error converting characters to tiles, are cube faces square?"
          })?
      };
      let coord = *cur;
      let side = [None, None, None, None];

      let face = Face { data, coord, side };
      acc.insert(coord, face);

      Ok::<HashMap<Coord, Face>, String>(acc)
    },
  )?;

  faces.iter_mut().for_each(|(coord, face)| {
    Heading::headings().for_each(|&heading| {
      let face_coord = heading.relative_coordinate(coord.to_owned());
      if face_coords.contains(&face_coord) {
        face.side[heading] =
          Some(FacePositioning { coord: face_coord, rotation: 0 });
      }
    });
  });

  loop {
    let faces_guard = faces.clone();
    let incomplete: Vec<&mut Face> = faces
      .values_mut()
      .filter(|face| face.side.iter().filter(|&s| s.is_some()).count() < 4)
      .collect();

    if incomplete.is_empty() {
      // All faces are completed, break the loop
      break;
    }

    for face in incomplete {
      let mut early_exit = false;
      Heading::headings().for_each(|&side| {
        if face.side[side].is_none() {
          // Look for neighbour on one of our attached sides that we can then rotate and attach
          for i in [1, -1] {
            let candidate_side =
              Heading::from((side as isize + i).rem_euclid(4) as usize);
            if let Some(candidate_positioning) =
              face.side[candidate_side].clone()
            {
              let neighbor_rotation = i + candidate_positioning.rotation;
              let candidate = &faces_guard[&candidate_positioning.coord];
              let neighbor_side = Heading::from(
                (side as isize + candidate_positioning.rotation).rem_euclid(4)
                  as usize,
              );
              if let Some(neighbor_positioning) =
                candidate.side[neighbor_side].clone()
              {
                let rotation =
                  (neighbor_positioning.rotation + neighbor_rotation) % 4;

                face.side[side] = Some(FacePositioning {
                  coord: neighbor_positioning.coord,
                  rotation,
                });

                early_exit = true;
                break;
              }
            }
          }
        }
      });
      if early_exit {
        break;
      }
    }
  }

  let mut face_x = std::usize::MAX;
  let mut face_y = std::usize::MAX;

  for &coord in &face_coords {
    if coord.y < face_y || (coord.y == face_y && coord.x < face_x) {
      face_x = coord.x;
      face_y = coord.y;
    }
  }

  let mut turtle = Position {
    face: Coord::from((face_x, face_y)),
    coord: Coord::from((0, 0)),
    rotation: Heading::Right,
  };

  //let moves: Vec<> = Vec::new();
  let tape = DATA
    .split('\n')
    .skip_while(|line| !line.is_empty())
    .nth(1)
    .ok_or("invalid tape of instructions")?;
  let re = Regex::new(r"(?P<instruction>\d+|[RL])").unwrap();
  let mut moves = Vec::new();
  for cap in re.captures_iter(tape) {
    moves.push(turtle.clone());
    if let Some(instr) = cap.name("instruction").map(|instr| instr.as_str()) {
      match instr {
        "L" => {
          turtle.rotation =
            Heading::from((turtle.rotation as isize - 1).rem_euclid(4) as usize)
        }
        "R" => {
          turtle.rotation =
            Heading::from((turtle.rotation as isize + 1).rem_euclid(4) as usize)
        }
        _ if instr.chars().all(|c| c.is_ascii_digit()) => {
          if let Ok(num) = instr.parse::<usize>() {
            turtle = move_count(turtle, num, &faces);
          } else {
            unreachable!()
          }
        }
        _ => {
          unimplemented!()
        }
      }
    }
  }
  moves.push(turtle.clone());

  moves.iter().for_each(|pos| {
    let row = grid[(pos.face.y - 1) * SIZE + pos.coord.y].to_string();
    let mut new_row = String::with_capacity(row.len());
    new_row.push_str(&row[..(pos.face.x - 1) * SIZE + pos.coord.x]);
    new_row.push_str(&pos.rotation.to_string());
    new_row.push_str(&row[(pos.face.x - 1) * SIZE + pos.coord.x + 1..]);
    grid[(pos.face.y - 1) * SIZE + pos.coord.y] =
      Box::leak(new_row.into_boxed_str());
  });
  grid.iter().for_each(|row| eprintln!("{row}"));

  let password = 1000 * ((turtle.face.y - 1) * SIZE + turtle.coord.y + 1)
    + 4 * ((turtle.face.x - 1) * SIZE + turtle.coord.x + 1)
    + turtle.rotation as usize;
  eprintln!("password {password}");

  Ok(())
}

fn step(p: &mut Position, faces: &HashMap<Coord, Face>) {
  let x = p.coord.x as isize + [1, 0, -1, 0][p.rotation as usize];
  let y = p.coord.y as isize + [0, 1, 0, -1][p.rotation as usize];
  let mut d = p.rotation;
  let mut face = p.face;
  let mut new_coord = Coord::from((x as usize, y as usize));

  if x < 0 || x >= SIZE as isize || y < 0 || y >= SIZE as isize {
    let next_positioning = faces[&p.face].side[p.rotation].clone().unwrap();
    let rot = next_positioning.rotation;
    face = next_positioning.coord;
    d = Heading::from((p.rotation as isize + rot).rem_euclid(4) as usize);
    let x = x.rem_euclid(SIZE as isize) as usize;
    let y = y.rem_euclid(SIZE as isize) as usize;
    let (x, y) = match rot {
      -3 | 1 => [
        (SIZE - 1 - y, 0),
        (SIZE - 1, x),
        (SIZE - 1 - y, SIZE - 1),
        (0, x),
      ][p.rotation as usize],
      -2 | 2 => (SIZE - 1 - x, SIZE - 1 - y),
      -1 | 3 => [
        (y, SIZE - 1),
        (0, SIZE - 1 - x),
        (y, 0),
        (SIZE - 1, SIZE - 1 - x),
      ][p.rotation as usize],
      _ => (x, y),
    };
    new_coord = Coord::from((x, y))
  }
  *p = Position { face, coord: new_coord, rotation: d };
}

fn move_count(
  mut pos: Position,
  mut num: usize,
  faces: &HashMap<Coord, Face>,
) -> Position {
  let mut last_good_pos = pos.clone();

  while num > 0 {
    step(&mut pos, faces);

    match faces[&pos.face].data[pos.coord.y][pos.coord.x] {
      Tile::Space => last_good_pos = pos.clone(),
      Tile::Wall => return last_good_pos,
    }

    num -= 1;
  }

  last_good_pos
}
