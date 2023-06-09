pub fn rotate_2d_collection<T, C>(collection: &C) -> Vec<Vec<T>>
where
T: Clone + Default,
C: AsRef<[Vec<T>]>,
{
  let rows = collection.as_ref().len();
  let columns = collection.as_ref().get(0).map_or(0, |row| row.len());
  
  let mut rotated = vec![vec![Default::default(); rows]; columns];
  
  for (i, row) in collection.as_ref().iter().enumerate() {
    for (j, item) in row.iter().enumerate() {
      rotated[j][i] = item.clone();
    }
  }
  
  rotated
}
