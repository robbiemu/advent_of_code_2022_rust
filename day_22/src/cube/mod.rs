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

enum Winding {
  Clockwise,
  CounterClockwise,
}

fn rotate_matrix<T: Clone + Default>(
  matrix: &Vec<Vec<T>>,
  winding: Winding,
) -> Vec<Vec<T>> {
  let rows = matrix.len();
  let cols = matrix[0].len();
  let mut rotated_matrix = vec![vec![Default::default(); rows]; cols];

  (0..rows).for_each(|i| {
    (0..cols).for_each(|j| match winding {
      Winding::Clockwise => {
        rotated_matrix[j][rows - i - 1] = matrix[i][j].clone()
      }
      Winding::CounterClockwise => {
        rotated_matrix[cols - j - 1][i] = matrix[i][j].clone()
      }
    });
  });

  rotated_matrix
}


#[derive(Clone, Debug, PartialEq)]
pub enum Orientation {
  Horizontal,
  Vertical,
}

impl Orientation {
  pub fn from_heading(heading: &Heading) -> Self {
    match heading {
      Heading::Left | Heading::Right => Orientation::Horizontal,
      Heading::Up | Heading::Down => Orientation::Vertical,
    }
  }

  fn get_opposite(&self) -> Orientation {
    match self {
      Orientation::Horizontal => Orientation::Vertical,
      Orientation::Vertical => Orientation::Horizontal,
    }
  }
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

