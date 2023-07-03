pub mod prelude {
  use std::hash::{Hash, Hasher};


  #[derive(Clone, Copy, Debug, Eq, PartialOrd, Ord)]
  pub struct Valve {
    pub label: &'static str,
    pub coefficient: Coefficient,
  }

  impl Valve {
    pub fn get_hash(label: &'static str) -> Valve {
      Valve { label, coefficient: 0 }
    }
  }

  impl Hash for Valve {
    fn hash<H: Hasher>(&self, state: &mut H) {
      self.label.hash(state);
      self.coefficient.hash(state);
    }
  }

  impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
      self.label == other.label
    }
  }

  pub type Coefficient = usize;

  #[derive(Clone)]
  pub struct Tunnel {
    pub from: Valve,
    pub to: Valve,
  }
}

use prelude::*;


pub fn parse_line(line: &str) -> (Valve, Vec<Tunnel>) {
  // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
  let parts: Vec<&str> = line.split_whitespace().collect();

  let rate_str = parts
    .get(4)
    .cloned()
    .unwrap_or_else(|| panic!("Invalid input: missing flow rate\n{line}"));
  let rate_til_semicolon = rate_str
    .split_once('=')
    .unwrap_or_else(|| panic!("Invalid input: missing flow rate\n{line}"))
    .1;
  let flow_rate: usize = rate_til_semicolon[..rate_til_semicolon.len() - 1]
    .parse()
    .unwrap_or_else(|_| panic!("Invalid flow rate\n{line}"));

  let label = parts
    .get(1)
    .cloned()
    .unwrap_or_else(|| panic!("Invalid input: missing label\n{line}"));
  let label_static: &'static str = Box::leak(label.to_owned().into_boxed_str());
  let valve = Valve { label: label_static, coefficient: flow_rate };

  let edges: Vec<Tunnel> = parts[9..]
    .iter()
    .map(|&edge_str| {
      let to_short_lived = edge_str.trim_end_matches(',').to_owned();
      let to: &'static str = Box::leak(to_short_lived.into_boxed_str());

      Tunnel { from: valve, to: Valve::get_hash(to) }
    })
    .collect();

  (valve, edges)
}
