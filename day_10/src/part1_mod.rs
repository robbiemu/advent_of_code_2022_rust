use std::collections::BTreeMap;

use super::common::interpret_command;
use super::problem_solver::ProblemSolver;

pub struct PSInput {
  commands: Vec<(String, Option<i32>)>,
}

pub struct PSSolution {
  total_signal_strength: i32,
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let commands: Vec<(String, Option<i32>)> = lines
      .enumerate()
      .map(|(h, l)| interpret_command(h, l))
      .collect();

    Self::Input { commands }
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let mut x = 1;
    let mut actions: BTreeMap<usize, i32> = BTreeMap::new();
    let mut step: usize = 0;
    input.commands.iter().for_each(|(cmd, v)| {
      step += 1;
      if let Some(value) = v {
        let i = match cmd.as_str() {
          "addx" => 1,
          "noop" => 0,
          _ => unreachable!(),
        };
        step += i;
        actions.insert(step, *value);
      }
    });

    // let key_steps = [1, 2, 3, 4, 5, 6];
    let key_steps = [20, 60, 100, 140, 180, 220];
    for measurement in key_steps {
      actions.entry(measurement).or_insert(0);
      actions.entry(measurement + 1).or_insert(0);
    }

    let signal_strengths: Vec<i32> = actions
      .iter()
      .filter_map(|(k, v)| {
        x += v;
        if key_steps.contains(k) && *k <= *key_steps.last().unwrap() {
          Some((x - v) * *k as i32)
        } else {
          None
        }
      })
      .collect();
    dbg!(signal_strengths.clone());

    Self::Solution {
      total_signal_strength: signal_strengths.iter().sum(),
    }
  }

  fn output(solution: Self::Solution) {
    println!("total signal strength: {}", solution.total_signal_strength)
  }
}
