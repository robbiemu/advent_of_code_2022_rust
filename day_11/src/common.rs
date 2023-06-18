use sscanf::sscanf;
use std::collections::{BinaryHeap, VecDeque};


#[derive(Clone)]
pub struct Monkey {
  pub test: i64,
  pub result: (usize, usize),
  pub operation: (char, i64),
}

impl Monkey {
  pub fn input_props_from(record: Vec<String>) -> (VecDeque<i64>, Monkey) {
    let invalid_format_error =
      &format!("invalid record format for monkey:\n{}", record.join("\n"));
    assert!(record.len() > 5, "{}", invalid_format_error);

    let (_, items_str) = sscanf!(record[1], r"{str:/[^:]+:\s+/}{str}")
      .expect(invalid_format_error);
    let items = items_str
      .trim()
      .split(", ")
      .map(|v| v.parse::<i64>().unwrap())
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
      o[1].parse::<i64>().unwrap_or(i64::MAX),
    );
    let test = sscanf!(record[3], r"{str:/.*\s/}{i64:/\d+$/}")
      .expect(invalid_format_error)
      .1;
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

  pub fn inspect_items<R>(
    &mut self,
    items: &VecDeque<i64>,
    relief: R,
  ) -> Vec<(usize, i64)>
  where
    R: Fn(i64) -> i64,
  {
    let mut actions: Vec<(usize, i64)> = Vec::new();
    for item in items.iter() {
      let updated_item = match self.operation.0 {
        '+' => item + self.operation.1,
        '*' => {
          let r = match self.operation.1 {
            i64::MAX => *item,
            x => x,
          };
          item * r
        }
        _ => unimplemented!(),
      };
      let updated_item = relief(updated_item);
      let to = match inv_heaviside(updated_item % self.test) {
        0 => self.result.0,
        1 => self.result.1,
        _ => unreachable!(),
      };
      actions.push((to, updated_item));
    }

    actions
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

pub fn factory_ordered_troup_tallies<R>(
  mut monkeys: Vec<Monkey>,
  mut items: Vec<VecDeque<i64>>,
  rounds: usize,
  relief: R,
) -> BinaryHeap<usize>
where
  R: Fn(i64) -> i64 + Copy,
{
  let rates = (0..rounds).fold(vec![0; monkeys.len()], |mut acc, _| {
    for (index, monkey) in monkeys.iter_mut().enumerate() {
      let actions = monkey.inspect_items(&items[index], relief);
      acc[index] += items[index].len();
      actions.iter().for_each(|(to, value)| {
        println!("@{}  {}:{}", index, to, *value);
        items[*to].push_front(*value);
      });
      items[index].clear();
    }
    dbg!(acc.clone());
    acc
  });

  BinaryHeap::from(rates)
}

pub fn inv_heaviside(x: i64) -> i64 {
  if x <= 0 {
    1
  } else {
    0
  }
}
