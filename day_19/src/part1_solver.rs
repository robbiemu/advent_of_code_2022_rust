use good_lp::{
  constraint, default_solver, variable, variables, Solution, SolverModel,
};
use std::collections::HashMap;

use super::problem_solver::ProblemSolver;
use crate::common::{factory_system, prelude::*};


const TIME_STEPS: usize = 24;

pub struct PSInput {
  systems: Vec<System>,
}

pub struct PSSolution {
  scores: HashMap<usize, usize>,
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let systems = lines.map(|l| factory_system(l, TIME_STEPS)).collect();

    Self::Input { systems }
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let scores = input.systems.iter().map(score_system).collect();

    Self::Solution { scores }
  }

  fn output(solution: Self::Solution) {
    let mut order = solution.scores.keys().collect::<Vec<_>>();
    order.sort();
    let res = order
      .iter()
      .map(|key| {
        let value = solution.scores.get(key.to_owned()).unwrap();
        format!("{}:{}", key, value)
      })
      .collect::<Vec<_>>()
      .join("\n");

    println!(
      "quality sum: {}",
      solution.scores.iter().map(|(k, v)| k * v).sum::<usize>(),
    );
    println!("{res}");
  }
}

fn score_system(system: &System) -> (usize, usize) {
  let score = model_system(system);

  (system.id, score)
}

