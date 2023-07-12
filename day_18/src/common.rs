pub mod prelude {
  pub type Position = (usize, usize, usize);
}

use ndarray::{prelude::*, OwnedRepr};

pub fn get_neighbors(
  position: (usize, usize, usize),
  space: &ArrayBase<OwnedRepr<bool>, Dim<[usize; 3]>>,
) -> Vec<(usize, usize, usize)> {
  let (x, y, z) = position;
  let dim = space.dim();
  let x_len = dim.0;
  let y_len = dim.1;
  let z_len = dim.2;

  let mut neighbors = vec![(x + 1, y, z), (x, y + 1, z), (x, y, z + 1)];
  if x > 0 {
    neighbors.push((x - 1, y, z));
  }
  if y > 0 {
    neighbors.push((x, y - 1, z));
  }
  if z > 0 {
    neighbors.push((x, y, z - 1));
  }

  neighbors
    .into_iter()
    .filter(|&(i, j, k)| i < x_len && j < y_len && k < z_len)
    .collect()
}
