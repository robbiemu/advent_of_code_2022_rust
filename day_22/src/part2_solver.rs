use super::problem_solver::ProblemSolver;
use crate::common::{
  extract_board_and_turns_from_stream, get_password, prelude::*,
};
use crate::cube::*;


pub struct PSInput {
  cube: Cube,
  tape: Tape,
}

pub struct PSSolution {
  read_head: Option<Turtle>,
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;
  type Err = String;


  fn initialize(
    lines: impl Iterator<Item = String>,
  ) -> Result<Self::Input, Self::Err> {
    let (board, tape) = extract_board_and_turns_from_stream(lines)?;
    let mapping = Mapping::from_2d_vector(board.get_ref())?;

    let mut cube = Cube::from(&mapping);
    cube.board = Some(board.clone());
    cube.dim = Some(
      Mapping::get_board_dimension(board.get_ref()).ok_or("invalid board")?,
    );

    Ok(Self::Input { cube, tape })
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let mut read_head = Turtle::new();
    let Some(location) = input.cube.get_first_open_position() else {
      return Self::Solution { read_head: None };
    };
    read_head.location = location;

    let mut cube = input.cube.clone();

    input.tape.iter().for_each(|instruction| {
      read_head.apply(instruction.to_owned(), &mut cube)
    });
    represent_solution(&mut input.cube.clone(), &read_head);

    let face_position =
      cube.faces[read_head.cube_face_index].position.to_owned();
    let dim = cube.dim.unwrap();
    read_head.location = cube
      .rotate_coordinates(
        read_head.location.clone(),
        &cube.faces[read_head.cube_face_index].face_type,
      )
      .unwrap();
    read_head.location = Coord::from((
      read_head.location.x + dim * face_position.x,
      read_head.location.y + dim * face_position.y,
    ));

    read_head.heading = Heading::from_score(
      (read_head.heading.get_score()
        + cube
          .get_turn_to_face(&cube.faces[read_head.cube_face_index].face_type)
          .unwrap_or(0))
      .rem_euclid(4),
    );

    Self::Solution { read_head: Some(read_head) }
  }

  fn output(solution: Self::Solution) {
    let Some(read_head) = solution.read_head else {
      println!("no solution found!");
      return;
    };
    let password = get_password(&read_head.location, &read_head.heading);

    println!(
      "final coord {} heading {} : password {}",
      read_head.location, read_head.heading, password
    )
  }
}

fn represent_solution(cube: &mut Cube, read_head: &Turtle) {
  let mut binding = cube.clone();
  let board = binding.board.as_mut().unwrap();
  let canvas = board.get_mut_ref();
  let dim = cube.dim.unwrap();
  read_head.previous_way_points.iter().for_each(
    |(pt, cube_face_index, heading)| {
      let face = &cube.faces[*cube_face_index];
      let coords = cube
        .rotate_coordinates(pt.to_owned(), &face.face_type)
        .unwrap();
      eprintln!("({}) from {} @{:?}", coords, pt, face.face_type);
      canvas[coords.y + face.position.y * dim]
        [coords.x + face.position.x * dim] =
        Legend::RepresentationOnlyTurtle(heading.to_owned());
    },
  );

  let face = &cube.faces[read_head.cube_face_index];
  let coords = cube
    .rotate_coordinates(read_head.location.clone(), &face.face_type)
    .unwrap();
  canvas[coords.y + face.position.y * dim][coords.x + face.position.x * dim] =
    Legend::RepresentationOnlyTurtle(read_head.heading.clone());

  println!("{}", board);
}

trait Apply {
  fn apply(&mut self, instruction: Instruction, cube: &mut Cube);
  fn traverse(&mut self, n: usize, cube: &mut Cube);
}

impl Apply for Turtle {
  fn apply(&mut self, instruction: Instruction, cube: &mut Cube) {
    match instruction {
      Instruction::AdjustHeading(turn) => self.heading.apply_turn(turn),
      Instruction::Move(n) => self.traverse(n, cube),
    }
  }

