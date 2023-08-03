use std::{
  cmp::Ordering,
  collections::{HashMap, HashSet},
  ops::Deref,
  rc::Rc,
};

use crate::common::prelude::*;


#[allow(non_snake_case)]
fn measure_2D_array_members_span<T>(target: &[Vec<T>], span_type: T) -> usize
where
  T: PartialEq<T>,
{
  target.iter().fold(0, |acc, row| {
    let row_value = row.iter().fold(([0; 2], 0), |mut acc_inner, cur_inner| {
      if *cur_inner == span_type {
        acc_inner.0[acc_inner.1] += 1;
      } else if acc_inner.1 == 0 {
        acc_inner.1 = 1;
      }
      acc_inner
    });

    let Some(row_min) = row_value.0.iter().min() else {
      return acc;
    };
    if *row_min > 0 {
      // Include only non-zero counts in the minimum calculation
      (*row_min).min(acc)
    } else {
      acc
    }
  })
}

fn get_ring_values<'a>(
  heading: Heading,
  face_values: &'a [Vec<Legend>],
  location: Coord,
) -> Box<dyn Iterator<Item = Legend> + 'a> {
  match heading {
    Heading::Right => {
      let y = location.y;
      Box::new(face_values[y].iter().cloned())
    }
    Heading::Left => {
      let y = location.y;
      Box::new(face_values[y].iter().rev().cloned())
    }
    Heading::Down => {
      let x = location.x;
      Box::new(face_values.iter().map(move |row| &row[x]).cloned())
    }
    Heading::Up => {
      let x = location.x;
      Box::new(face_values.iter().map(move |row| &row[x]).rev().cloned())
    }
  }
}


#[derive(Clone)]
pub struct Mapping(Vec<Vec<Legend>>);

impl Mapping {
  pub fn get_board_dimension(board: &[Vec<Legend>]) -> usize {
    let row_dimension = measure_2D_array_members_span(board, Legend::Space);
    let col_dimension = {
      let transposed_board: Vec<Vec<Legend>> = (0..board[0].len())
        .map(|c| (0..board.len()).map(|r| board[r][c].to_owned()).collect())
        .collect();

      measure_2D_array_members_span(&transposed_board, Legend::Space)
    };

    row_dimension.min(col_dimension)
  }

  fn get_mapping_location_of_index(&self, index: usize) -> Option<Coord> {
    let mut cnt = 0;
    for (y, row) in self.0.iter().enumerate() {
      for (x, item) in row.iter().enumerate() {
        if matches!(item, Legend::Open) {
          cnt += 1;
        }
        if (cnt - 1) as usize == index {
          return Some(Coord::from((x, y)));
        }
      }
    }

    None
  }

  fn get_index_of_mapping_location(&self, location: Coord) -> Option<usize> {
    let mut cnt = 0;
    for (y, row) in self.0.iter().enumerate() {
      for (x, item) in row.iter().enumerate() {
        if matches!(item, Legend::Open) {
          cnt += 1;
        }
        if Coord::from((x, y)) == location {
          return Some(cnt - 1);
        }
      }
    }

    None
  }

  fn get_2d_adjacencies(&self, coord: Coord) -> [Option<usize>; 4] {
    let mut adjacencies: [Option<usize>; 4] = [None; 4];

    if coord.x > 0 && matches!(self.0[coord.y][coord.x - 1], Legend::Space) {
      adjacencies[2] =
        self.get_index_of_mapping_location(Coord::from((coord.x - 1, coord.y)));
    }
    if coord.y > 0 && matches!(self.0[coord.y - 1][coord.x], Legend::Space) {
      adjacencies[3] =
        self.get_index_of_mapping_location(Coord::from((coord.x, coord.y - 1)));
    }
    if coord.x < self.0[0].len() - 1
      && matches!(self.0[coord.y][coord.x + 1], Legend::Space)
    {
      adjacencies[0] =
        self.get_index_of_mapping_location(Coord::from((coord.x + 1, coord.y)));
    }
    if coord.y < self.0.len() - 1
      && matches!(self.0[coord.y + 1][coord.x], Legend::Space)
    {
      adjacencies[1] =
        self.get_index_of_mapping_location(Coord::from((coord.x, coord.y + 1)));
    }

    adjacencies
  }
}