  fn get_2d_adjacencies(&self, coord: &Coord) -> [Option<usize>; 4] {
    /* these adjacencies match the Heading scoring:
       0 => Right,
       1 => Down,
       2 => Left,
       3 => Up,
    */
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

  fn get_front_face(&self) -> Option<Coord> {
    let rows = self.0.len();
    let cols = self.0[0].len();
    let mut front_face_option: Option<Coord> = None;
    for y in 0..rows {
      for x in 0..cols {
        if self.0[y][x] == Legend::Open {
          front_face_option = Some(Coord::from((x, y)));
          break;
        }
      }
      if front_face_option.is_some() {
        break;
      }
    }

    front_face_option
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

  fn get_natural_heading(&self) -> Heading {
    match self {
      CubeFace::Down => Heading::Down,
      CubeFace::Up => Heading::Up,
      CubeFace::Right | CubeFace::Front => Heading::Right,
      CubeFace::Left | CubeFace::Back => Heading::Left,
    }
  }

  fn get_natural_orientation(&self) -> Orientation {
    match self {
      CubeFace::Down | CubeFace::Up => Orientation::Vertical,
      _ => Orientation::Horizontal,
    }
  }

  fn get_cube_face(&self, cardinal_index: usize) -> CubeFace {
    /* we will hold back as oriented so up and down remain constant */
    let axis = if cardinal_index % 2 == 1 {
      Orientation::Vertical
    } else {
      Orientation::Horizontal
    };
    let offset = match (self, &axis) {
      (CubeFace::Right | CubeFace::Back, Orientation::Horizontal) => 2,
      (CubeFace::Down, Orientation::Vertical) => 2,
      _ => 0,
    };
    let face = match (cardinal_index + offset) % 4 {
      0 => CubeFace::Right,
      1 => CubeFace::Down,
      2 => CubeFace::Left,
      3 => CubeFace::Up,
      _ => unreachable!(),
    };

    if axis == self.get_natural_orientation()
      && !matches!(self, CubeFace::Front | CubeFace::Back)
    {
      if (cardinal_index + offset) % 4 < 2 {
        CubeFace::Front
      } else {
        CubeFace::Back
      }
    } else {
      face
    }
  }

  fn get_cardinal_index_to_face(&self, face_type: &CubeFace) -> Option<usize> {
    (0..4).find(|&i| &self.get_cube_face(i) == face_type)
  }
}


// Struct to represent a face of the cube
#[derive(Debug, Clone)]
pub struct Face {
  face_type: CubeFace,
  position: Coord, // Position in the 2D mapping
}
impl Face {
  fn orient_values_to_face(
    &self,
    from_values: &mut Vec<Vec<Legend>>,
    heading: &Heading,
    to: &Face,
  ) -> Option<()> {
    let local_heading = Heading::from_score(
      self
        .face_type
        .get_cardinal_index_to_face(&to.face_type)?
        .try_into()
        .unwrap(),
    );
    let mut i = 0;
    while heading.get_score() != (local_heading.get_score() + i) % 4 {
      i += 1;
      *from_values = rotate_matrix(from_values, Winding::Clockwise);
    }
    if self.face_type == CubeFace::Back && i == 2 {
      match Orientation::from_heading(heading) {
        Orientation::Horizontal => from_values.reverse(),
        Orientation::Vertical => {
          from_values.iter_mut().for_each(|row| row.reverse())
        }
      }
    }
    dbg!(local_heading, i);

    Some(())
  }
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

  fn get_path(&self, front: &Face, target: &Face) -> Option<Vec<CubeFace>> {
    /* get the path along mapping from front to target */
    let mapping = self.mapping.as_ref()?;
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut final_path: Vec<CubeFace> = Vec::new();
    let mut stack = vec![(front.to_owned(), Vec::new())];
    while let Some((face, mut path)) = stack.pop() {
      visited.insert(face.position.to_owned());
      path.push(face.face_type.to_owned());
      for coord in mapping
        .get_2d_adjacencies(&face.position)
        .iter()
        .filter_map(|opt| *opt)
        .map(|i| mapping.get_mapping_location_of_index(i))
      {
        let next_face = self
          .faces
          .iter()
          .find(|f| Some(f.position.to_owned()) == coord)?;
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
    Some(final_path)
  }

  fn get_turn(&self, path: &Vec<CubeFace>) -> Option<isize> {
    let from = self.get_face(&path[path.len() - 2])?;
    let to = self.get_face(path.last().unwrap())?;

    let expected = to.face_type.get_cardinal_index_to_face(&from.face_type)?;
    let received = to.position.get_adjacency_index_to_coord(&from.position)?;

    let value = if to.face_type == CubeFace::Back {
      0
    } else {
      received as isize - expected as isize
    };

    Some(value)
  }

  fn get_face(&self, cube_face: &CubeFace) -> Option<&Face> {
    let target_index = self.face_indices.get(cube_face)?;
    Some(&self.faces[*target_index])
  }

  fn get_cube_face_values(
    &self,
    cube_face: CubeFace,
  ) -> Option<Vec<Vec<Legend>>> {
    /* All values will be correct from Front's perspective, followed up Up/Down,
    so Back will be oriented up but left and right reversed. */
    let board = self.board.clone()?;
    let dim = self.dim?;

    let target = &self.get_face(&cube_face)?;
    if target.position.x == usize::MAX && target.position.y == usize::MAX {
      return Some(vec![vec![Legend::Wall; dim]; dim]);
    }
    let (x, y) = (target.position.x * dim, target.position.y * dim);
    let mut face_values: Vec<Vec<Legend>> = board.get_ref()[y..(y + dim)]
      .iter()
      .map(|row| row[x..(x + dim)].to_vec())
      .collect();

    if target.face_type != CubeFace::Front {
      let front = &self.get_face(&CubeFace::Front)?;
      let path = self.get_path(front, target)?;
      dbg!(&path);
      let turns = self.get_turn(&path)?;
      dbg!(&turns);
      for _ in 0..turns.rem_euclid(4) {
        face_values = rotate_matrix(&face_values, Winding::CounterClockwise);
      }
      if cube_face == CubeFace::Back {
        let from_face = self.get_face(&path[path.len() - 2])?;
        if cube_face.get_cardinal_index_to_face(&from_face.face_type)
          != target
            .position
            .get_adjacency_index_to_coord(&from_face.position)
        {
          match from_face.face_type.get_natural_orientation() {
            Orientation::Horizontal => {
              face_values.iter_mut().for_each(|row| row.reverse())
            }
            Orientation::Vertical => face_values.reverse(),
          }
        }
      }
    }

    Some(face_values.to_owned())
  }

  fn get_cube_face_ring_indices(
    &mut self,
    cube_face: &CubeFace,
    heading: &Heading,
  ) -> Vec<usize> {
    let opposites = match Orientation::from_heading(heading) {
      Orientation::Horizontal => [CubeFace::Down, CubeFace::Up],
      Orientation::Vertical => [CubeFace::Left, CubeFace::Right],
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

    for ij in [
      cube_face_indices.as_slice().windows(2).collect::<Vec<_>>(),
      vec![&[
        cube_face_indices.last().unwrap().to_owned(),
        cube_face_indices.first().unwrap().to_owned(),
      ]],
    ]
    .concat()
    {
      let from = &self.faces[*ij.first().unwrap()];
      let to = &self.faces[*ij.last().unwrap()];
      let mut from_values = self.get_cube_face_values(from.face_type)?;

      /* if the starting face is Back, the local heading from following faces
      will be inverted if the next local heading from face to face is inverted */
      let cardinal_direction = if cube_face == CubeFace::Back
        && from.face_type != CubeFace::Back
        && from.face_type.get_cardinal_index_to_face(&to.face_type)? as isize
          == (heading.get_score() + 2) % 4
      {
        eprintln!("did it");
        Heading::from_score((heading.get_score() + 2) % 4)
      } else {
        heading.clone()
      };

      from.orient_values_to_face(&mut from_values, &cardinal_direction, to);
      dbg!(&location, &from.face_type, &from_values);

      ring_data.extend(get_ring_values(
        cardinal_direction.clone(),
        &from_values,
        location.clone(),
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

    let Some(front_face_location) = mapping.get_front_face() else {
      return cube;
    };
    let position = front_face_location.clone();
    cube.add_face(Face { face_type: CubeFace::Front, position });

    let mut stack: Vec<(Vec<CubeFace>, [Option<usize>; 4])> = Vec::new();
    let neighbors = mapping.get_2d_adjacencies(&front_face_location);
    stack.push((vec![CubeFace::Front], neighbors));

    let mut visited: Vec<Coord> = Vec::with_capacity(6);
    visited.push(front_face_location.clone());
    while let Some((path, adjacencies)) = stack.pop() {
      let center = cube.get_face(path.last().unwrap()).unwrap().to_owned();
      let mut index_offset = 0;
      if path.len() > 1 {
        let previous = cube.get_face(&path[path.len() - 2]).unwrap().to_owned();
        if let Some(from_index) = adjacencies.iter().position(|i_opt| {
          if let Some(i) = i_opt {
            if let Some(position) = mapping.get_mapping_location_of_index(*i) {
              return previous.position == position;
            }
          }

          false
        }) {
          index_offset = from_index as isize
            - center
              .face_type
              .get_cardinal_index_to_face(&previous.face_type)
              .unwrap() as isize;
        }
      }

      for i in 0..4 {
        if let Some(index) = adjacencies[i] {
          let Some(position) = mapping.get_mapping_location_of_index(index) else {
            return cube;
          };
          if !visited.contains(&position) {
            let cardinal_index =
              (i as isize - index_offset).rem_euclid(4) as usize;
            let target_face = center.face_type.get_cube_face(cardinal_index);
            let result = cube.add_to(target_face, position.to_owned());
            if result.is_none() {
              eprintln!(
                "failed to add relative face {:?} {} + direction \
                 {i}+{index_offset} with path {:?} and faces {:?}",
                target_face,
                position.to_owned(),
                path,
                cube.face_indices.keys()
              );
              break;
            }

            let mut new_path = path.clone();
            new_path.push(target_face);
            stack.push((new_path, mapping.get_2d_adjacencies(&position)));

            visited.push(position);
          }
        }
      }
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