  fn traverse(&mut self, mut n: usize, cube: &mut Cube) {
    self.previous_way_points.push((
      self.location.clone(),
      self.cube_face_index,
      self.heading.clone(),
    ));

    let cube_face = cube
      .face_indices
      .keys()
      .find(|key| cube.face_indices[key] == self.cube_face_index)
      .unwrap()
      .to_owned();

    let ring = cube
      .get_cube_face_ring(
        cube_face,
        self.location.to_owned(),
        self.heading.to_owned(),
      )
      .unwrap();
    dbg!(&cube_face, &self.heading, &self.location, &ring);

    if let Some(wall_offset) = ring
      .iter()
      .take(n.min(ring.len() - 1))
      .position(|l| matches!(l, Legend::Wall))
    {
      /*  wall_offset is the index in the ring where the wall is. for example,
      if it is at 1, then the second point away from start is the index of the
      wall. So, turtle will stop just before, at index 0. this will be a
      traversal distance n of 1; the turtle travelled one point away from start.
      n is the traversal distance: it starts at the value requested, but a wall
      interrupts. if so, we need to update n to the distance to the wall. this
      is the length of the array up to the wall - 1, which happens to be the
      wall index.
      */
      eprintln!("limiting traversal length {n} to wall {wall_offset}");
      n = wall_offset;
    }

    // adjust final face (we have starting face assigned to cube_face)
    let face_dim = ring.len() / 4;
    let coordinate_dimension = match Orientation::from_heading(&self.heading) {
      Orientation::Horizontal => self.location.x,
      Orientation::Vertical => self.location.y,
    };
    let span_on_initial_face = match self.heading {
      Heading::Right | Heading::Down => face_dim - coordinate_dimension - 1,
      Heading::Left | Heading::Up => coordinate_dimension,
    };
    if let Some(faces_to_traverse_to_destination) =
      get_faces_traversed(n, face_dim)
    {
      let cube_face_indices =
        cube.get_cube_face_ring_indices(&cube_face, &self.heading);
      self.cube_face_index =
        cube_face_indices[faces_to_traverse_to_destination];
      eprintln!(
        "[on move of {}] from {:?}, faces traversed {} to {:?}",
        n,
        cube_face,
        faces_to_traverse_to_destination,
        cube.faces[self.cube_face_index].face_type
      );

      if let Some(next_heading) =
        cube.get_next_heading_in_ring(self.cube_face_index, &cube_face_indices)
      {
        if self.heading != next_heading {
          eprintln!(
            "heading changed when changing faces. new heading: {:?}",
            next_heading
          );
          match Orientation::from_heading(&self.heading) {
            Orientation::Horizontal => self.location.x = self.location.y,
            Orientation::Vertical => self.location.y = self.location.x,
          };
          self.heading = next_heading;
        }
      }
    }

    let mut dim_value = get_dim_value(
      n,
      face_dim,
      &self.location,
      &self.heading,
      span_on_initial_face,
    );
    if cube.faces[self.cube_face_index].face_type == CubeFace::Back
      && Orientation::from_heading(&self.heading) == Orientation::Horizontal
    {
      dim_value = face_dim - dim_value - 1;
    }
    match Orientation::from_heading(&self.heading) {
      Orientation::Horizontal => self.location.x = dim_value,
      Orientation::Vertical => self.location.y = dim_value,
    };
    eprintln!("after move, location on face is {:?}", self.location);
  }
}

fn get_faces_traversed(n: usize, dim: usize) -> Option<usize> {
  /* assuming motion around a ring of four faces of the cube, find how many
  faces beyond/behind the first we have traveled given distance n, heading and
  dim length of each face.
  Returns None if the traversal does not terminate on a different face.
  */
  let n = (n / dim) % 4;
  if n != 0 {
    Some(n)
  } else {
    None
  }
}

fn get_dim_value(
  n: usize,
  dim: usize,
  location: &Coord,
  heading: &Heading,
  span_on_initial_face: usize,
) -> usize {
  /* given a traversal length, face dimension, and location and heading data,
  return the position along that dimension on the destination cube face
  indicated by the length of traversal */
  let coordinate_dimension = match Orientation::from_heading(heading) {
    Orientation::Horizontal => location.x,
    Orientation::Vertical => location.y,
  };

  eprintln!("n {n} <> span on initial face {span_on_initial_face}");
  match n.cmp(&span_on_initial_face) {
    std::cmp::Ordering::Less => match heading {
      Heading::Right | Heading::Down => coordinate_dimension + n,
      Heading::Left | Heading::Up => coordinate_dimension - n,
    },
    std::cmp::Ordering::Equal => match heading {
      Heading::Right | Heading::Down => dim - 1,
      Heading::Left | Heading::Up => 0,
    },
    std::cmp::Ordering::Greater => {
      let final_span = (n - span_on_initial_face) % dim;
      if (matches!(heading, Heading::Right | Heading::Down) && final_span == 0)
        || (matches!(heading, Heading::Left | Heading::Up)
          && dim - (final_span - 1) - 1 == 0)
      {
        return match heading {
          Heading::Right | Heading::Down => 0,
          Heading::Left | Heading::Up => coordinate_dimension,
        };
      }
      eprintln!("{final_span} = {n} - {span_on_initial_face} % {dim}");
      match heading {
        Heading::Right | Heading::Down => final_span - 1,
        Heading::Left | Heading::Up => dim - (final_span - 1) - 1,
      }
    }
  }
}

#[cfg(test)]
#[path = "./tests/part2_tests.rs"]
mod part2_tests;
