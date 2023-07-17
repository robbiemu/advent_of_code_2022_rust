const INDICES: [i64; 3] = [1000, 2000, 3000];

pub fn get_coordinates(codex: Vec<i64>) -> [i64; 3] {
  let len = codex.len();
  let zero = codex
    .iter()
    .position(|x| *x == 0)
    .unwrap_or_else(|| panic!("no zero in input!"));

  INDICES
    .iter()
    .map(|x| codex[(zero + *x as usize) % len])
    .collect::<Vec<i64>>()
    .try_into()
    .unwrap()
}
