use petgraph::{algo::floyd_warshall, prelude::GraphMap, Undirected};
use std::collections::HashSet;

use super::problem_solver::ProblemSolver;
use crate::common::{parse_line, prelude::*};


const MAX_STEP: usize = 30;

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

    let mut shortest_flow_paths: GraphMap<Valve, usize, Undirected> =
      GraphMap::new();
    flow_valve_network.iter().enumerate().for_each(|(i, from)| {
      if *from != current_node {
        shortest_flow_paths.add_edge(
          *from,
          current_node,
          fw[&(*from, current_node)],
        );
      }
      flow_valve_network.iter().skip(i + 1).for_each(|to| {
        if !shortest_flow_paths.contains_edge(*from, *to) {
          shortest_flow_paths.add_edge(*from, *to, fw[&(*from, *to)]);
        }
      });
    });

    let sfp: String = shortest_flow_paths
      .nodes()
      .map(|v| {
        let edges = shortest_flow_paths
          .edges(v)
          .map(|(f, t, c)| format!("{}-{} {c}", f.label, t.label))
          .collect::<Vec<String>>()
          .join(",");
        format!("{} [{edges}]\n", v.label)
      })
      .collect();
    eprintln!("{sfp}");

    let (best_path, score) =
      find_path(1, 0, vec![current_node], shortest_flow_paths);

    dbg!(best_path.iter().map(|n| n.label).collect::<String>());

    Self::Solution { score }
  }

  fn output(solution: Self::Solution) {
    println!("score {}", solution.score);
  }
}

fn find_node(
  label: String,
  graph: &GraphMap<Valve, usize, Undirected>,
) -> Option<Valve> {
  graph.nodes().find(|valve| valve.label == label)
}

fn find_path(
  step: usize,
  score: usize,
  path: Vec<Valve>,
  graph: GraphMap<Valve, usize, Undirected>,
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
        graph.clone(),
      );
      if sc > acc.1 {
        (p, sc)
      } else {
        acc
      }
    },
  )
}
