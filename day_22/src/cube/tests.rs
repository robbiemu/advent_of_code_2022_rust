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
    mapping.get_2d_adjacencies(Coord { x: 0, y: 0 }),
    [None, Some(1), None, None]
  );
  assert_eq!(
    mapping.get_2d_adjacencies(Coord { x: 0, y: 1 }),
    [Some(2), None, None, Some(0)]
  );
  assert_eq!(
    mapping.get_2d_adjacencies(Coord { x: 1, y: 1 }),
    [Some(3), None, Some(1), None]
  );
  assert_eq!(
    mapping.get_2d_adjacencies(Coord { x: 2, y: 1 }),
    [None, Some(4), Some(2), None]
  );
}


// MARK Cube
use crate::common::extract_board_and_turns_from_stream;

#[test]
fn test_cube_initialization() {
  let inputs = [
    include_str!("../../sample.txt"),
    include_str!("../../sample_rotated.txt"),
    include_str!("../../mini_input.txt"),
    include_str!("../../sample_2.txt"),
    include_str!("../../sample_2_rotated.txt"),
  ];

  for input in inputs {
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

    // Test add_to and add_face
    assert_eq!(cube.faces.len(), 6);
    assert_eq!(cube.face_indices.len(), 6);
  }
}

#[test]
fn test_determine_rotation() {
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

  // Test determine_rotation
  assert_eq!(
    cube.determine_rotation(CubeFace::Front, Heading::Left),
    Some((0, false))
  );
  assert_eq!(
    cube.determine_rotation(CubeFace::Front, Heading::Up),
    Some((0, false))
  );

  assert_eq!(cube.determine_rotation(CubeFace::Down, Heading::Left), None);
  assert_eq!(
    cube.determine_rotation(CubeFace::Down, Heading::Up),
    Some((0, false))
  );

  assert_eq!(
    cube.determine_rotation(CubeFace::Left, Heading::Left),
    Some((3, false))
  );
  assert_eq!(cube.determine_rotation(CubeFace::Left, Heading::Up), None);

  assert_eq!(cube.determine_rotation(CubeFace::Up, Heading::Left), None);
  assert_eq!(
    cube.determine_rotation(CubeFace::Up, Heading::Up),
    Some((2, false))
  );

  assert_eq!(
    cube.determine_rotation(CubeFace::Back, Heading::Left),
    Some((2, false))
  );
  assert_eq!(
    cube.determine_rotation(CubeFace::Back, Heading::Up),
    Some((0, false))
  );

  assert_eq!(
    cube.determine_rotation(CubeFace::Right, Heading::Left),
    Some((2, true))
  );
  assert_eq!(cube.determine_rotation(CubeFace::Right, Heading::Up), None);


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

  // Test determine_rotation
  assert_eq!(
    cube.determine_rotation(CubeFace::Front, Heading::Left),
    Some((0, false))
  );
  assert_eq!(
    cube.determine_rotation(CubeFace::Front, Heading::Up),
    Some((0, false))
  );

  assert_eq!(cube.determine_rotation(CubeFace::Down, Heading::Left), None);
  assert_eq!(
    cube.determine_rotation(CubeFace::Down, Heading::Up),
    Some((0, false))
  );

  assert_eq!(
    cube.determine_rotation(CubeFace::Back, Heading::Left),
    Some((2, false))
  );
  assert_eq!(
    cube.determine_rotation(CubeFace::Back, Heading::Up),
    Some((0, false))
  );

  assert_eq!(
    cube.determine_rotation(CubeFace::Left, Heading::Left),
    Some((2, true))
  );
  assert_eq!(cube.determine_rotation(CubeFace::Left, Heading::Up), None);

  assert_eq!(cube.determine_rotation(CubeFace::Up, Heading::Left), None);
  assert_eq!(
    cube.determine_rotation(CubeFace::Up, Heading::Up),
    Some((1, true))
  );
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
    cube.get_cube_face_values(CubeFace::Front, Heading::Right),
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
    cube.get_cube_face_values(CubeFace::Back, Heading::Right),
    Some(
      [
        [Open, Open, Open, Open],
        [Open, Open, Wall, Open],
        [Open, Open, Open, Open],
        [Wall, Open, Open, Open]
      ]
      .iter()
      .map(|r| r.to_vec())
      .collect()
    )
  );

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Back, Heading::Up),
    Some(
      [
        [Open, Open, Open, Wall],
        [Open, Open, Open, Open],
        [Open, Wall, Open, Open],
        [Open, Open, Open, Open]
      ]
      .iter()
      .map(|r| r.to_vec())
      .collect()
    )
  );

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Right, Heading::Right),
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
    cube.get_cube_face_values(CubeFace::Left, Heading::Right),
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
    cube.get_cube_face_values(CubeFace::Down, Heading::Down),
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
    cube.get_cube_face_values(CubeFace::Up, Heading::Down),
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
    cube.get_cube_face_values(CubeFace::Front, Heading::Right),
    Some(
      [[Open, Open, Open], [Wall, Open, Wall], [Open, Open, Open]]
        .iter()
        .map(|r| r.to_vec())
        .collect()
    )
  );

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Back, Heading::Right),
    Some(
      [[Wall, Open, Wall], [Open, Open, Open], [Wall, Open, Wall]]
        .iter()
        .map(|r| r.to_vec())
        .collect()
    )
  );

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Back, Heading::Up),
    Some(
      [[Wall, Open, Wall], [Open, Open, Open], [Wall, Open, Wall]]
        .iter()
        .map(|r| r.to_vec())
        .collect()
    )
  );

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Left, Heading::Right),
    Some(
      [[Wall, Open, Wall], [Open, Wall, Open], [Wall, Open, Wall],]
        .iter()
        .map(|r| r.to_vec())
        .collect()
    )
  );

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Down, Heading::Down),
    Some(
      [[Wall, Open, Open], [Open, Wall, Open], [Open, Open, Wall]]
        .iter()
        .map(|r| r.to_vec())
        .collect()
    )
  );

  assert_eq!(
    cube.get_cube_face_values(CubeFace::Up, Heading::Down),
    Some(
      [[Wall, Open, Wall], [Wall, Open, Wall], [Wall, Open, Wall]]
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

  // Test get_cube_face_ring
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
}
