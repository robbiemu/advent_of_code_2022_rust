pub mod prelude {
  use std::ops::{Index, IndexMut};

  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
  }

  impl Material {
    pub fn type_str(&self) -> String {
      match self {
        Material::Ore => "ore",
        Material::Clay => "clay",
        Material::Obsidian => "obsidian",
        Material::Geode => "geode",
      }
      .to_string()
    }
  }

  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct Robot {
    pub requirements: Requirement,
    pub output: Material,
  }

  impl Robot {
    pub fn from_type(material: String) -> Robot {
      let output = match material.as_str() {
        "ore" => Material::Ore,
        "clay" => Material::Clay,
        "obsidian" => Material::Obsidian,
        "geode" => Material::Geode,
        _ => {
          panic!("{material}");
        }
      };

      Robot { output, requirements: Requirement::default() }
    }
  }

  #[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
  pub struct Requirement {
    pub ore: Option<usize>,
    pub clay: Option<usize>,
    pub obsidian: Option<usize>,
    pub geode: Option<usize>,
  }

  impl Index<&str> for Requirement {
    type Output = Option<usize>;

    fn index(&self, index: &str) -> &Self::Output {
      match index {
        "ore" => &self.ore,
        "clay" => &self.clay,
        "obsidian" => &self.obsidian,
        "geode" => &self.geode,
        _ => panic!("Invalid material: {}", index),
      }
    }
  }

  impl IndexMut<&str> for Requirement {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
      match index {
        "ore" => &mut self.ore,
        "clay" => &mut self.clay,
        "obsidian" => &mut self.obsidian,
        "geode" => &mut self.geode,
        _ => panic!("Invalid material: {}", index),
      }
    }
  }

  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct System {
    pub id: usize,
    pub ore: usize,
    pub clay: usize,
    pub obsidian: usize,
    pub geode: usize,
    pub ore_robot: Robot,
    pub clay_robot: Robot,
    pub obsidian_robot: Robot,
    pub geode_robot: Robot,
    pub time_steps: usize,
  }
}

impl Default for System {
  fn default() -> Self {
    Self {
      id: Default::default(),
      ore: 0,
      clay: 0,
      obsidian: 0,
      geode: 0,
      ore_robot: Robot {
        output: Material::Ore,
        requirements: Requirement::default(),
      },
      clay_robot: Robot {
        output: Material::Clay,
        requirements: Requirement::default(),
      },
      obsidian_robot: Robot {
        output: Material::Obsidian,
        requirements: Requirement::default(),
      },
      geode_robot: Robot {
        output: Material::Geode,
        requirements: Requirement::default(),
      },
      time_steps: Default::default(),
    }
  }
}

use good_lp::{
  constraint, default_solver, solvers::scip::SCIPSolved, variable, variables,
  Solution, SolverModel, Variable,
};
use sscanf::sscanf;
use std::collections::{HashMap, VecDeque};

use prelude::*;


pub fn factory_system(line: String, time_steps: usize) -> System {
  /* Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each
  obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7
  obsidian. */
  let mut blueprint: VecDeque<String> =
    line.split("Each").map(|s| s.to_string()).collect();

  let Some(id_str) = blueprint.pop_front() else {
    panic!("invalid blueprint:\n{}", line);
  };
  let id = match sscanf!(id_str.trim(), "Blueprint {}:", usize) {
    Ok(n) => n,
    Err(_) => panic!("invalid blueprint: '{}'\n{}", id_str, line),
  };

  let robots: HashMap<String, Robot> = blueprint
    .iter()
    .map(|s| {
      let robot = factory_robot(s.to_string());

      (robot.output.type_str(), robot)
    })
    .collect();

  System {
    id,
    ore: 1,
    time_steps,
    ore_robot: robots["ore"].clone(),
    clay_robot: robots["clay"].clone(),
    obsidian_robot: robots["obsidian"].clone(),
    geode_robot: robots["geode"].clone(),
    ..System::default()
  }
}

fn factory_robot(spec: String) -> Robot {
  let parts: Vec<&str> = spec.split("costs").collect();
  let mut robot_type = parts[0].trim().to_string();
  robot_type = robot_type.split_whitespace().next().unwrap().to_string();
  let robot_costs = parts[1].trim().to_string();
  let mut robot = Robot::from_type(robot_type);
  let costs: Vec<String> =
    robot_costs.split("and").map(|s| s.to_string()).collect();
  let requirements =
    costs
      .iter()
      .fold(Requirement::default(), |mut acc, cost_str| {
        let (amt, mat) = match sscanf!(
          cost_str.trim().trim_end_matches('.'),
          r"{} {}",
          usize,
          String
        ) {
          Ok((a, b)) => (a, b),
          Err(_) => panic!("invalid blueprint: '{cost_str}'\n{spec}"),
        };

        acc[&mat] = Some(amt);

        acc
      });
  robot.requirements = requirements;

  robot
}

