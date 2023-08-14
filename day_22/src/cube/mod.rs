use std::{
  borrow::ToOwned,
  collections::{HashMap, HashSet},
  fmt::Debug,
};

use crate::common::prelude::*;


#[allow(non_snake_case)]
fn measure_2D_array_members_span<T>(
  target: &[Vec<T>],
  span_type: &T,
) -> Option<usize>
where
  T: PartialEq<T>,
  T: Debug,
{
  let target_min = target.iter().fold(usize::MAX, |acc, row| {
    let row_value = row.iter().fold(([0; 2], 0), |mut acc_inner, cur_inner| {
      if cur_inner == span_type {
        acc_inner.0[acc_inner.1] += 1;
      } else if acc_inner.1 == 0 {
        acc_inner.1 = 1;
      }
      acc_inner
    });

    let Some(row_min) = row_value.0.iter().filter(|v| **v > 0).min() else {
      return acc;
    };
    if *row_min > 0 {
      // Include only non-zero counts in the minimum calculation
      (*row_min).min(acc)
    } else {
      acc
    }
  });

  if target_min != usize::MAX {
    Some(target_min)
  } else {
    None
  }
}

fn get_ring_values<'a>(
  heading: Heading,
  face_values: &'a [Vec<Legend>],
  location: Coord,
) -> Box<dyn Iterator<Item = Legend> + 'a> {
  match heading {
    Heading::Right => Box::new(face_values[location.y].iter().cloned()),
    Heading::Left => Box::new(face_values[location.y].iter().rev().cloned()),
    Heading::Down => {
      Box::new(face_values.iter().map(move |row| &row[location.x]).cloned())
    }
    Heading::Up => Box::new(
      face_values
        .iter()
        .map(move |row| &row[location.x])
        .rev()
        .cloned(),
    ),
  }
}

fn rotate_matrix<T: Clone + Default>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
  let rows = matrix.len();
  let cols = matrix[0].len();
  let mut rotated_matrix = vec![vec![Default::default(); rows]; cols];

  (0..rows).for_each(|i| {
    (0..cols).for_each(|j| {
      // rotated_matrix[j][rows - i - 1] = matrix[i][j].clone();
      rotated_matrix[cols - j - 1][i] = matrix[i][j].clone();
    });
  });

  rotated_matrix
}


#[derive(Clone, Debug)]
pub struct Mapping(Vec<Vec<Legend>>);

type FromErrorForMapping = String;
impl Mapping {
  pub fn from_2d_vector(
    value: &[Vec<Legend>],
  ) -> Result<Self, FromErrorForMapping> {
    let dim =
      Mapping::get_board_dimension(value).ok_or("board must have spaces")?;

    let max_row_sample = value.iter().fold(0, |acc, cur| {
      let len = cur.len();
      if acc < len {
        len
      } else {
        acc
      }
    });

    let mut vector: Vec<Vec<Legend>> = Vec::with_capacity(value.len() % dim);
    for y in 0..value.len() / dim {
      vector.push(vec![Legend::Space; max_row_sample / dim]);
      for x in 0..max_row_sample / dim {
        if !(value.len() - 1 < y * dim
          || value[y * dim].len() - 1 < x * dim
          || value[y * dim][x * dim] == Legend::Space)
        {
          vector[y][x] = Legend::Open;
        }
      }
    }

    Ok(Mapping(vector))
  }

