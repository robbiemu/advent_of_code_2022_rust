use sscanf::sscanf;
use std::collections::VecDeque;


#[derive(Clone)]
pub struct Monkey {
  pub test: i32,
  pub result: (usize, usize),
  pub operation: (char, i32),
}

impl Monkey {
  pub fn input_props_from(record: Vec<String>) -> (VecDeque<i32>, Monkey) {
    let invalid_format_error =
      &format!("invalid record format for monkey:\n{}", record.join("\n"));
    assert!(record.len() > 5, "{}", invalid_format_error);

    let (_, items_str) = sscanf!(record[1], r"{str:/[^:]+:\s+/}{str}")
      .expect(invalid_format_error);
    let items = items_str
      .trim()
      .split(", ")
      .map(|v| v.parse::<i32>().unwrap())
      .collect();

    let o = record[2]
      .split_whitespace()
      .collect::<Vec<&str>>()
      .as_slice()
      .chunks(2)
      .last()
      .unwrap_or(&[])
      .to_vec();
    let operation = (
      o[0].chars().next().unwrap(),
      o[1].parse::<i32>().unwrap_or(i32::MAX),
    );
    let (_, test) = sscanf!(record[3], r"{str:/.*\s/}{i32:/\d+$/}")
      .expect(invalid_format_error);
    let result = (
      sscanf!(record[5], r"{str:/.*\s/}{usize:/\d+$/}")
        .expect(invalid_format_error)
        .1,
      sscanf!(record[4], r"{str:/.*\s/}{usize:/\d+$/}")
        .expect(invalid_format_error)
        .1,
    );

    (items, Monkey { test, result, operation })
  }
}

pub fn convert_vec_to_vec_and_vec<T, U>(vec: Vec<(T, U)>) -> (Vec<T>, Vec<U>) {
  let mut vec_deques = Vec::new();
  let mut monkeys = Vec::new();

  for (vec_deque, monkey) in vec {
    vec_deques.push(vec_deque);
    monkeys.push(monkey);
  }

  (vec_deques, monkeys)
}

const MODULUS: i32 = 46340; // Square root of i32::MAX
pub fn modular_reduction(mut value: i32) -> i32 {
  // Apply modulus operation
  value %= MODULUS;

  // Apply signed modular arithmetic
  let midpoint = MODULUS / 2;
  if value >= midpoint {
    value -= MODULUS;
  }

  value
}

pub fn inv_heaviside(x: i32) -> i32 {
  if x <= 0 {
    1
  } else {
    0
  }
}
