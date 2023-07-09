pub mod prelude {
  use indexmap::IndexSet;
  use lazy_static::lazy_static;
  use std::fmt;


  pub const CHAMBER_WIDTH: usize = 7;
  pub const ORIGIN_OFFSET_Y: usize = 3;
  pub const ORIGIN_OFFSET_X: usize = 2;

  pub enum AirDirection {
    Port,
    Starboard,
  }

  impl AirDirection {
    pub fn from_char(pattern: char) -> AirDirection {
      match pattern {
        '<' => AirDirection::Port,
        '>' => AirDirection::Starboard,
        _ => unimplemented!(),
      }
    }
  }
  #[derive(Default)]
  pub struct CycleDetector {
    tracked_values: Vec<usize>,            // index
    tracked_heights: Vec<usize>,           // height
    tracked_shapes: Vec<u64>,              // shape count
    cycle_candidates: Vec<(usize, usize)>, // start & end indices
  }

  impl CycleDetector {
    pub fn step(
      &mut self,
      height: usize,
      shapes_count: u64,
      index: usize,
    ) -> Option<(usize, usize, usize, u64)> {
      let next_i = self.tracked_values.len();
      for (i, _value) in self.tracked_values.iter().enumerate() {
        self.cycle_candidates.push((i, next_i));
      }
      self.tracked_values.push(index);
      self.tracked_heights.push(height);
      self.tracked_shapes.push(shapes_count);

      for (i, j) in &self.cycle_candidates {
        if *j + (*j - *i) <= next_i {
          let mut candidate_cycled = true;
          for k in *j..*j + (*j - *i) {
            if self.tracked_values[k] != self.tracked_values[k - *j + *i]
              || (k - *j + *i > 0
                && self.tracked_heights[k] - self.tracked_heights[k - 1]
                  != self.tracked_heights[k - *j + *i]
                    - self.tracked_heights[k - *j + *i - 1])
            {
              candidate_cycled = false;
              break;
            }
          }

          if candidate_cycled {
            return Some((
              *i,
              *j,
              self.tracked_heights[*j] - self.tracked_heights[*i],
              self.tracked_shapes[*j] - self.tracked_shapes[*i],
            ));
          }
        }
      }

      None
    }
  }

  #[derive(Clone)]
  pub struct Chamber(pub Vec<[bool; CHAMBER_WIDTH]>);

  impl fmt::Debug for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let rows: Vec<String> = self
        .0
        .iter()
        .rev()
        .map(|row| {
          row
            .iter()
            .map(|c| if *c { "#" } else { "." })
            .collect::<Vec<_>>()
            .join("")
        })
        .collect();

      write!(f, "Chamber\n{}", rows.join("\n"))
    }
  }

  impl Chamber {
    pub fn append_shape(
      &mut self,
      current_pos: &(usize, usize),
      shape: &Shape,
    ) {
      // dbg!(&self, shape, current_pos);
      let last = self.0.len() - 1;
      for (i, row) in shape.points.iter().rev().enumerate() {
        let mut new_row = [false; CHAMBER_WIDTH];
        let start = current_pos.0;
        let end = current_pos.0 + row.len();
        new_row[start..end].copy_from_slice(row);
        if current_pos.1 + i > last {
          self.0.push(new_row);
        } else {
          new_row = merge_rows(&new_row, unsafe {
            self.0.get_unchecked(current_pos.1 + i)
          });
          self.0[current_pos.1 + i] = new_row;
        }
      }
    }

    pub fn get_rise(&self) -> [Option<usize>; CHAMBER_WIDTH] {
      let mut result = [None; CHAMBER_WIDTH];

      for col in 0..CHAMBER_WIDTH {
        for (row_index, row) in self.0.iter().rev().enumerate() {
          if row[col] {
            result[col] = Some(self.0.len() - row_index - 1);
            break;
          }
        }
      }

      result
    }
  }

  #[derive(PartialEq, Eq, Hash)]
  pub struct Shape {
    pub points: Vec<Vec<bool>>,
  }

  impl fmt::Debug for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let rows: Vec<String> = self
        .points
        .iter()
        .map(|row| {
          row
            .iter()
            .map(|c| if *c { "@" } else { "." })
            .collect::<Vec<_>>()
            .join("")
        })
        .collect();

      write!(f, "Shape\n{}", rows.join("\n"))
    }
  }

  impl Shape {
    pub fn get_descent(&self) -> Vec<Option<usize>> {
      let width = self.points[0].len();
      let mut result = vec![None; width];

      for col in 0..width {
        for (row_index, row) in self.points.iter().rev().enumerate() {
          if row[col] {
            result[col] = Some(row_index);
            break;
          }
        }
      }

      result
    }
  }

  lazy_static! {
    /*
      .... ... ... #. ..
      .... .#. ..# #. ..
      .... ### ..# #. ##
      #### .#. ### #. ##
    */
    pub static ref SHAPES: IndexSet<Shape> = {
      let mut shapes = IndexSet::with_capacity(5);
      #[allow(non_snake_case)]
      let T = true;
      #[allow(non_snake_case)]
      let F = false;
      shapes.insert(Shape {
        points: vec![vec![T; 4]],
      });
      shapes.insert(Shape {
        points: vec![
          vec![F,T,F],
          vec![T,T,T],
          vec![F,T,F],
          ],
      });
      shapes.insert(Shape {
        points: vec![
          vec![F,F,T],
          vec![F,F,T],
          vec![T,T,T]
          ],
      });
      shapes.insert(Shape {
        points: vec![vec![T]; 4],
      });
      shapes.insert(Shape {
        points: vec![vec![T, T]; 2],
      });

      shapes
    };

  }

  fn merge_rows<'a>(
    left: &'a [bool; CHAMBER_WIDTH],
    right: &'a [bool; CHAMBER_WIDTH],
  ) -> [bool; CHAMBER_WIDTH] {
    let mut merged_row = [false; CHAMBER_WIDTH];

    for i in 0..CHAMBER_WIDTH {
      if left[i] && right[i] {
        panic!("Merge conflict at column {}: {:?}, {:?}", i, left, right);
      } else {
        merged_row[i] = left[i] || right[i];
      }
    }

    merged_row
  }
}

