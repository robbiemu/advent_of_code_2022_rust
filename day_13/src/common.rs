use std::cmp::Ordering;


pub fn compare(left: &[u8], right: &[u8]) -> Ordering {
  match (left.len(), right.len()) {
    (0, 0) => Ordering::Equal,
    (0, _) => Ordering::Less,
    (_, 0) => Ordering::Greater,
    _ => {
      let mut li = 0;
      let mut ri = 0;
      let mut l_comp = left[0];
      let mut r_comp = right[0];
      if left.get(0..2) == Some(&[49, 48]) {
        li = 1;
        l_comp = b'a'; // placeholder for b"10", which is the maximum in the input
      }

      if right.get(0..2) == Some(&[49, 48]) {
        ri = 1;
        r_comp = b'a';
      }
      match (l_comp, r_comp) {
        (l, r) if l == r => compare(&left[li + 1..], &right[ri + 1..]),
        // now a & b are never equal:
        (l, b'[') => {
          let converted_left = [&[l, b']'], &left[li + 1..]].concat();

          compare(&converted_left, &right[ri + 1..])
        }
        (_, b']') => Ordering::Greater,
        (b'[', r) => {
          let converted_right = [&[r, b']'], &right[ri + 1..]].concat();

          compare(&left[li + 1..], &converted_right)
        }
        (b']', _) => Ordering::Less,
        (l, r) => l.cmp(&r), // a comma is 4 less than b'0'
      }
    }
  }
}