fn model_system(system: &System) -> usize {
  let minutes = system.time_steps + 1;
  let mut problem = variables!();
  // 4 robots to harvest materials
  let ore = problem.add_vector(variable().integer().min(0), minutes);
  let clay = problem.add_vector(variable().integer().min(0), minutes);
  let obsidian = problem.add_vector(variable().integer().min(0), minutes);
  let geode = problem.add_vector(variable().integer().min(0), minutes);
  // and a cumulative total of materials
  let ore_sum = problem.add_vector(variable().integer().min(0), minutes);
  let clay_sum = problem.add_vector(variable().integer().min(0), minutes);
  let obsidian_sum = problem.add_vector(variable().integer().min(0), minutes);
  let geode_sum = problem.add_vector(variable().integer().min(0), minutes);

  let mut model = problem
    // the cumulative pile is correct at the time step, but we want _after_
    .maximise(*geode_sum.last().unwrap())
    .using(default_solver)
    .with(constraint!(ore[0] == system.ore.get() as f64)) // robots
    .with(constraint!(clay[0] == system.clay.get() as f64))
    .with(constraint!(obsidian[0] == system.obsidian.get() as f64))
    .with(constraint!(geode[0] == system.geode.get() as f64))
    .with(constraint!(ore_sum[0] == 0)) // cumulative totals
    .with(constraint!(clay_sum[0] == 0))
    .with(constraint!(obsidian_sum[0] == 0))
    .with(constraint!(geode_sum[0] == 0));

  for i in 1..minutes {
    // cumulative totals
    model.add_constraint(constraint!(ore_sum[i] == ore_sum[i - 1] + ore[i]));
    model.add_constraint(constraint!(clay_sum[i] == clay_sum[i - 1] + clay[i]));
    model.add_constraint(constraint!(
      obsidian_sum[i] == obsidian_sum[i - 1] + obsidian[i]
    ));
    model
      .add_constraint(constraint!(geode_sum[i] == geode_sum[i - 1] + geode[i]));

    // the constraints ensure that the values do no decline from step to step
    model.add_constraint(constraint!(ore[i] >= ore[i - 1]));
    model.add_constraint(constraint!(clay[i] >= clay[i - 1]));
    model.add_constraint(constraint!(obsidian[i] >= obsidian[i - 1]));
    model.add_constraint(constraint!(geode[i] >= geode[i - 1]));

    // only one robot can be built each turn.
    let expr = ore[i] + clay[i] + obsidian[i] + geode[i]
      - ore[i - 1]
      - clay[i - 1]
      - obsidian[i - 1]
      - geode[i - 1];
    model.add_constraint(constraint!(expr.clone() <= 1));
    model.add_constraint(constraint!(expr >= 0));

    /* The amount of robots of each type on each minute is limited by the materials available to build them until that minute. */
    if i == 1 {
      continue;
    }
    model.add_constraint(constraint!(
      ore_sum[i - 2] // -- earlier I needed enough ore to...
        >= system.ore_robot.requirements.ore.unwrap_or(0) as f64
          * (ore[i] - ore[0]) // build my ore robots...
          + system.clay_robot.requirements.ore.unwrap_or(0) as f64
            * (clay[i] - clay[0]) // and my clay robots, etc
          + system.obsidian_robot.requirements.ore.unwrap_or(0) as f64
            * (obsidian[i] - obsidian[0])
          + system.geode_robot.requirements.ore.unwrap_or(0) as f64
            * (geode[i] - geode[0])
    ));
    model.add_constraint(constraint!(
      clay_sum[i - 2]
        >= system.ore_robot.requirements.clay.unwrap_or(0) as f64
          * (ore[i] - ore[0])
          + system.clay_robot.requirements.clay.unwrap_or(0) as f64
            * (clay[i] - clay[0])
          + system.obsidian_robot.requirements.clay.unwrap_or(0) as f64
            * (obsidian[i] - obsidian[0])
          + system.geode_robot.requirements.clay.unwrap_or(0) as f64
            * (geode[i] - geode[0])
    ));
    model.add_constraint(constraint!(
      obsidian_sum[i - 2]
        >= system.ore_robot.requirements.obsidian.unwrap_or(0) as f64
          * (ore[i] - ore[0])
          + system.clay_robot.requirements.obsidian.unwrap_or(0) as f64
            * (clay[i] - clay[0])
          + system.obsidian_robot.requirements.obsidian.unwrap_or(0) as f64
            * (obsidian[i] - obsidian[0])
          + system.geode_robot.requirements.obsidian.unwrap_or(0) as f64
            * (geode[i] - geode[0])
    ));
    model.add_constraint(constraint!(
      geode_sum[i - 2]
        >= system.ore_robot.requirements.geode.unwrap_or(0) as f64
          * (ore[i] - ore[0])
          + system.clay_robot.requirements.geode.unwrap_or(0) as f64
            * (clay[i] - clay[0])
          + system.obsidian_robot.requirements.geode.unwrap_or(0) as f64
            * (obsidian[i] - obsidian[0])
          + system.geode_robot.requirements.geode.unwrap_or(0) as f64
            * (geode[i] - geode[0])
    ));
  }

  let solution = model.solve().unwrap();
  for i in 0..minutes {
    let sci = if i == minutes - 1 { i } else { i + 1 };
    println!(
      "[blueprint {} @{}] ore:{}({}) clay:{}({}) obsidian:{}({}) geode:{}({})",
      system.id,
      i + 1,
      (solution.value(ore_sum[i])
        - (system.ore_robot.requirements.ore.unwrap_or(0) as f64
          * (solution.value(ore[sci]) - system.ore.get() as f64)
          + system.clay_robot.requirements.ore.unwrap_or(0) as f64
            * solution.value(clay[sci])
          + system.obsidian_robot.requirements.ore.unwrap_or(0) as f64
            * solution.value(obsidian[sci])
          + system.geode_robot.requirements.ore.unwrap_or(0) as f64
            * solution.value(geode[sci])))
      .round() as usize,
      (solution.value(ore[i])).round() as usize,
      (solution.value(clay_sum[i])
        - (system.ore_robot.requirements.clay.unwrap_or(0) as f64
          * solution.value(ore[sci])
          + system.clay_robot.requirements.clay.unwrap_or(0) as f64
            * solution.value(clay[sci])
          + system.obsidian_robot.requirements.clay.unwrap_or(0) as f64
            * solution.value(obsidian[sci])
          + system.geode_robot.requirements.clay.unwrap_or(0) as f64
            * solution.value(geode[sci])))
      .round() as usize,
      (solution.value(clay[i])).round() as usize,
      (solution.value(obsidian_sum[i])
        - (system.ore_robot.requirements.obsidian.unwrap_or(0) as f64
          * solution.value(ore[sci])
          + system.clay_robot.requirements.obsidian.unwrap_or(0) as f64
            * solution.value(clay[sci])
          + system.obsidian_robot.requirements.obsidian.unwrap_or(0) as f64
            * solution.value(obsidian[sci])
          + system.geode_robot.requirements.obsidian.unwrap_or(0) as f64
            * solution.value(geode[sci])))
      .round() as usize,
      (solution.value(obsidian[i])).round() as usize,
      (solution.value(geode_sum[i])
        - (system.ore_robot.requirements.geode.unwrap_or(0) as f64
          * solution.value(ore[sci])
          + system.clay_robot.requirements.geode.unwrap_or(0) as f64
            * solution.value(clay[sci])
          + system.obsidian_robot.requirements.geode.unwrap_or(0) as f64
            * solution.value(obsidian[sci])
          + system.geode_robot.requirements.geode.unwrap_or(0) as f64
            * solution.value(geode[sci])))
      .round() as usize,
      (solution.value(geode[i])).round() as usize
    );
  }
  let geode_solution = solution.value(*geode_sum.last().unwrap()).round();

  geode_solution as usize
}
