use super::*;

#[test]
#[allow(non_snake_case)]
fn test_measure_2D_array_members_span() {
  let sample: Vec<Vec<Legend>> = vec![
    vec![Legend::Open, Legend::Open, Legend::Open],
    vec![Legend::Open, Legend::Open, Legend::Open],
    vec![Legend::Open, Legend::Open, Legend::Open],
    vec![Legend::Wall, Legend::Wall, Legend::Space],
    vec![Legend::Wall, Legend::Wall, Legend::Space],
    vec![Legend::Space, Legend::Space, Legend::Space],
  ];

  assert_eq!(
    measure_2D_array_members_span(&sample, &Legend::Open),
    Some(3)
  );

  assert_eq!(
    measure_2D_array_members_span(&sample, &Legend::Space),
    Some(1)
  );

  assert_eq!(
    measure_2D_array_members_span(&sample, &Legend::Wall),
    Some(2)
  );
}

#[test]
fn test_get_ring_values() {
  let arr: Vec<Vec<Legend>> = vec![
    vec![Legend::Open, Legend::Space, Legend::Open],
    vec![Legend::Open, Legend::Wall, Legend::Open],
    vec![Legend::Space, Legend::Open, Legend::Open],
  ];

  let location = Coord::from((1, 1));
  let mut expected_result: Vec<Legend> = arr[1].clone();
  assert_eq!(
    get_ring_values(Heading::Right, &arr, location.to_owned())
      .collect::<Vec<_>>(),
    expected_result
  );
  expected_result.reverse();
  assert_eq!(
    get_ring_values(Heading::Left, &arr, location.to_owned())
      .collect::<Vec<_>>(),
    expected_result
  );

  expected_result = arr.iter().map(|row| row[1].to_owned()).collect();
  assert_eq!(
    get_ring_values(Heading::Down, &arr, location.to_owned())
      .collect::<Vec<_>>(),
    expected_result
  );
  expected_result.reverse();
  assert_eq!(
    get_ring_values(Heading::Up, &arr, location.to_owned()).collect::<Vec<_>>(),
    expected_result
  );
}


// MARK Mapping
#[test]
fn test_mapping_methods() {
  let sample: Vec<Vec<Legend>> = vec![
    vec![Legend::Open],
    vec![Legend::Open, Legend::Wall, Legend::Open],
    vec![Legend::Space, Legend::Space, Legend::Open, Legend::Open],
  ];

  let mapping = Mapping::from_2d_vector(sample.as_slice())
    .expect("failed to parse mapping");

  // Test get_board_dimension
  assert_eq!(Mapping::get_board_dimension(sample.as_slice()), Some(1));

  // Test get_mapping_location_of_index
  assert_eq!(
    mapping.get_mapping_location_of_index(0),
    Some(Coord { x: 0, y: 0 })
  );
  assert_eq!(
    mapping.get_mapping_location_of_index(1),
    Some(Coord { x: 0, y: 1 })
  );
  assert_eq!(
    mapping.get_mapping_location_of_index(2),
    Some(Coord { x: 1, y: 1 })
  );

  // Test get_index_of_mapping_location
  assert_eq!(
    mapping.get_index_of_mapping_location(Coord { x: 0, y: 0 }),
    Some(0)
  );
  assert_eq!(
    mapping.get_index_of_mapping_location(Coord { x: 0, y: 1 }),
    Some(1)
  );
  assert_eq!(
    mapping.get_index_of_mapping_location(Coord { x: 1, y: 1 }),
    Some(2)
  );

  // Test get_2d_adjacencies
  assert_eq!(
    mapping.get_2d_adjacencies(&Coord { x: 0, y: 0 }),
    [None, Some(1), None, None]
  );
  assert_eq!(
    mapping.get_2d_adjacencies(&Coord { x: 0, y: 1 }),
    [Some(2), None, None, Some(0)]
  );
  assert_eq!(
    mapping.get_2d_adjacencies(&Coord { x: 1, y: 1 }),
    [Some(3), None, Some(1), None]
  );
  assert_eq!(
    mapping.get_2d_adjacencies(&Coord { x: 2, y: 1 }),
    [None, Some(4), Some(2), None]
  );
}


