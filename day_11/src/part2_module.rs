use std::collections::VecDeque;

use super::common::*;
use super::problem_solver::ProblemSolver;


const ROUNDS: usize = 10_000;

pub struct PSInput {
  monkeys: Vec<Monkey>,
  items: Vec<VecDeque<i64>>, // separate because iterating each monkey we must mutate each other's items. assignment there requires a double borrow if from monkey.items
}

pub struct PSSolution {
  monkey_business: i64,
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let (items, monkeys): (Vec<VecDeque<i64>>, Vec<Monkey>) =
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
    let lcd: i64 = input.monkeys.iter().map(|m| m.test).product();
    let mut tallies =
      factory_ordered_troup_tallies(input.monkeys, input.items, ROUNDS, |x| {
        x % lcd
      });
    let monkey_business =
      (tallies.pop().unwrap() * tallies.pop().unwrap()) as i64;

    PSSolution { monkey_business }
  }

  fn output(solution: Self::Solution) {
    dbg!(solution.monkey_business);
  }
}
