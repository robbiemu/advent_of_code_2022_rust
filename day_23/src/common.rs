use std::cell::RefCell;
use std::rc::Rc;

pub fn extend_vector_up<T: Clone + Default>(
  grid: Rc<RefCell<Vec<Vec<T>>>>,
  num_rows: usize,
) {
  let num_cols = grid.borrow()[0].len();
  let new_row = vec![Default::default(); num_cols];
  for _ in 0..num_rows {
    let mut grid_ref = grid.borrow_mut();
    grid_ref.insert(0, new_row.clone());
  }
}

pub fn extend_vector_down<T: Clone + Default>(
  grid: Rc<RefCell<Vec<Vec<T>>>>,
  num_rows: usize,
) {
  let num_cols = grid.borrow()[0].len();
  let new_row = vec![Default::default(); num_cols];
  for _ in 0..num_rows {
    let mut grid_ref = grid.borrow_mut();
    grid_ref.push(new_row.clone());
  }
}

pub fn extend_vector_left<T: Clone + Default>(
  grid: Rc<RefCell<Vec<Vec<T>>>>,
  num_cols: usize,
) {
  let mut grid_ref = grid.borrow_mut();
  for row in grid_ref.iter_mut() {
    let new_elements = vec![Default::default(); num_cols];
    row.splice(..0, new_elements);
  }
}

pub fn extend_vector_right<T: Clone + Default>(
  grid: Rc<RefCell<Vec<Vec<T>>>>,
  num_cols: usize,
) {
  let mut grid_ref = grid.borrow_mut();
  for row in grid_ref.iter_mut() {
    let new_elements = vec![Default::default(); num_cols];
    row.extend(new_elements);
  }
}
