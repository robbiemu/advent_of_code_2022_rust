use crate::part1_solver::*;


const SIMPLE_MAP: &str = "    \n ..#\n ...\n #..\n";
const LARGER_MAP: &str = "     \n ....\n .#..\n ....\n ..#.\n ....\n";

#[test]
fn it_should_go_right_without_hitting_a_wall() {
  // Arrange
  let mut input: Vec<String> = SIMPLE_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "1".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((2, 2));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((3, 2)));
}

#[test]
fn it_should_go_down_without_hitting_a_wall() {
  // Arrange
  let mut input: Vec<String> = SIMPLE_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "R1".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((2, 2));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((2, 3)));
}

#[test]
fn it_should_go_up_without_hitting_a_wall() {
  // Arrange
  let mut input: Vec<String> = SIMPLE_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "L1".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((2, 2));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((2, 1)));
}

#[test]
fn it_should_go_left_without_hitting_a_wall() {
  // Arrange
  let mut input: Vec<String> = SIMPLE_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "LL1".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((2, 2));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((1, 2)));
}

#[test]
fn it_should_wrap_right_without_hitting_a_wall() {
  // Arrange
  let mut input: Vec<String> = SIMPLE_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "2".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((2, 2));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((1, 2)));
}


#[test]
fn it_should_wrap_down_without_hitting_a_wall() {
  // Arrange
  let mut input: Vec<String> = SIMPLE_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "R2".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((2, 2));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((2, 1)));
}

#[test]
fn it_should_wrap_up_without_hitting_a_wall() {
  // Arrange
  let mut input: Vec<String> = SIMPLE_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "L2".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((2, 2));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((2, 3)));
}

#[test]
fn it_should_wrap_left_without_hitting_a_wall() {
  // Arrange
  let mut input: Vec<String> = SIMPLE_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "LL2".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((2, 2));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((3, 2)));
}

#[test]
fn it_should_wrap_through_right_without_hitting_a_wall() {
  // Arrange
  let mut input: Vec<String> = SIMPLE_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "4".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((2, 2));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((3, 2)));
}

#[test]
fn it_should_wrap_through_down_without_hitting_a_wall() {
  // Arrange
  let mut input: Vec<String> = SIMPLE_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "R4".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((2, 2));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((2, 3)));
}

#[test]
fn it_should_wrap_through_up_without_hitting_a_wall() {
  // Arrange
  let mut input: Vec<String> = SIMPLE_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "L4".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((2, 2));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((2, 1)));
}

#[test]
fn it_should_wrap_through_left_without_hitting_a_wall() {
  // Arrange
  let mut input: Vec<String> = SIMPLE_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "LL4".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((2, 2));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((1, 2)));
}

#[test]
fn it_should_stop_short_right_at_a_wall() {
  // Arrange
  let mut input: Vec<String> = SIMPLE_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "3".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((1, 1));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((2, 1)));
}

#[test]
fn it_should_stop_short_down_at_a_wall() {
  // Arrange
  let mut input: Vec<String> = SIMPLE_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "R4".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((1, 1));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((1, 2)));
}

#[test]
fn it_should_stop_short_up_at_a_wall() {
  // Arrange
  let mut input: Vec<String> = SIMPLE_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "L4".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((3, 3));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((3, 2)));
}

#[test]
fn it_should_stop_short_left_at_a_wall() {
  // Arrange
  let mut input: Vec<String> = SIMPLE_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "LL4".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((3, 3));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((2, 3)));
}

#[test]
fn it_stop_at_a_wall_when_wrapping_right() {
  // Arrange
  let mut input: Vec<String> = LARGER_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "4".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((1, 4));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((2, 4)));
}

#[test]
fn it_stop_at_a_wall_when_wrapping_down() {
  // Arrange
  let mut input: Vec<String> = LARGER_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "R4".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((3, 2));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((3, 3)));
}

#[test]
fn it_stop_at_a_wall_when_wrapping_up() {
  // Arrange
  let mut input: Vec<String> = LARGER_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "L4".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((2, 4));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  represent_solution(&mut board.clone(), &read_head);

  // Assert
  assert_eq!(read_head.location, Coord::from((2, 3)));
}

#[test]
fn it_stop_at_a_wall_when_wrapping_left() {
  // Arrange
  let mut input: Vec<String> = LARGER_MAP
    .split('\n')
    .map(|l| l.to_string())
    .collect::<Vec<_>>();
  input.extend(vec!["".to_string(), "LL3".to_string()]);
  let (board, tape) =
    extract_board_and_turns_from_stream(input.into_iter()).unwrap();

  let mut read_head = Turtle::new();
  read_head.location = Coord::from((4, 2));

  // Act
  tape
    .iter()
    .for_each(|instruction| read_head.apply(instruction.to_owned(), &board));

  // Assert
  assert_eq!(read_head.location, Coord::from((3, 2)));
}