// MARK Cube
use crate::common::extract_board_and_turns_from_stream;

#[test]
fn test_cube_face_get_cube_face() {
  assert_eq!(
    CubeFace::Front.get_cube_face(Heading::Left.get_score() as usize),
    CubeFace::Left,
  );
  assert_eq!(
    CubeFace::Front.get_cube_face(Heading::Right.get_score() as usize),
    CubeFace::Right,
  );
  assert_eq!(
    CubeFace::Front.get_cube_face(Heading::Up.get_score() as usize),
    CubeFace::Up,
  );
  assert_eq!(
    CubeFace::Front.get_cube_face(Heading::Down.get_score() as usize),
    CubeFace::Down,
  );

  assert_eq!(
    CubeFace::Back.get_cube_face(Heading::Left.get_score() as usize),
    CubeFace::Right,
  );
  assert_eq!(
    CubeFace::Back.get_cube_face(Heading::Right.get_score() as usize),
    CubeFace::Left,
  );
  assert_eq!(
    CubeFace::Back.get_cube_face(Heading::Up.get_score() as usize),
    CubeFace::Up,
  );
  assert_eq!(
    CubeFace::Back.get_cube_face(Heading::Down.get_score() as usize),
    CubeFace::Down,
  );

  assert_eq!(
    CubeFace::Left.get_cube_face(Heading::Left.get_score() as usize),
    CubeFace::Back,
  );
  assert_eq!(
    CubeFace::Left.get_cube_face(Heading::Right.get_score() as usize),
    CubeFace::Front,
  );
  assert_eq!(
    CubeFace::Left.get_cube_face(Heading::Up.get_score() as usize),
    CubeFace::Up,
  );
  assert_eq!(
    CubeFace::Left.get_cube_face(Heading::Down.get_score() as usize),
    CubeFace::Down,
  );

  assert_eq!(
    CubeFace::Right.get_cube_face(Heading::Left.get_score() as usize),
    CubeFace::Front,
  );
  assert_eq!(
    CubeFace::Right.get_cube_face(Heading::Right.get_score() as usize),
    CubeFace::Back,
  );
  assert_eq!(
    CubeFace::Right.get_cube_face(Heading::Up.get_score() as usize),
    CubeFace::Up,
  );
  assert_eq!(
    CubeFace::Right.get_cube_face(Heading::Down.get_score() as usize),
    CubeFace::Down,
  );

  assert_eq!(
    CubeFace::Up.get_cube_face(Heading::Left.get_score() as usize),
    CubeFace::Left,
  );
  assert_eq!(
    CubeFace::Up.get_cube_face(Heading::Right.get_score() as usize),
    CubeFace::Right,
  );
  assert_eq!(
    CubeFace::Up.get_cube_face(Heading::Up.get_score() as usize),
    CubeFace::Back,
  );
  assert_eq!(
    CubeFace::Up.get_cube_face(Heading::Down.get_score() as usize),
    CubeFace::Front,
  );

  assert_eq!(
    CubeFace::Down.get_cube_face(Heading::Left.get_score() as usize),
    CubeFace::Left,
  );
  assert_eq!(
    CubeFace::Down.get_cube_face(Heading::Right.get_score() as usize),
    CubeFace::Right,
  );
  assert_eq!(
    CubeFace::Down.get_cube_face(Heading::Up.get_score() as usize),
    CubeFace::Front,
  );
  assert_eq!(
    CubeFace::Down.get_cube_face(Heading::Down.get_score() as usize),
    CubeFace::Back,
  );
}

