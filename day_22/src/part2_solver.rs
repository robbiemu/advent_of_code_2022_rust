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

  fn solve(_input: Self::Input) -> Self::Solution {
    let mut read_head = Turtle::new();
    // let Some(location) = input.cube.get_first_open_position() else {
    //   return Self::Solution { read_head: None }
    // };
    // read_head.location = location;

    // input.tape.iter().for_each(|instruction| {
    //   read_head.apply(instruction.to_owned(), &input.cube)
    // });
    // represent_solution(&mut input.cube.clone(), &read_head);

    // Self::Solution { read_head: Some(read_head) }
    todo!()
  }

  fn output(_solution: Self::Solution) {
    // let Some(read_head) = solution.read_head else {
    //   println!("no solution found!");
    //   return;
    // };
    // let password = get_password(&read_head.location, &read_head.heading);

    // println!(
    //   "final coord {} heading {} : password {}",
    //   read_head.location, read_head.heading, password
    // )

    todo!()
  }
}

fn represent_solution(representation: &mut Board, read_head: &Turtle) {
  let canvas = representation.get_mut_ref();
  read_head
    .previous_way_points
    .iter()
    .for_each(|(pt, heading)| {
      canvas[pt.y][pt.x] = Legend::RepresentationOnlyTurtle(heading.to_owned())
    });
  canvas[read_head.location.y][read_head.location.x] =
    Legend::RepresentationOnlyTurtle(read_head.heading.clone());
  println!("{}", representation);
}

trait Apply {
  fn apply(&mut self, instruction: Instruction, board: &Board);
  fn traverse(&mut self, n: usize, board: &Board);
}
impl Apply for Turtle {
  fn apply(&mut self, instruction: Instruction, board: &Board) {
    match instruction {
      Instruction::AdjustHeading(turn) => self.heading.apply_turn(turn),
      Instruction::Move(n) => self.traverse(n, board),
    }
  }

  fn traverse(&mut self, n: usize, board: &Board) {
    self
      .previous_way_points
      .push((self.location.clone(), self.heading.clone()));

    let module = self.get_module(board);
    let offsets = self.get_offsets(board);
    let view = module.view_range(offsets.clone(), 0..=0);

    let (bearing, dim): (isize, &mut usize) = match self.heading {
      Heading::Left => (-1, &mut self.location.x),
      Heading::Right => (1, &mut self.location.x),
      Heading::Up => (-1, &mut self.location.y),
      Heading::Down => (1, &mut self.location.y),
    };

    let initial = *dim - offsets.start;
    if view.iter().all(|l| !matches!(l, Legend::Wall)) {
      *dim = offsets.start
        + (initial as isize + n as isize * bearing)
          .rem_euclid(view.len() as isize) as usize;
    } else {
      let target_index = offsets.start
        + (initial as isize + n as isize * bearing)
          .rem_euclid(view.len() as isize) as usize;

      let wall_relative_distance = if bearing.is_positive() {
        view
          .iter()
          .cycle()
          .skip(initial + 1 % view.len())
          .take(n.min(view.len()))
          .position(|t| matches!(t, Legend::Wall))
          .unwrap_or(n)
      } else {
        view
          .iter()
          .rev()
          .cycle()
          .skip(view.len() - initial)
          .take(n.min(view.len()))
          .position(|t| matches!(t, Legend::Wall))
          .unwrap_or(n)
      };
      let is_target = n < wall_relative_distance;
      let wall_index = offsets.start
        + (initial as isize + (wall_relative_distance as isize * bearing))
          .rem_euclid(view.len() as isize) as usize;

      *dim = if is_target { target_index } else { wall_index };
    }
  }
}


#[cfg(test)]
#[path = "./tests/part2_tests.rs"]
mod part2_tests;