impl From<&[Vec<Legend>]> for Mapping {
  fn from(value: &[Vec<Legend>]) -> Self {
    let dim = Mapping::get_board_dimension(value);

    let max_row_sample =
      value
        .iter()
        .enumerate()
        .fold((usize::MAX, 0), |acc, (i, cur)| {
          let len = cur.len();
          if acc.1 < len {
            (i, len)
          } else {
            acc
          }
        });

    let mut vector: Vec<Vec<Legend>> = Vec::with_capacity(value.len() % dim);
    for _ in 0..value.len() % dim {
      vector.push(Vec::with_capacity(max_row_sample.1 % dim))
    }

    Mapping(vector)
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum CubeFace {
  #[default]
  Front,
  Back,
  Left,
  Right,
  Up,
  Down,
}

impl CubeFace {
  fn from_cardinal_index(index: usize) -> CubeFace {
    match index {
      0 => CubeFace::Right,
      1 => CubeFace::Down,
      2 => CubeFace::Left,
      3 => CubeFace::Up,
      _ => unreachable!(),
    }
  }
}

// Struct to represent a face of the cube
#[derive(Debug, Clone)]
pub struct Face {
  face_type: CubeFace,
  position: Coord, // Position in the 2D mapping
}

// Define a cube struct to hold the faces
#[derive(Default)]
pub struct Cube {
  pub faces: Vec<Face>,
  pub face_indices: HashMap<CubeFace, usize>, // Mapping from CubeFace to indices in Cube faces
  pub mapping: Option<Mapping>,
  pub board: Option<Board>,
  pub dim: Option<usize>,
}

impl Cube {
  fn new() -> Cube {
    Cube {
      faces: Vec::with_capacity(6),
      face_indices: HashMap::with_capacity(6),
      ..Default::default()
    }
  }

  fn add_face(&mut self, face: Face) {
    let index = self.faces.len();
    self.faces.push(face);
    self.face_indices.insert(self.faces[index].face_type, index);
  }

  fn get_neighbors(&mut self, face: &Face) -> [Option<usize>; 4] {
    let original_front = self.face_indices[&CubeFace::Front];
    self.rotate_to_front(face.face_type);

    let mapping = self.mapping.as_ref();
    let neighbors = [
      self.face_indices.get(&CubeFace::Right).and_then(|&index| {
        let position = self.faces[index].position.clone();
        mapping?.get_index_of_mapping_location(position)
      }),
      self.face_indices.get(&CubeFace::Down).and_then(|&index| {
        let position = self.faces[index].position.clone();
        mapping?.get_index_of_mapping_location(position)
      }),
      self.face_indices.get(&CubeFace::Left).and_then(|&index| {
        let position = self.faces[index].position.clone();
        mapping?.get_index_of_mapping_location(position)
      }),
      self.face_indices.get(&CubeFace::Up).and_then(|&index| {
        let position = self.faces[index].position.clone();
        mapping?.get_index_of_mapping_location(position)
      }),
    ];

    self.rotate_to_front(self.faces[original_front].face_type);

    neighbors
  }

  // Function to perform a 2D clockwise rotation around the x-axis (Front, Left, Back, Right)
  fn rotate_x(&mut self) {
    let front_index = self.face_indices[&CubeFace::Front];
    let left_index = self.face_indices[&CubeFace::Left];
    let back_index = self.face_indices[&CubeFace::Back];
    let right_index = self.face_indices[&CubeFace::Right];

    self.faces[front_index].face_type = CubeFace::Left;
    self.faces[left_index].face_type = CubeFace::Back;
    self.faces[back_index].face_type = CubeFace::Right;
    self.faces[right_index].face_type = CubeFace::Front;

    self.face_indices.insert(CubeFace::Left, front_index);
    self.face_indices.insert(CubeFace::Back, left_index);
    self.face_indices.insert(CubeFace::Right, back_index);
    self.face_indices.insert(CubeFace::Front, right_index);
  }

  // Function to perform a 2D clockwise rotation around the y-axis (Front, Up, Back, Down)
  fn rotate_y(&mut self) {
    let front_index = self.face_indices[&CubeFace::Front];
    let up_index = self.face_indices[&CubeFace::Up];
    let back_index = self.face_indices[&CubeFace::Back];
    let down_index = self.face_indices[&CubeFace::Down];

    self.faces[front_index].face_type = CubeFace::Up;
    self.faces[up_index].face_type = CubeFace::Back;
    self.faces[back_index].face_type = CubeFace::Down;
    self.faces[down_index].face_type = CubeFace::Front;

    self.face_indices.insert(CubeFace::Up, front_index);
    self.face_indices.insert(CubeFace::Back, up_index);
    self.face_indices.insert(CubeFace::Down, back_index);
    self.face_indices.insert(CubeFace::Front, down_index);
  }

  fn rotate_to_front(&mut self, target_face: CubeFace) {
    let num_rotations = match target_face {
      CubeFace::Front => 0,
      CubeFace::Back => 2,
      CubeFace::Left | CubeFace::Up => 3,
      CubeFace::Right | CubeFace::Down => 1,
    };

    for _ in 0..num_rotations {
      match target_face {
        CubeFace::Up | CubeFace::Down => self.rotate_y(),
        _ => self.rotate_x(),
      }
    }
  }

  fn add_to(&mut self, target_face: CubeFace, position: Coord) -> Option<()> {
    if self.face_indices.contains_key(&target_face)
      || !self.face_indices.contains_key(&CubeFace::Front)
    {
      return None;
    }

    let original_front = self.face_indices[&CubeFace::Front];
    self.rotate_to_front(target_face);
    self.add_face(Face { face_type: CubeFace::Front, position });
    self.rotate_to_front(self.faces[original_front].face_type);

    return Some(());
  }

  fn determine_rotation(&self, target: CubeFace) -> Option<isize> {
    let front = &self
      .faces
      .get(*self.face_indices.get(&CubeFace::Front)?)?
      .position;
    let target = &self.faces.get(*self.face_indices.get(&target)?)?.position;
    let mapping = self.mapping.as_ref()?;

    let mut visited: HashSet<Coord> = HashSet::new();
    let mut path: Vec<isize> = Vec::new(); // these indices are heading scores and are rotationally correct
    let mut stack = vec![(Rc::new(front.to_owned()), Vec::<isize>::new())];
    while let Some((direction_rc, current_path)) = stack.pop() {
      if !path.is_empty() {
        break;
      }
      let dir = direction_rc.deref().clone();
      visited.insert(dir.clone());
      mapping.get_2d_adjacencies(dir.clone()).iter().enumerate().for_each(|(i, opt_index)| {
        let Some(index) = opt_index else {
          return;
        };
        let Some(position) = mapping.get_mapping_location_of_index(*index) else {
          return;
        };
        if *target == position {
          path = [current_path.clone(), vec![i as isize]].concat();
          return;
        }
        if !visited.contains(&position) {
          let mut new_path = current_path.clone();
          new_path.push(i as isize);
          stack.push((Rc::new(position.clone()), new_path));}
      });
    }

    let mut rotation = 0;
    let mut turns = 0;
    if let Some(mut prev_heading) = path.first() {
      for heading in path.iter().skip(1) {
        if heading != prev_heading {
          turns += 1;
        }
        let mut current_rotation = heading - prev_heading;
        if current_rotation.abs() == 3 {
          current_rotation = -current_rotation.signum();
        }
        rotation += current_rotation;
        prev_heading = heading;
      }
    }
    if turns % 2 == 1 {
      rotation += 2;
    }

    Some(rotation)
  }

  fn get_cube_face_values(
    &self,
    cube_face: CubeFace,
  ) -> Option<Vec<Vec<Legend>>> {
    if let Some(board) = &self.board {
      let dim = self.dim?;
      let face_index = self.face_indices.get(&cube_face)?;
      let face = &self.faces[*face_index];
      let (x, y) = (face.position.x * dim, face.position.y * dim);
      let mut face_values: Vec<Vec<Legend>> = board.get_ref()[y..(y + dim)]
        .iter()
        .map(|row| row[x..(x + dim)].to_vec())
        .collect();

      let rotation = self.determine_rotation(cube_face)?;
      if rotation != 0 {
        if rotation > 0 {
          // Rotate clockwise
          for _ in 0..rotation {
            face_values.rotate_right(1);
          }
        } else {
          for _ in 0..(-rotation) {
            face_values.rotate_left(1);
          }
        }
      }

      Some(face_values)
    } else {
      None
    }
  }

  fn get_cube_face_ring(
    &mut self,
    cube_face: CubeFace,
    location: Coord,
    heading: Heading,
  ) -> Option<Vec<Legend>> {
    let opposites = match heading {
      Heading::Left | Heading::Right => [CubeFace::Down, CubeFace::Up],
      Heading::Up | Heading::Down => [CubeFace::Left, CubeFace::Right],
    };

    let original_front = self.face_indices[&CubeFace::Front];
    self.rotate_to_front(cube_face);

    let cube_faces: Vec<CubeFace> = self
      .faces
      .iter()
      .filter_map(|face| {
        if !opposites.contains(&face.face_type) {
          Some(face.face_type.to_owned())
        } else {
          None
        }
      })
      .collect();

    self.rotate_to_front(self.faces[original_front].face_type);

    let mut ring_data: Vec<Legend> = Vec::new();
    let dim = Mapping::get_board_dimension(self.board.clone()?.get_ref());
    let skip_count = match heading {
      Heading::Left => dim - location.x,
      Heading::Right => location.x,
      Heading::Up => dim - location.y,
      Heading::Down => location.y,
    };
    for face in cube_faces {
      let face_values = self.get_cube_face_values(face)?;
      ring_data.extend(get_ring_values(
        heading.to_owned(),
        &face_values,
        location.to_owned(),
      ));
    }
    ring_data = ring_data
      .into_iter()
      .cycle()
      .skip(skip_count)
      .take(self.dim?)
      .collect::<Vec<_>>();

    Some(ring_data)
  }
}

impl From<Mapping> for Cube {
  fn from(mapping: Mapping) -> Self {
    let mut cube = Cube::new();
    cube.mapping = Some(mapping.clone());

    let rows = mapping.0.len();
    let cols = mapping.0[0].len();
    let mut front_face_option: Option<(usize, usize)> = None;
    for r in 0..rows {
      for c in 0..cols {
        if mapping.0[r][c] == Legend::Open {
          front_face_option = Some((r, c));
          break;
        }
      }
      if front_face_option.is_some() {
        break;
      }
    }
    let Some(location) = front_face_option else {
      return cube;
    };
    let front_face_location = Coord::from(location);
    let position = front_face_location.clone();
    cube.add_face(Face { face_type: CubeFace::Front, position });

    let mut stack: Vec<[Option<usize>; 4]> = Vec::new();
    let neighbors = mapping.get_2d_adjacencies(front_face_location.clone());
    stack.push(neighbors);

    let mut visited: Vec<Coord> = Vec::with_capacity(6);
    visited.push(front_face_location);
    while let Some(adjacencies) = stack.pop() {
      for i in 0..4 {
        if let Some(index) = adjacencies[i] {
          let Some(position) = mapping.get_mapping_location_of_index(index) else {
            return cube;
          };
          if !visited.contains(&position) {
            cube.add_to(CubeFace::from_cardinal_index(i), position.to_owned());
            stack.push(mapping.get_2d_adjacencies(position.to_owned()));
            visited.push(position);
          }
        }
      }
    }

    cube
  }
}
