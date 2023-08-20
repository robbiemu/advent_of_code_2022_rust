fn get_rotation(
  &self,
  a_face: &CubeFace,
  b_face: &CubeFace,
  c_face: &CubeFace,
) -> Option<isize> {
  /* return the number of clockwise or counterclockwise rotations to align with front */
  let mapping = self.mapping.clone()?.0;
  let len_x = mapping[0].len();
  let len_y = mapping.len();
  let a = &self.get_face(a_face)?.position;
  let b = &self.get_face(b_face)?.position;
  let c = &self.get_face(c_face)?.position;
  let first_oriented_move = OrientedMappingMove::from_coordinates(a, b).ok()?;
  let second_oriented_move =
    OrientedMappingMove::from_coordinates(b, c).ok()?;
  eprintln!("{:?} {:?}", first_oriented_move, second_oriented_move);

  match (first_oriented_move, second_oriented_move) {
    (OrientedMappingMove::Horizontal(h), OrientedMappingMove::Vertical(v))
      if h == 0 || v == 0 =>
    {
      Some(0)
    }
    (OrientedMappingMove::Vertical(v), OrientedMappingMove::Horizontal(h))
      if h == 0 || v == 0 =>
    {
      Some(0)
    }
    (OrientedMappingMove::Vertical(_), OrientedMappingMove::Vertical(_)) => {
      Some(0)
    }
    (
      OrientedMappingMove::Horizontal(_),
      OrientedMappingMove::Horizontal(_),
    ) => Some(0),
    (OrientedMappingMove::Horizontal(h), OrientedMappingMove::Vertical(v))
      if h > 0 && v > 0 =>
    {
      if b.x + 1 < len_x && mapping[b.y][b.x + 1] == Legend::Space {
        Some(-2)
      } else if b.x > 0
        && b.y + 1 < len_y
        && mapping[b.y + 1][b.x - 1] == Legend::Space
      {
        Some(1)
      } else {
        None
      }
    }
    (OrientedMappingMove::Vertical(v), OrientedMappingMove::Horizontal(h))
      if h > 0 && v > 0 =>
    {
      if b.y + 1 < len_y
        && b.x + 1 < len_x
        && mapping[b.y + 1][b.x] == Legend::Space
      {
        Some(2)
      } else if b.y > 0
        && b.x + 1 < len_x
        && mapping[b.y - 1][b.x + 1] == Legend::Space
      {
        Some(-1)
      } else {
        None
      }
    }
    (OrientedMappingMove::Horizontal(h), OrientedMappingMove::Vertical(v))
      if h < 0 && v > 0 =>
    {
      if b.x > 0 && b.y + 1 > len_y && mapping[b.y][b.x - 1] == Legend::Space {
        Some(2)
      } else if b.y + 1 < len_y
        && b.x + 1 < len_x
        && mapping[b.y + 1][b.x + 1] == Legend::Space
      {
        Some(-1)
      } else {
        None
      }
    }
    (OrientedMappingMove::Vertical(v), OrientedMappingMove::Horizontal(h))
      if h < 0 && v > 0 =>
    {
      if b.y + 1 < len_y && b.x > 0 && mapping[b.y + 1][b.x] == Legend::Space {
        Some(-2)
      } else if b.y > 0 && b.x > 0 && mapping[b.y - 1][b.x - 1] == Legend::Space
      {
        Some(1)
      } else {
        None
      }
    }
    (OrientedMappingMove::Horizontal(h), OrientedMappingMove::Vertical(v))
      if h > 0 && v < 0 =>
    {
      if b.x + 1 < len_x && b.y > 0 && mapping[b.y][b.x + 1] == Legend::Space {
        Some(2)
      } else if b.y > 0 && b.x > 0 && mapping[b.y - 1][b.x - 1] == Legend::Space
      {
        Some(-1)
      } else {
        None
      }
    }
    (OrientedMappingMove::Vertical(v), OrientedMappingMove::Horizontal(h))
      if h > 0 && v < 0 =>
    {
      if b.y > 0 && b.x + 1 < len_x && mapping[b.y - 1][b.x] == Legend::Space {
        Some(-2)
      } else if b.y + 1 < len_y
        && b.x + 1 < len_x
        && mapping[b.y + 1][b.x + 1] == Legend::Space
      {
        Some(1)
      } else {
        None
      }
    }
    (OrientedMappingMove::Horizontal(h), OrientedMappingMove::Vertical(v))
      if h < 0 && v < 0 =>
    {
      if b.x > 0 && b.y > 0 && mapping[b.y][b.x - 1] == Legend::Space {
        Some(-2)
      } else if b.y > 0
        && b.x + 1 < len_x
        && mapping[b.y - 1][b.x + 1] == Legend::Space
      {
        Some(1)
      } else {
        None
      }
    }
    (OrientedMappingMove::Vertical(v), OrientedMappingMove::Horizontal(h))
      if h < 0 && v < 0 =>
    {
      if b.y > 0 && b.x > 0 && mapping[b.y - 1][b.x] == Legend::Space {
        Some(2)
      } else if b.x > 0
        && b.y + 1 < len_y
        && mapping[b.y + 1][b.x - 1] == Legend::Space
      {
        Some(-1)
      } else {
        None
      }
    }
    _ => None,
  }
}