#[test]
fn test_cube_initialization() {
  let inputs = [
    include_str!("../../sample.txt"),
    include_str!("../../sample_rotated.txt"),
    include_str!("../../mini_input.txt"),
    include_str!("../../sample_2.txt"),
    include_str!("../../sample_2_rotated.txt"),
    include_str!("../../sample_3.txt"),
  ];

  for (i, input) in inputs.iter().enumerate() {
    let (board, _tape) = extract_board_and_turns_from_stream(
      input.split('\n').map(|s| s.to_string()),
    )
    .expect("invalid board");

    let mapping = Mapping::from_2d_vector(board.get_ref())
      .expect("invalid board for mapping");
    dbg!(&mapping);
    let mut cube = Cube::from(&mapping);
    cube.board = Some(board.clone());
    cube.dim = Some(
      Mapping::get_board_dimension(board.get_ref())
        .unwrap_or_else(|| panic!("invalid board")),
    );

    if i != 2 {
      assert!(!cube.faces.iter().any(|face| face.position.x == usize::MAX));
    }
    assert_eq!(cube.faces.len(), 6);
    assert_eq!(cube.face_indices.len(), 6);
  }
}

#[test]
fn test_get_cube_face_values() {
  let input = include_str!("../../sample.txt");

  let (board, _tape) = extract_board_and_turns_from_stream(
    input.split('\n').map(|s| s.to_string()),
  )
  .expect("invalid board");

  let mapping = Mapping::from_2d_vector(board.get_ref())
    .expect("invalid board for mapping");
  dbg!(&mapping);
  let mut cube = Cube::from(&mapping);
  cube.board = Some(board.clone());
  cube.dim = Some(
    Mapping::get_board_dimension(board.get_ref())
      .unwrap_or_else(|| panic!("invalid board")),
  );

  dbg!(&cube.faces);

  // Test get_cube_face_values
  use Legend::*;

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Front),
    Some(
      [
        [Open, Open, Open, Wall],
        [Open, Wall, Open, Open],
        [Wall, Open, Open, Open],
        [Open, Open, Open, Open]
      ]
      .iter()
      .map(|r| r.to_vec())
      .collect()
    )
  );

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Back),
    Some(
      [
        [Open, Open, Open, Open],
        [Open, Wall, Open, Open],
        [Open, Open, Open, Open],
        [Open, Open, Open, Wall]
      ]
      .iter()
      .map(|r| r.to_vec())
      .collect()
    )
  );

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Right),
    Some(
      [
        [Open, Wall, Open, Open],
        [Open, Open, Open, Open],
        [Open, Open, Wall, Open],
        [Open, Open, Open, Open]
      ]
      .iter()
      .map(|r| r.to_vec())
      .collect()
    )
  );

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Left),
    Some(
      [
        [Open, Open, Open, Open],
        [Open, Open, Open, Open],
        [Open, Open, Open, Open],
        [Open, Wall, Open, Open],
      ]
      .iter()
      .map(|r| r.to_vec())
      .collect()
    )
  );

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Down),
    Some(
      [
        [Open, Open, Open, Wall],
        [Wall, Open, Open, Open],
        [Open, Open, Open, Open],
        [Open, Open, Wall, Open]
      ]
      .iter()
      .map(|r| r.to_vec())
      .collect()
    )
  );

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Up),
    Some(
      [
        [Open, Open, Open, Open],
        [Open, Wall, Open, Open],
        [Open, Open, Open, Open],
        [Wall, Open, Open, Open],
      ]
      .iter()
      .map(|r| r.to_vec())
      .collect()
    )
  );

  let input = include_str!("../../mini_input.txt");

  let (board, _tape) = extract_board_and_turns_from_stream(
    input.split('\n').map(|s| s.to_string()),
  )
  .expect("invalid board");

  let mapping = Mapping::from_2d_vector(board.get_ref())
    .expect("invalid board for mapping");
  dbg!(&mapping);
  let mut cube = Cube::from(&mapping);
  cube.board = Some(board.clone());
  cube.dim = Some(
    Mapping::get_board_dimension(board.get_ref())
      .unwrap_or_else(|| panic!("invalid board")),
  );

  dbg!(&cube.faces);

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Front),
    Some(
      [[Open, Open, Open], [Wall, Open, Open], [Open, Wall, Open]]
        .iter()
        .map(|r| r.to_vec())
        .collect()
    )
  );

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Back),
    Some(
      [[Wall, Open, Wall], [Open, Open, Open], [Open, Wall, Wall]]
        .iter()
        .map(|r| r.to_vec())
        .collect()
    )
  );

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Left),
    Some(
      [[Open, Wall, Wall], [Open, Wall, Open], [Wall, Open, Wall],]
        .iter()
        .map(|r| r.to_vec())
        .collect()
    )
  );

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Down),
    Some(
      [[Open, Wall, Open], [Open, Wall, Open], [Open, Open, Wall]]
        .iter()
        .map(|r| r.to_vec())
        .collect()
    )
  );

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Up),
    Some(
      [[Wall, Wall, Wall], [Open, Open, Wall], [Wall, Open, Wall]]
        .iter()
        .map(|r| r.to_vec())
        .collect()
    )
  );
}

