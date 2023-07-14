pub mod prelude {
  use std::ops::{Index, IndexMut};

  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub enum Material {
    Ore(usize),
    Clay(usize),
    Obsidian(usize),
    Geode(usize),
  }

  impl Material {
    pub fn get(&self) -> usize {
      match self {
        Material::Ore(n) => *n,
        Material::Clay(n) => *n,
        Material::Obsidian(n) => *n,
        Material::Geode(n) => *n,
      }
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
        "ore" => Material::Ore(1),
        "clay" => Material::Clay(1),
        "obsidian" => Material::Obsidian(1),
        "geode" => Material::Geode(1),
        _ => {
          panic!("{material}");
        }
      };

      Robot { output, requirements: Requirement::default() }
    }

    pub fn type_str(&self) -> String {
      match self.output {
        Material::Ore(_) => "ore",
        Material::Clay(_) => "clay",
        Material::Obsidian(_) => "obsidian",
        Material::Geode(_) => "geode",
      }
      .to_string()
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
    pub ore: Material,
    pub clay: Material,
    pub obsidian: Material,
    pub geode: Material,
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
      ore: Material::Ore(0),
      clay: Material::Clay(0),
      obsidian: Material::Obsidian(0),
      geode: Material::Geode(0),
      ore_robot: Robot {
        output: Material::Ore(1),
        requirements: Requirement::default(),
      },
      clay_robot: Robot {
        output: Material::Clay(1),
        requirements: Requirement::default(),
      },
      obsidian_robot: Robot {
        output: Material::Obsidian(1),
        requirements: Requirement::default(),
      },
      geode_robot: Robot {
        output: Material::Geode(1),
        requirements: Requirement::default(),
      },
      time_steps: Default::default(),
    }
  }
}

use sscanf::sscanf;
use std::collections::{HashMap, VecDeque};

use prelude::*;


pub fn factory_system(line: String, time_steps: usize) -> System {
  // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.

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

      (robot.type_str(), robot)
    })
    .collect();


  System {
    id,
    ore: Material::Ore(1),
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