pub fn score_system(system: &System) -> (usize, usize) {
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
    .with(constraint!(ore[0] == system.ore as f64)) // robots
    .with(constraint!(clay[0] == system.clay as f64))
    .with(constraint!(obsidian[0] == system.obsidian as f64))
    .with(constraint!(geode[0] == system.geode as f64))
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
  log_solution(
    &solution,
    system,
    MinuteRecord {
      ore,
      clay,
      obsidian,
      geode,
      ore_sum,
      clay_sum,
      obsidian_sum,
      geode_sum: geode_sum.clone(),
    },
  );
  let geode_solution = solution.value(*geode_sum.last().unwrap()).round();

  geode_solution as usize
}

struct MinuteRecord {
  ore: Vec<Variable>,
  clay: Vec<Variable>,
  obsidian: Vec<Variable>,
  geode: Vec<Variable>,
  ore_sum: Vec<Variable>,
  clay_sum: Vec<Variable>,
  obsidian_sum: Vec<Variable>,
  geode_sum: Vec<Variable>,
}

fn log_solution(
  solution: &SCIPSolved,
  system: &System,
  minute_record: MinuteRecord,
) {
  let minutes = system.time_steps + 1;

  for i in 0..minutes {
    let sci = if i == minutes - 1 { i } else { i + 1 };
    println!(
      "[blueprint {} @{}] ore:{}({}) clay:{}({}) obsidian:{}({}) geode:{}({})",
      system.id,
      i + 1,
      (solution.value(minute_record.ore_sum[i])
        - (system.ore_robot.requirements.ore.unwrap_or(0) as f64
          * (solution.value(minute_record.ore[sci]) - system.ore as f64)
          + system.clay_robot.requirements.ore.unwrap_or(0) as f64
            * solution.value(minute_record.clay[sci])
          + system.obsidian_robot.requirements.ore.unwrap_or(0) as f64
            * solution.value(minute_record.obsidian[sci])
          + system.geode_robot.requirements.ore.unwrap_or(0) as f64
            * solution.value(minute_record.geode[sci])))
      .round() as usize,
      (solution.value(minute_record.ore[i])).round() as usize,
      (solution.value(minute_record.clay_sum[i])
        - (system.ore_robot.requirements.clay.unwrap_or(0) as f64
          * solution.value(minute_record.ore[sci])
          + system.clay_robot.requirements.clay.unwrap_or(0) as f64
            * solution.value(minute_record.clay[sci])
          + system.obsidian_robot.requirements.clay.unwrap_or(0) as f64
            * solution.value(minute_record.obsidian[sci])
          + system.geode_robot.requirements.clay.unwrap_or(0) as f64
            * solution.value(minute_record.geode[sci])))
      .round() as usize,
      (solution.value(minute_record.clay[i])).round() as usize,
      (solution.value(minute_record.obsidian_sum[i])
        - (system.ore_robot.requirements.obsidian.unwrap_or(0) as f64
          * solution.value(minute_record.ore[sci])
          + system.clay_robot.requirements.obsidian.unwrap_or(0) as f64
            * solution.value(minute_record.clay[sci])
          + system.obsidian_robot.requirements.obsidian.unwrap_or(0) as f64
            * solution.value(minute_record.obsidian[sci])
          + system.geode_robot.requirements.obsidian.unwrap_or(0) as f64
            * solution.value(minute_record.geode[sci])))
      .round() as usize,
      (solution.value(minute_record.obsidian[i])).round() as usize,
      (solution.value(minute_record.geode_sum[i])
        - (system.ore_robot.requirements.geode.unwrap_or(0) as f64
          * solution.value(minute_record.ore[sci])
          + system.clay_robot.requirements.geode.unwrap_or(0) as f64
            * solution.value(minute_record.clay[sci])
          + system.obsidian_robot.requirements.geode.unwrap_or(0) as f64
            * solution.value(minute_record.obsidian[sci])
          + system.geode_robot.requirements.geode.unwrap_or(0) as f64
            * solution.value(minute_record.geode[sci])))
      .round() as usize,
      (solution.value(minute_record.geode[i])).round() as usize
    );
  }
}