use prelude::*;

pub fn simulate(
  flow_input: &[char],
  shapes_count: u64,
  skip_cycle_padding: bool,
) -> (Chamber, u64) {
  let mut detector = CycleDetector::default();
  let mut air_flow = flow_input.iter().cycle();
  let s_l = SHAPES.len() as u64;
  let f_l = flow_input.len() as u64;
  let mut chamber: Chamber = Chamber(vec![[false; CHAMBER_WIDTH]]);
  let mut current_shape: Option<&Shape> = None;
  let mut current_pos: (usize, usize) = (0, 0);
  let mut i = 0;
  let mut j = 0;
  let mut height_adjust = 0;
  let mut height = 0;
  loop {
    if current_shape.is_none() {
      if j == shapes_count {
        height = chamber
          .get_rise()
          .iter()
          .fold(0, |acc, o| acc.max(o.unwrap_or(0)));
        break;
      }
      current_shape = SHAPES.get_index((j % s_l) as usize);
      current_pos = (ORIGIN_OFFSET_X, get_origin_y(&chamber));
      j += 1;
    }
    let shape = current_shape.unwrap();

    // eprintln!("(x{},y{})", current_pos.0, current_pos.1);
    let direction = air_flow.next().unwrap();
    offset_x(direction, shape, &mut current_pos, &chamber);

    // eprintln!("flow {} (x{},y{})", direction, current_pos.0, current_pos.1);
    if !descent_y(shape, &mut current_pos, &chamber) {
      // eprintln!(
      //   "(x{},y{}) vs {:?}",
      //   current_pos.0,
      //   current_pos.1,
      //   get_limit(shape, &current_pos)
      // );

      chamber.append_shape(&current_pos, shape);
      // eprintln!(
      //   "height at placing shape {j}: {}",
      //   chamber.get_rise().iter().fold(0, |acc, cur| {
      //     if let Some(y) = cur {
      //       acc.max(y.to_owned())
      //     } else {
      //       acc
      //     }
      //   }) + 1
      // );
      current_shape = None;
    }

    // detect cycle
    if i % f_l == 0 && height_adjust == 0 {
      if let Some((_cycle_start, _cycle_end, factor, factor_shapes)) = detector
        .step(
          chamber
            .get_rise()
            .iter()
            .fold(0, |acc, o| acc.max(o.unwrap_or(0))),
          j,
          (j % s_l) as usize,
        )
      {
        let target = shapes_count - j;
        let cycles = target / factor_shapes;
        height_adjust += factor as u64 * cycles;

        j += cycles * factor_shapes;
        eprintln!(
          "cycle found. {} +{height_adjust} @ {cycles} cycles",
          chamber
            .get_rise()
            .iter()
            .fold(0, |acc, o| acc.max(o.unwrap_or(0)))
        );
      }
      eprintln!("{} - {} {}", i, j, j % s_l);
    }
    i += 1;
  }

  if height_adjust > 0 && !skip_cycle_padding {
    for _ in 0..height_adjust as usize {
      chamber.0.push([true; CHAMBER_WIDTH]);
    }
  }

  (chamber, height as u64 + height_adjust + 1)
}