#[test]
fn test_get_cube_face_ring() {
  let input = include_str!("../../sample.txt");

  let (board, _tape) = extract_board_and_turns_from_stream(
    input.split('\n').map(|s| s.to_string()),
  )
  .expect("invalid board");

  let mapping = Mapping::from_2d_vector(board.get_ref())
    .expect("invalid board for mapping");
  dbg!(&mapping);
  let mut cube = Cube::from(&mapping);
  cube.board = Some(board.clone());
  cube.dim = Some(
    Mapping::get_board_dimension(board.get_ref())
      .unwrap_or_else(|| panic!("invalid board")),
  );

  dbg!(&cube.faces);

  let mut expected_ring_values: Vec<Legend> = vec![
    // Right
    Legend::Open,
    // Front
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Wall,
    // Left
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Back
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Right
    Legend::Open,
    Legend::Wall,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(
      CubeFace::Right,
      Coord { x: 1, y: 2 },
      Heading::Left
    ),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Right
    Legend::Wall,
    Legend::Open,
    // Back
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Left
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Front
    Legend::Wall,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Right
    Legend::Open,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(
      CubeFace::Right,
      Coord { x: 1, y: 2 },
      Heading::Right
    ),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Right
    Legend::Open,
    Legend::Wall,
    // Up
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Left
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Down
    Legend::Wall,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Right
    Legend::Open,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(CubeFace::Right, Coord { x: 1, y: 2 }, Heading::Up),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Right
    Legend::Wall,
    Legend::Open,
    // Down
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Left
    Legend::Wall,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Up
    Legend::Open,
    Legend::Wall,
    Legend::Open,
    Legend::Open,
    // Right
    Legend::Open,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(
      CubeFace::Right,
      Coord { x: 2, y: 1 },
      Heading::Down
    ),
    Some(expected_ring_values)
  );

  // Up
  expected_ring_values = vec![
    // Up
    Legend::Wall,
    Legend::Open,
    // Back
    Legend::Open,
    Legend::Wall,
    Legend::Open,
    Legend::Open,
    // Down
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Front
    Legend::Open,
    Legend::Open,
    Legend::Wall,
    Legend::Open,
    // Up
    Legend::Open,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(CubeFace::Up, Coord { x: 1, y: 2 }, Heading::Up),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Up
    Legend::Open,
    // Front
    Legend::Open,
    Legend::Wall,
    Legend::Open,
    Legend::Open,
    // Down
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Back
    Legend::Open,
    Legend::Open,
    Legend::Wall,
    Legend::Open,
    // Up
    Legend::Open,
    Legend::Wall,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(CubeFace::Up, Coord { x: 1, y: 2 }, Heading::Down),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Up
    Legend::Open,
    // Left
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Down
    Legend::Wall,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Right
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Wall,
    // Up
    Legend::Open,
    Legend::Open,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(CubeFace::Up, Coord { x: 1, y: 2 }, Heading::Left),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Up
    Legend::Open,
    Legend::Open,
    // Right
    Legend::Wall,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Down
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Wall,
    // Left
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Up
    Legend::Open,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(CubeFace::Up, Coord { x: 1, y: 2 }, Heading::Right),
    Some(expected_ring_values)
  );

  // Back
  expected_ring_values = vec![
    // Back
    Legend::Open,
    // Left
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Front
    Legend::Wall,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Right
    Legend::Open,
    Legend::Open,
    Legend::Wall,
    Legend::Open,
    // Back
    Legend::Open,
    Legend::Open,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(
      CubeFace::Back,
      Coord { x: 2, y: 2 },
      Heading::Right
    ),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Back
    Legend::Open,
    Legend::Open,
    // Right
    Legend::Open,
    Legend::Wall,
    Legend::Open,
    Legend::Open,
    // Front
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Wall,
    // Left
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Back
    Legend::Open,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(
      CubeFace::Back,
      Coord { x: 2, y: 2 },
      Heading::Left
    ),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Back
    Legend::Open,
    // Down
    Legend::Wall,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Front
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Up
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Back
    Legend::Open,
    Legend::Open,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(
      CubeFace::Back,
      Coord { x: 2, y: 2 },
      Heading::Down
    ),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Back
    Legend::Wall,
    Legend::Open,
    // Up
    Legend::Open,
    Legend::Wall,
    Legend::Open,
    Legend::Open,
    // Front
    Legend::Open,
    Legend::Wall,
    Legend::Open,
    Legend::Open,
    // Down
    Legend::Open,
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Back
    Legend::Open,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(CubeFace::Back, Coord { x: 1, y: 2 }, Heading::Up),
    Some(expected_ring_values)
  );

  let input = include_str!("../../mini_input.txt");

  let (board, _tape) = extract_board_and_turns_from_stream(
    input.split('\n').map(|s| s.to_string()),
  )
  .expect("invalid board");

  let mapping = Mapping::from_2d_vector(board.get_ref())
    .expect("invalid board for mapping");
  dbg!(&mapping);
  let mut cube = Cube::from(&mapping);
  cube.board = Some(board.clone());
  cube.dim = Some(
    Mapping::get_board_dimension(board.get_ref())
      .unwrap_or_else(|| panic!("invalid board")),
  );

  dbg!(&cube.faces);

  // Up
  expected_ring_values = vec![
    // Up
    Legend::Wall,
    // Back
    Legend::Open,
    Legend::Open,
    Legend::Wall,
    // Down
    Legend::Open,
    Legend::Wall,
    Legend::Wall,
    // Front
    Legend::Wall,
    Legend::Open,
    Legend::Open,
    // Up
    Legend::Open,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(CubeFace::Up, Coord { x: 1, y: 1 }, Heading::Up),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Up
    Legend::Open,
    // Front
    Legend::Open,
    Legend::Open,
    Legend::Wall,
    // Down
    Legend::Wall,
    Legend::Wall,
    Legend::Open,
    // Back
    Legend::Wall,
    Legend::Open,
    Legend::Open,
    // Up
    Legend::Wall,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(CubeFace::Up, Coord { x: 1, y: 1 }, Heading::Down),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Up
    Legend::Open,
    // Left
    Legend::Wall,
    Legend::Wall,
    Legend::Open,
    // Down
    Legend::Open,
    Legend::Wall,
    Legend::Open,
    // Right
    Legend::Wall,
    Legend::Wall,
    Legend::Wall,
    // Up
    Legend::Wall,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(CubeFace::Up, Coord { x: 1, y: 1 }, Heading::Left),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Up
    Legend::Wall,
    // Right
    Legend::Wall,
    Legend::Wall,
    Legend::Wall,
    // Down
    Legend::Open,
    Legend::Wall,
    Legend::Open,
    // Left
    Legend::Open,
    Legend::Wall,
    Legend::Wall,
    // Up
    Legend::Open,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(CubeFace::Up, Coord { x: 1, y: 1 }, Heading::Right),
    Some(expected_ring_values)
  );

  // Back
  expected_ring_values = vec![
    // Back
    Legend::Wall,
    // Up
    Legend::Wall,
    Legend::Open,
    Legend::Wall,
    // Front
    Legend::Open,
    Legend::Wall,
    Legend::Open,
    // Down
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Back
    Legend::Open,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(CubeFace::Back, Coord { x: 0, y: 1 }, Heading::Up),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Back
    Legend::Open,
    // Down
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Front
    Legend::Open,
    Legend::Wall,
    Legend::Open,
    // Up
    Legend::Wall,
    Legend::Open,
    Legend::Wall,
    // Back
    Legend::Wall,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(
      CubeFace::Back,
      Coord { x: 0, y: 1 },
      Heading::Down
    ),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Back
    // Right
    Legend::Wall,
    Legend::Wall,
    Legend::Wall,
    // Front
    Legend::Open,
    Legend::Open,
    Legend::Wall,
    // Left
    Legend::Open,
    Legend::Wall,
    Legend::Open,
    // Back
    Legend::Open,
    Legend::Open,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(
      CubeFace::Back,
      Coord { x: 0, y: 1 },
      Heading::Left
    ),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Back
    Legend::Open,
    Legend::Open,
    // Left
    Legend::Open,
    Legend::Wall,
    Legend::Open,
    // Front
    Legend::Wall,
    Legend::Open,
    Legend::Open,
    // Right
    Legend::Wall,
    Legend::Wall,
    Legend::Wall,
    // Back
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(
      CubeFace::Back,
      Coord { x: 0, y: 1 },
      Heading::Right
    ),
    Some(expected_ring_values)
  );

  // Front
  expected_ring_values = vec![
    // Front
    Legend::Open,
    // Up
    Legend::Wall,
    Legend::Wall,
    Legend::Wall,
    // Back
    Legend::Wall,
    Legend::Open,
    Legend::Wall,
    // Down
    Legend::Wall,
    Legend::Open,
    Legend::Open,
    // Front
    Legend::Open,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(CubeFace::Front, Coord { x: 2, y: 1 }, Heading::Up),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Front
    Legend::Open,
    // Down
    Legend::Open,
    Legend::Open,
    Legend::Wall,
    // Back
    Legend::Wall,
    Legend::Open,
    Legend::Wall,
    // Up
    Legend::Wall,
    Legend::Wall,
    Legend::Wall,
    // Front
    Legend::Open,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(
      CubeFace::Front,
      Coord { x: 2, y: 1 },
      Heading::Down
    ),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Front
    Legend::Open,
    Legend::Wall,
    // Left
    Legend::Open,
    Legend::Wall,
    Legend::Open,
    // Back
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Right
    Legend::Wall,
    Legend::Wall,
    Legend::Wall,
    // Front
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(
      CubeFace::Front,
      Coord { x: 2, y: 1 },
      Heading::Left
    ),
    Some(expected_ring_values)
  );

  expected_ring_values = vec![
    // Front
    // Right
    Legend::Wall,
    Legend::Wall,
    Legend::Wall,
    // Back
    Legend::Open,
    Legend::Open,
    Legend::Open,
    // Left
    Legend::Open,
    Legend::Wall,
    Legend::Open,
    // Front
    Legend::Wall,
    Legend::Open,
    Legend::Open,
  ];
  assert_eq!(
    cube.get_cube_face_ring(
      CubeFace::Front,
      Coord { x: 2, y: 1 },
      Heading::Right
    ),
    Some(expected_ring_values)
  );
}
