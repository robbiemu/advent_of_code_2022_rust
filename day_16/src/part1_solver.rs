use petgraph::{algo::floyd_warshall, prelude::GraphMap, Undirected};
use std::collections::HashSet;

use super::problem_solver::ProblemSolver;
use crate::common::{
  find_node, find_path, get_shortest_flow_paths, parse_line, prelude::*,
};


pub struct PSInput {
  graph: GraphMap<Valve, usize, Undirected>,
}

pub struct PSSolution {
  score: usize,
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let mut graph: GraphMap<Valve, usize, Undirected> = GraphMap::new();
    lines
      .map(|l| parse_line(&l))
      .fold(Vec::new(), |acc, (valve, tunnels)| {
        graph.add_node(valve);

        [acc, tunnels].concat()
      })
      .iter()
      .for_each(|Tunnel { from, to }| {
        let Some(from_node) = graph.nodes().find(|x| x == from) else {
          return;
        };
        let Some(to_node) = graph.nodes().find(|x| x == to) else {
          return;
        };
        graph.add_edge(from_node, to_node, 1);
      });

    Self::Input { graph }
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let flow_valve_network: HashSet<Valve> = input
      .graph
      .nodes()
      .filter(|valve| valve.coefficient > 0)
      .collect();
    let fw = floyd_warshall(&input.graph, |_| 1).unwrap();

    let start_node_label = "AA".to_owned();
    let current_node = find_node(start_node_label, &input.graph).unwrap();

    let shortest_flow_paths: GraphMap<Valve, usize, Undirected> =
      get_shortest_flow_paths(current_node, flow_valve_network, fw);

    let (best_path, score) =
      find_path(1, 0, vec![current_node], &shortest_flow_paths);

    dbg!(best_path.iter().map(|n| n.label).collect::<String>());

    Self::Solution { score }
  }

  fn output(solution: Self::Solution) {
    println!("score {}", solution.score);
  }
}
