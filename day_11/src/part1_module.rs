use std::collections::BinaryHeap;
use std::collections::VecDeque;

use super::common::*;
use super::problem_solver::ProblemSolver;


const ROUNDS: usize = 20;
const RELIEF: i32 = 3;

pub struct PSInput {
  monkeys: Vec<Monkey>,
  items: Vec<VecDeque<i32>>, // separate because iterating each monkey we must mutate each other's items. assignment there requires a double borrow if from monkey.items
}

pub struct PSSolution {
  monkey_business: i32,
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let (items, monkeys): (Vec<VecDeque<i32>>, Vec<Monkey>) =
      convert_vec_to_vec_and_vec(
        lines
          .collect::<Vec<String>>()
          .chunks(7)
          .map(|record| Monkey::input_props_from(record.to_vec()))
          .collect(),
      );

    PSInput { monkeys, items }
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let mut monkeys = input.monkeys;
    let mut items = input.items;

    let rates = (0..ROUNDS).fold(vec![0; monkeys.len()], |mut acc, _| {
      // Move closure
      for (index, monkey) in monkeys.iter_mut().enumerate() {
        let actions = monkey.inspect_items(&items[index], RELIEF);
        acc[index] += items[index].len();
        actions.iter().for_each(|(to, value)| {
          println!("@{}  {}:{}", index, to, *value);
          items[*to].push_front(*value);
          items[index].pop_back();
        });
        items[index].clear();
      }
      dbg!(acc.clone());
      acc
    });
    let mut orderly_troup = BinaryHeap::from(rates);
    let monkey_business =
      (orderly_troup.pop().unwrap() * orderly_troup.pop().unwrap()) as i32;

    PSSolution { monkey_business }
  }

  fn output(solution: Self::Solution) {
    dbg!(solution.monkey_business);
  }
}

impl Monkey {
  fn inspect_items(
    &mut self,
    items: &VecDeque<i32>,
    with_relief: i32,
  ) -> Vec<(usize, i32)> {
    let mut actions: Vec<(usize, i32)> = Vec::new();
    for item in items.iter() {
      let updated_item = match self.operation.0 {
        '+' => item + self.operation.1,
        '*' => {
          let r = match self.operation.1 {
            i32::MAX => *item,
            x => x,
          };
          item * r
        }
        _ => unimplemented!(),
      };
      let updated_item = modular_reduction(updated_item / with_relief).abs();
      let to = match inv_heaviside(updated_item % self.test) {
        0 => self.result.0,
        1 => self.result.1,
        x => panic!("{}", x),
      };
      actions.push((to, updated_item));
    }

    actions
  }
}
