pub mod prelude {
  use chumsky::prelude::*;
  use nalgebra::DVector;
  use std::{
    fmt,
    fmt::{Display, Formatter},
    ops::Range,
  };

  use Heading::*;
  use Instruction::*;
  use Legend::*;
  use Turn::*;


  pub const STARTING_COORD: Coord = Coord { x: 0, y: 0 };

  #[derive(Clone, Debug, PartialEq)]
  pub enum Heading {
    Left,
    Right,
    Up,
    Down,
  }

  impl Heading {
    pub fn get_score(&self) -> isize {
      match self {
        Left => 2,
        Right => 0,
        Up => 3,
        Down => 1,
      }
    }

    pub fn from_score(score: isize) -> Heading {
      match score {
        0 => Right,
        1 => Down,
        2 => Left,
        3 => Up,
        _ => unimplemented!(),
      }
    }

    pub fn apply_turn(&mut self, turn: Turn) {
      let rotation = Heading::get_score(self);

      let offset = match turn {
        Clockwise => 1,
        CounterClickwise => -1,
      };

      *self = Heading::from_score((rotation + offset).rem_euclid(4))
    }
  }

  impl Display for Heading {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
      let representation = match self {
        Left => "<",
        Right => ">",
        Up => "^",
        Down => "v",
      };
      write!(f, "{representation}")
    }
  }

  #[derive(Clone, Debug, PartialEq)]
  pub enum Legend {
    RepresentationOnlyTurtle(Heading),
    Space,
    Open,
    Wall,
  }

  impl From<char> for Legend {
    fn from(value: char) -> Self {
      match value {
        ' ' => Space,
        '.' => Open,
        '#' => Wall,
        _ => unimplemented!(),
      }
    }
  }

  impl Display for Legend {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
      let form = match self {
        RepresentationOnlyTurtle(heading) => heading.to_string(),
        Space => " ".to_string(),
        Open => ".".to_string(),
        Wall => "#".to_string(),
      };

      write!(f, "{form}")
    }
  }

  #[derive(Clone, Debug, PartialEq)]
  pub struct Coord {
    pub x: usize,
    pub y: usize,
  }

  impl From<(usize, usize)> for Coord {
    fn from(coords: (usize, usize)) -> Self {
      Coord { x: coords.0, y: coords.1 }
    }
  }

  impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
      write!(f, "(x:{},y:{})", self.x, self.y)
    }
  }

  pub struct Turtle {
    pub location: Coord,
    pub heading: Heading,
    pub previous_way_points: Vec<(Coord, Heading)>,
  }

  impl Turtle {
    pub fn new() -> Turtle {
      Turtle {
        location: STARTING_COORD,
        heading: Right,
        previous_way_points: Vec::new(),
      }
    }

    /*
    A row or column of a matrix under modular arithmetic can be described as
    a mathematical structure called a module (also known as a vector space over
    a ring). Modules are generalizations of vector spaces, where instead of
    working over a field (as in the case of vector spaces), we work over a ring.
    */
    pub fn get_module(&self, board: &Board) -> DVector<Legend> {
      DVector::from(match self.heading {
        Left | Right => board.0[self.location.y].clone(),
        Up | Down => board
          .0
          .iter()
          .map(|row| row[self.location.x].clone())
          .collect(),
      })
    }

    pub(crate) fn get_offsets(&self, board: &Board) -> Range<usize> {
      let vector = match self.heading {
        Left | Right => board.0[self.location.y].clone(),
        Up | Down => board
          .0
          .iter()
          .map(|row| row[self.location.x].clone())
          .collect(),
      };

      Range {
        start: vector
          .iter()
          .position(|tile| !matches!(tile, Space))
          .unwrap(),
        end: vector.len()
          - vector
            .iter()
            .rev()
            .position(|tile| !matches!(tile, Space))
            .unwrap(),
      }
    }
  }

  #[derive(Clone)]
  pub struct Board(Vec<Vec<Legend>>);

  impl Board {
    pub fn from(source: impl Iterator<Item = String>) -> Board {
      let lines: Vec<String> = source.collect();
      let max_len = lines.iter().fold(0, |acc, cur| acc.max(cur.len()));

      Board(
        lines
          .into_iter()
          .map(|mut line| {
            if line.len() < max_len {
              let spaces = max_len - line.len();
              line.extend(vec![' '; spaces]);
            }

            line.chars().map(Legend::from).collect()
          })
          .collect::<Vec<Vec<_>>>(),
      )
    }

    pub fn get_first_open_position(&self) -> Option<Coord> {
      for (y, row) in self.0.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
          if matches!(tile, Open) {
            return Some(Coord::from((x, y)));
          }
        }
      }

      None
    }

    pub fn get_mut_ref(&mut self) -> &mut Vec<Vec<Legend>> {
      &mut self.0
    }
  }

  impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
      let representation: Vec<String> = self
        .0
        .iter()
        .map(|row| {
          row
            .iter()
            .map(|tile| tile.to_string())
            .collect::<Vec<_>>()
            .join("")
        })
        .collect();

      writeln!(f, "{}", representation.join("\n"))
    }
  }

  #[derive(Clone, Debug)]
  pub enum Turn {
    Clockwise,
    CounterClickwise,
  }

  impl Turn {
    fn is_turn_indication(c: char) -> bool {
      matches!(c, 'L' | 'R')
    }
  }

  impl From<char> for Turn {
    fn from(value: char) -> Self {
      match value {
        'L' => CounterClickwise,
        'R' => Clockwise,
        _ => unimplemented!(),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub enum Instruction {
    AdjustHeading(Turn),
    Move(usize),
  }

  pub type Tape = Vec<Instruction>;

  pub fn tokenizer(
  ) -> impl Parser<char, Vec<Instruction>, Error = chumsky::error::Simple<char>>
  {
    choice((
      filter(|c: &char| c.is_ascii_digit())
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(|digits| Move(digits.parse().unwrap_or(0))),
      filter(|c: &char| Turn::is_turn_indication(*c))
        .map(|c| AdjustHeading(Turn::from(c))),
    ))
    .repeated()
    .collect::<Vec<_>>()
  }
}

use chumsky::Parser;
use prelude::*;


pub fn extract_board_and_turns_from_stream(
  source: impl Iterator<Item = String>,
) -> Result<(Board, Tape), String> {
  let mut board_src: Vec<String> = source.collect();
  let Some(instructions_src) = board_src.pop() else {
    return Err("no valid board in source".to_string());
  };
  board_src.pop();

  let board = Board::from(board_src.into_iter());

  let parser = tokenizer();
  let Ok(instructions) = parser.parse(instructions_src) else {
    return Err("no valid instructions in source".to_string());
  };

  Ok((board, instructions))
}