  pub fn get_board_dimension(board: &[Vec<Legend>]) -> Option<usize> {
    let row_dimension = measure_2D_array_members_span(board, &Legend::Space)?;
    let col_dimension = {
      let transposed_board: Vec<Vec<Legend>> = (0..board[0].len())
        .map(|c| (0..board.len()).map(|r| board[r][c].to_owned()).collect())
        .collect();

      measure_2D_array_members_span(&transposed_board, &Legend::Space).unwrap()
    };

    Some(row_dimension.min(col_dimension))
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

    if coord.x > 0 && matches!(self.0[coord.y][coord.x - 1], Legend::Open) {
      adjacencies[2] =
        self.get_index_of_mapping_location(Coord::from((coord.x - 1, coord.y)));
    }
    if coord.y > 0 && matches!(self.0[coord.y - 1][coord.x], Legend::Open) {
      adjacencies[3] =
        self.get_index_of_mapping_location(Coord::from((coord.x, coord.y - 1)));
    }
    if coord.x < self.0[0].len() - 1
      && matches!(self.0[coord.y][coord.x + 1], Legend::Open)
    {
      adjacencies[0] =
        self.get_index_of_mapping_location(Coord::from((coord.x + 1, coord.y)));
    }
    if coord.y < self.0.len() - 1
      && matches!(self.0[coord.y + 1][coord.x], Legend::Open)
    {
      adjacencies[1] =
        self.get_index_of_mapping_location(Coord::from((coord.x, coord.y + 1)));
    }

    adjacencies
  }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CubeFace {
  #[default]
  Front,
  Right,
  Down,
  Back,
  Left,
  Up,
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

  fn get_turn(path: &Vec<CubeFace>) -> Option<usize> {
    let Some(mut raw_turns) = path.as_slice().windows(2).fold(Some(0), |acc, segment| {
      let Some(cumulative) = acc else {
        unreachable!();
      };
      let heading = get_orientation(segment);
      Some(cumulative + heading as isize)
    }) else {
      return None;
    };
    if path.contains(&CubeFace::Back)
      && (*path.last().unwrap() != CubeFace::Back
        || path[path.len() - 2] == CubeFace::Left
        || path[path.len() - 2] == CubeFace::Up)
    {
      // omg this was hard
      raw_turns += 2;
    }
    Some(raw_turns.rem_euclid(4) as usize)
  }

  fn opposite(cubeface: &CubeFace) -> CubeFace {
    match cubeface {
      CubeFace::Front => CubeFace::Back,
      CubeFace::Back => CubeFace::Front,
      CubeFace::Left => CubeFace::Right,
      CubeFace::Right => CubeFace::Left,
      CubeFace::Up => CubeFace::Down,
      CubeFace::Down => CubeFace::Up,
    }
  }

  fn get_natural_orientation(&self) -> Heading {
    match self {
      CubeFace::Down => Heading::Down,
      CubeFace::Up => Heading::Up,
      CubeFace::Right | CubeFace::Front => Heading::Right,
      CubeFace::Left | CubeFace::Back => Heading::Left,
    }
  }
}

fn get_orientation(segment: &[CubeFace]) -> usize {
  match segment {
    [CubeFace::Down, CubeFace::Left] => 1,
    [CubeFace::Down, CubeFace::Right] => 3,
    [CubeFace::Down, CubeFace::Down] => 2,
    [CubeFace::Up, CubeFace::Left] => 3,
    [CubeFace::Up, CubeFace::Right] => 1,
    [CubeFace::Up, CubeFace::Up] => 2,
    [CubeFace::Right, CubeFace::Up] => 3,
    [CubeFace::Right, CubeFace::Down] => 1,
    [CubeFace::Right, CubeFace::Right] => 2,
    [CubeFace::Left, CubeFace::Up] => 1,
    [CubeFace::Left, CubeFace::Down] => 3,
    [CubeFace::Left, CubeFace::Left] => 2,
    _ => 0,
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

  // Function to perform a 2D clockwise rotation around the x-axis (Front, Left, Back, Right)
  fn rotate_x(&mut self) {
    let front_index_opt = self
      .face_indices
      .get(&CubeFace::Front)
      .map(ToOwned::to_owned);
    let left_index_opt = self
      .face_indices
      .get(&CubeFace::Left)
      .map(ToOwned::to_owned);
    let back_index_opt = self
      .face_indices
      .get(&CubeFace::Back)
      .map(ToOwned::to_owned);
    let right_index_opt = self
      .face_indices
      .get(&CubeFace::Right)
      .map(ToOwned::to_owned);

    if let Some(front_index) = front_index_opt {
      self.face_indices.remove(&CubeFace::Front);
      self.faces[front_index].face_type = CubeFace::Left;
      self.face_indices.insert(CubeFace::Left, front_index);
    }
    if let Some(left_index) = left_index_opt {
      if front_index_opt.is_none() {
        self.face_indices.remove(&CubeFace::Left);
      }
      self.faces[left_index].face_type = CubeFace::Back;
      self.face_indices.insert(CubeFace::Back, left_index);
    }
    if let Some(back_index) = back_index_opt {
      if left_index_opt.is_none() {
        self.face_indices.remove(&CubeFace::Back);
      }
      self.faces[back_index].face_type = CubeFace::Right;
      self.face_indices.insert(CubeFace::Right, back_index);
    }
    if let Some(right_index) = right_index_opt {
      if back_index_opt.is_none() {
        self.face_indices.remove(&CubeFace::Right);
      }
      self.faces[right_index].face_type = CubeFace::Front;
      self.face_indices.insert(CubeFace::Front, right_index);
    }
  }

  // Function to perform a 2D clockwise rotation around the y-axis (Front, Up, Back, Down)
  fn rotate_y(&mut self) {
    let front_index_opt = self
      .face_indices
      .get(&CubeFace::Front)
      .map(ToOwned::to_owned);
    let up_index_opt =
      self.face_indices.get(&CubeFace::Up).map(ToOwned::to_owned);
    let back_index_opt = self
      .face_indices
      .get(&CubeFace::Back)
      .map(ToOwned::to_owned);
    let down_index_opt = self
      .face_indices
      .get(&CubeFace::Down)
      .map(ToOwned::to_owned);


    if let Some(front_index) = front_index_opt {
      self.face_indices.remove(&CubeFace::Front);
      self.faces[front_index].face_type = CubeFace::Up;
      self.face_indices.insert(CubeFace::Up, front_index);
    }
    if let Some(up_index) = up_index_opt {
      if front_index_opt.is_none() {
        self.face_indices.remove(&CubeFace::Up);
      }
      self.faces[up_index].face_type = CubeFace::Back;
      self.face_indices.insert(CubeFace::Back, up_index);
    }
    if let Some(back_index) = back_index_opt {
      if up_index_opt.is_none() {
        self.face_indices.remove(&CubeFace::Back);
      }
      self.faces[back_index].face_type = CubeFace::Down;
      self.face_indices.insert(CubeFace::Down, back_index);
    }
    if let Some(down_index) = down_index_opt {
      if back_index_opt.is_none() {
        self.face_indices.remove(&CubeFace::Down);
      }
      self.faces[down_index].face_type = CubeFace::Front;
      self.face_indices.insert(CubeFace::Front, down_index);
    }
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

    Some(())
  }

  fn determine_rotation(
    &self,
    target: CubeFace,
    heading: Heading,
  ) -> Option<(usize, bool)> {
    /* working along the ring, from when you get to the surface, how must the mapping surface rotate to work along the row/column? For example, working across rows from front, back will be encountered second, so if the oriention of back and front in mapping are the same, back still needs to be treated in reverse. If left can only be reached after an up/down movement, it will require a rotation.

    Heading is only used to determine horizontal or vertical progression, not which way it winds across the face.

    Result (usize, bool) is number of clockwise rotations and if it must be reversed. */
    let front = self.faces.get(*self.face_indices.get(&CubeFace::Front)?)?;
    let target = self.faces.get(*self.face_indices.get(&target)?)?;

    if front.face_type == target.face_type {
      return Some((0, false));
    }

    let mapping = self.mapping.as_ref()?;

    let mut visited: HashSet<Coord> = HashSet::new();
    let mut final_path: Vec<CubeFace> = Vec::new();
    let mut stack = vec![(front.to_owned(), Vec::new())];
    while let Some((face, mut path)) = stack.pop() {
      eprintln!("considering {:?} {}", face.face_type, face.position);
      visited.insert(face.position.to_owned());
      path.push(face.face_type.to_owned());
      for coord in mapping
        .get_2d_adjacencies(face.position.to_owned())
        .iter()
        .filter_map(|opt| *opt)
        .map(|i| mapping.get_mapping_location_of_index(i))
      {
        let next_face = self
          .faces
          .iter()
          .find(|f| Some(f.position.to_owned()) == coord)?;
        eprintln!(
          "-@{:?} {:?} {}",
          coord, next_face.face_type, next_face.position
        );
        if next_face.position == target.position {
          final_path = vec![path, vec![target.face_type]].concat();
          break;
        }
        if !visited.contains(&next_face.position) {
          stack.push((next_face.to_owned(), path.clone()));
        }
      }
      if !final_path.is_empty() {
        break;
      }
    }

    dbg!(&final_path);
    let opt = if matches!(heading, Heading::Up | Heading::Down) {
      let mut is_reversed = final_path.contains(&CubeFace::Back)
        && final_path
          .iter()
          .filter(|cf| matches!(cf, CubeFace::Left | CubeFace::Right))
          .count()
          % 2
          == 1;
      let mut v = (-(CubeFace::get_turn(&final_path).unwrap_or(0) as isize))
        .rem_euclid(4) as usize;
      if target.face_type == CubeFace::Back && is_reversed {
        is_reversed = false;
        v += 2;
      }
      match target.face_type {
        CubeFace::Back | CubeFace::Up | CubeFace::Down => {
          Some((v, is_reversed))
        }
        _ => None,
      }
    } else {
      let mut is_reversed = final_path.contains(&CubeFace::Back)
        && final_path
          .iter()
          .filter(|cf| matches!(cf, CubeFace::Up | CubeFace::Down))
          .count()
          % 2
          == 1;
      let mut h = (-(CubeFace::get_turn(&final_path).unwrap_or(0) as isize))
        .rem_euclid(4) as usize;
      if target.face_type == CubeFace::Back && is_reversed {
        is_reversed = false;
        h += 2;
      }
      match target.face_type {
        CubeFace::Back | CubeFace::Left | CubeFace::Right => {
          Some((h, is_reversed))
        }
        _ => None,
      }
    };

    opt
  }

  fn get_cube_face_values(
    &self,
    cube_face: CubeFace,
    heading: Heading,
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

      let facing = self.determine_rotation(cube_face, heading.clone());
      let is_natural_orientation = facing.is_some();
      let mut orientation = heading;
      let (rotation, is_reversed) = if is_natural_orientation {
        facing.unwrap()
      } else {
        orientation = Heading::from_score((orientation.get_score() + 1) % 4);
        self.determine_rotation(cube_face, orientation.to_owned())?
      };
      dbg!(rotation, is_reversed, x, y, dim, &orientation);
      for _ in 0..rotation {
        face_values = rotate_matrix(&face_values);
      }
      if is_reversed {
        match orientation {
          Heading::Down | Heading::Up => {
            if matches!(cube_face, CubeFace::Up) {
              for row in face_values.iter_mut() {
                row.reverse();
              }
            } else if cube_face != CubeFace::Down {
              face_values.reverse()
            }
          }
          Heading::Left | Heading::Right => {
            if matches!(cube_face, CubeFace::Left) {
              face_values.reverse();
            } else if cube_face != CubeFace::Right {
              for row in face_values.iter_mut() {
                row.reverse();
              }
            }
          }
        }
      }

      Some(face_values)
    } else {
      None
    }
  }

  fn get_cube_face_ring_indices(
    &mut self,
    cube_face: &CubeFace,
    heading: &Heading,
  ) -> Vec<usize> {
    let opposites = match heading {
      Heading::Left | Heading::Right => [CubeFace::Down, CubeFace::Up],
      Heading::Up | Heading::Down => [CubeFace::Left, CubeFace::Right],
    };

    let original_front = self.face_indices[&CubeFace::Front];
    self.rotate_to_front(cube_face.to_owned());

    let mut cube_face_indices: Vec<usize> = self
      .faces
      .iter()
      .enumerate()
      .filter_map(|(i, face)| {
        if !opposites.contains(&face.face_type) {
          Some(i)
        } else {
          None
        }
      })
      .collect();
    {
      #[allow(clippy::type_complexity)]
      let comparator: Box<dyn Fn(&usize, &usize) -> std::cmp::Ordering> =
        match heading {
          Heading::Right | Heading::Down => Box::new(|i: &usize, j: &usize| {
            self.faces[*i].face_type.cmp(&self.faces[*j].face_type)
          }),
          Heading::Left | Heading::Up => Box::new(|i: &usize, j: &usize| {
            self.faces[*j].face_type.cmp(&self.faces[*i].face_type)
          }),
        };
      cube_face_indices.sort_by(|i, j| comparator(i, j));
    }
    self.rotate_to_front(self.faces[original_front].face_type);
    cube_face_indices
      .iter()
      .cycle()
      .skip_while(|i| self.faces[**i].face_type != *cube_face)
      .take(4)
      .map(|i| i.to_owned())
      .collect()
  }

  fn get_cube_face_ring(
    &mut self,
    cube_face: CubeFace,
    location: Coord,
    heading: Heading,
  ) -> Option<Vec<Legend>> {
    let cube_face_indices =
      self.get_cube_face_ring_indices(&cube_face, &heading);
    let mut ring_data: Vec<Legend> = Vec::new();
    let dim = self.dim?;
    let skip_count = match heading {
      Heading::Left => dim - location.x,
      Heading::Right => location.x + 1,
      Heading::Up => dim - location.y,
      Heading::Down => location.y + 1,
    };

    eprintln!(
      "cube_faces order {:?}, skip count {skip_count}",
      cube_face_indices
        .iter()
        .map(|i| self.faces[*i].face_type)
        .collect::<Vec<_>>()
    );

    let has_rotation_offset = cube_face_indices
      .iter()
      .all(|j| self.faces[*j].face_type != CubeFace::Front);
    for i in cube_face_indices.clone() {
      let face = &self.faces[i];
      let mut face_values =
        self.get_cube_face_values(face.face_type, heading.to_owned())?;
      if has_rotation_offset {
        let clockwise_rotations =
          face.face_type.get_natural_orientation().get_score();
        dbg!(clockwise_rotations);
        for _ in 0..clockwise_rotations {
          face_values = rotate_matrix(&face_values);
        }
      }

      dbg!(&face.face_type, &face_values);

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
      .take(dim * 4)
      .collect::<Vec<_>>();

    Some(ring_data)
  }
}

impl From<&Mapping> for Cube {
  fn from(mapping: &Mapping) -> Self {
    let mut cube = Cube::new();
    cube.mapping = Some(mapping.clone());

    let rows = mapping.0.len();
    let cols = mapping.0[0].len();
    let mut front_face_option: Option<(usize, usize)> = None;
    for y in 0..rows {
      for x in 0..cols {
        if mapping.0[y][x] == Legend::Open {
          front_face_option = Some((x, y));
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

    let mut stack: Vec<(Vec<CubeFace>, [Option<usize>; 4])> = Vec::new();
    let neighbors = mapping.get_2d_adjacencies(front_face_location.clone());
    stack.push((vec![CubeFace::Front], neighbors));

    let mut visited: Vec<Coord> = Vec::with_capacity(6);
    visited.push(front_face_location.clone());
    while let Some((path, adjacencies)) = stack.pop() {
      let origin_face = path.last().unwrap();
      let original_front = cube.face_indices[&CubeFace::Front];
      if *origin_face == CubeFace::Back {
        let last = path.last().unwrap();
        cube.rotate_to_front(*last);
        cube.rotate_to_front(*last);
      } else {
        cube.rotate_to_front(*origin_face);
      }
      for i in 0..4 {
        if let Some(index) = adjacencies[i] {
          let Some(position) = mapping.get_mapping_location_of_index(index) else {
            return cube;
          };
          if !visited.contains(&position) {
            let heading = CubeFace::get_turn(&path).unwrap_or(0);
            let cardinal_index =
              (i as isize + heading as isize).rem_euclid(4) as usize;

            let result = cube.add_to(
              CubeFace::from_cardinal_index(cardinal_index),
              position.to_owned(),
            );
            if result.is_none() {
              eprintln!(
                "failed to add {:?} {} with heading \
                 {cardinal_index}={heading}+{i} from path {:?}",
                CubeFace::from_cardinal_index(cardinal_index),
                position.to_owned(),
                path
              );
              break;
            }
            cube.rotate_to_front(cube.faces[original_front].face_type);
            let mut new_path = path.clone();
            new_path.push(
              cube
                .faces
                .iter()
                .find(|face: &&Face| face.position == position)
                .unwrap()
                .face_type,
            );
            stack.push((
              new_path,
              mapping.get_2d_adjacencies(position.to_owned()),
            ));
            cube.rotate_to_front(*origin_face);

            visited.push(position);
          }
        }
      }
      cube.rotate_to_front(cube.faces[original_front].face_type);
    }

    if cube.faces.len() == 5 {
      for cubeface in cube.face_indices.keys() {
        let opposite = CubeFace::opposite(cubeface);
        if !cube.face_indices.contains_key(&opposite) {
          cube.add_face(Face {
            face_type: opposite,
            position: Coord { x: usize::MAX, y: usize::MAX },
          });
          break;
        }
      }
    }

    cube
  }
}

#[cfg(test)]
#[path = "./tests.rs"]
mod tests;
