use std::collections::BTreeMap;

use super::common::interpret_command;
use super::problem_solver::ProblemSolver;

pub struct PSInput {
  commands: Vec<(String, Option<i32>)>,
}

pub struct PSSolution {
  capital_letters: [char; 8],
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
    let mut step: usize = 0;
    let mut actions: BTreeMap<usize, i32> = BTreeMap::new();
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
    for measurement in 1..=240 {
      actions.entry(measurement).or_insert(0);
    }
    let mut cursor: i32 = 1;
    let screen_buffer: Vec<char> = (0..240)
      .map(|i| {
        if i % 40 == 0 && i > 2 {
          cursor += 40;
        }
        println!("{} c{}, c_after{}", i, cursor, cursor + actions[&(i + 1)]);
        let pixel = if (cursor - i as i32).abs() < 2 {
          '#'
        } else {
          '.'
        };
        cursor += actions[&(i + 1)];

        pixel
      })
      .collect();

    for line in screen_buffer.chunks(40) {
      println!("{}", line.iter().collect::<String>());
    }

    Self::Solution { capital_letters: ['.'; 8] }
  }

  fn output(solution: Self::Solution) {
    println!(
      "capital letters: {}",
      solution.capital_letters.iter().collect::<String>()
    )
  }
}
