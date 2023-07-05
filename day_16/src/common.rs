pub mod prelude {
  use std::hash::{Hash, Hasher};


  pub const MAX_STEP: usize = 30;

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

use std::collections::{HashMap, HashSet};

use petgraph::{prelude::GraphMap, Undirected};

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

pub fn find_node(
  label: String,
  graph: &GraphMap<Valve, usize, Undirected>,
) -> Option<Valve> {
  graph.nodes().find(|valve| valve.label == label)
}

pub fn get_shortest_flow_paths(
  current_node: Valve,
  nodes: HashSet<Valve>,
  fw: HashMap<(Valve, Valve), usize>,
) -> GraphMap<Valve, usize, Undirected> {
  let mut shortest_flow_paths: GraphMap<Valve, usize, Undirected> =
    GraphMap::new();
  nodes.iter().enumerate().for_each(|(i, from)| {
    if *from != current_node {
      shortest_flow_paths.add_edge(
        *from,
        current_node,
        fw[&(*from, current_node)],
      );
    }
    nodes.iter().skip(i + 1).for_each(|to| {
      if !shortest_flow_paths.contains_edge(*from, *to) {
        shortest_flow_paths.add_edge(*from, *to, fw[&(*from, *to)]);
      }
    });
  });

  shortest_flow_paths
}

#[allow(dead_code)]
pub fn find_path(
  step: usize,
  score: usize,
  path: Vec<Valve>,
  graph: &GraphMap<Valve, usize, Undirected>,
) -> (Vec<Valve>, usize) {
  if path.len() == graph.nodes().len() {
    return (path, score);
  }
  let current_node = *path.last().unwrap();
  graph.edges(current_node).fold(
    (path.clone(), score),
    |acc, (from, to, cost)| {
      let node = if from == current_node { to } else { from };

      if path.contains(&node) || MAX_STEP < (step + cost + 1) {
        return acc;
      }

      let (p, sc) = find_path(
        step + cost + 1,
        score + node.coefficient * (MAX_STEP - (step + cost)),
        [path.clone(), vec![node]].concat(),
        graph,
      );
      if sc > acc.1 {
        (p, sc)
      } else {
        acc
      }
    },
  )
}