/* Each rock appears so that its left edge is two units away from the left wall and its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one). */
fn get_origin_y(chamber: &Chamber) -> usize {
  let mut max_rise: Option<usize> = None;
  chamber.get_rise().iter().for_each(|o| {
    if let Some(y) = o {
      max_rise = match max_rise {
        Some(r) => Some(r.max(*y + 1)),
        None => Some(*y + 1),
      };
    }
  });

  max_rise.unwrap_or(0) + ORIGIN_OFFSET_Y
}

/* these offset and descend methods could use a common collision detection system with rotation but it is more computationally efficient to do it this way. my simulation is linear within the loop, I'm not budging. */

fn offset_x(
  direction: &char,
  shape: &Shape,
  current_pos: &mut (usize, usize),
  chamber: &Chamber,
) {
  let chamber_height = chamber.0.len() - 1;
  match AirDirection::from_char(*direction) {
    AirDirection::Port => {
      if current_pos.0 == 0 {
        return;
      }
      if current_pos.1 > chamber_height // if shape is completely above, it cannot collide
        || !shape.points.iter().rev().enumerate().any(|(i, row)| /* or if !any cell collides */ {
          if current_pos.1 + i > chamber_height { // if this row of the shape is above, it cannot collide
            return false;
          }
          if let Some(row_pos) = row.iter().position(|x| *x) {
            return chamber.0[current_pos.1 + i][current_pos.0 + row_pos - 1];
          }
          false // if we never encountered a point in this row, it cannot collide
        })
      {
        current_pos.0 -= 1;
      }
    }
    AirDirection::Starboard => {
      if current_pos.0 + shape.points[0].len() == CHAMBER_WIDTH {
        return;
      }
      if current_pos.1 > chamber_height
        || !shape.points.iter().rev().enumerate().any(|(i, row)| /* any shape's row */ {
          if current_pos.1 + i > chamber_height {
            return false; // if it is above the chamber, we will not collide
          }
          if let Some(pos) = row.iter().rev().position(|x| *x) /* has a positive rightmost position */ {
            let row_pos = (shape.points[0].len() - 1) - pos;
            // and the position right of it in the chamber is also positive
            return chamber.0[current_pos.1 + i][current_pos.0 + row_pos + 1];
          }
          false
        })
      {
        current_pos.0 += 1;
      }
    }
  }
}

fn descent_y(
  shape: &Shape,
  current_pos: &mut (usize, usize),
  chamber: &Chamber,
) -> bool {
  if current_pos.1 > chamber.0.len() {
    current_pos.1 -= 1;
    return true;
  } else if current_pos.1 == 0 {
    return false;
  }

  let limit = get_limit(shape, current_pos);

  let can_descend = !limit.iter().enumerate().any(|(x, oy)| {
    if let Some(y) = oy {
      if *y > chamber.0.len() {
        return false;
      }
      return *y == 0 || chamber.0[y - 1][x];
    }
    false
  });

  if can_descend {
    current_pos.1 -= 1;
  }

  can_descend
}

fn get_limit(
  shape: &Shape,
  current_pos: &(usize, usize),
) -> Vec<Option<usize>> {
  let mut limit: Vec<Option<usize>> = shape
    .get_descent()
    .iter()
    .map(|o| o.as_ref().map(|y| current_pos.1 + y))
    .collect();
  for index in 0..current_pos.0 {
    limit.insert(index, None);
  }
  for _ in limit.len()..CHAMBER_WIDTH {
    limit.push(None);
  }

  limit
}
