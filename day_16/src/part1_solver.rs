use petgraph::{algo::floyd_warshall, prelude::GraphMap, Undirected};
use std::collections::HashMap;

use super::problem_solver::ProblemSolver;
use crate::common::{parse_line, prelude::*};


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
    let fw = floyd_warshall(&input.graph, |_| 1).unwrap();
    let mut eulerian_network: GraphMap<Valve, usize, Undirected> =
      GraphMap::new();
    let nodes: Vec<Valve> = input.graph.nodes().collect();
    nodes.iter().enumerate().for_each(|(i, from)| {
      nodes.iter().skip(i + 1).for_each(|to| {
        if from != to && !eulerian_network.contains_edge(*from, *to) {
          eulerian_network.add_edge(*from, *to, fw[&(*from, *to)]);
        }
      });
    });

    let mut dp_table: HashMap<String, Vec<usize>> = HashMap::new();
    let max_steps = 30;
    let start_node_label = "AA".to_owned();
    for node in nodes {
      dp_table.insert(node.label.to_owned(), vec![0; max_steps + 1]);
    }

    for time_step in 1..=max_steps {
      for current_node in input.graph.nodes() {
        let base_score: usize =
          eulerian_network.edges(current_node).fold(0, |acc, cur| {
            if (time_step as isize - *cur.2 as isize) < 0 {
              return acc;
            }
            let candidate = if cur.0 == current_node { cur.1 } else { cur.0 };
            let candidate_score = dp_table[candidate.label][time_step - cur.2];

            if candidate_score >= acc {
              candidate_score
            } else {
              acc
            }
          });

        let score_vector = dp_table.get_mut(current_node.label).unwrap();
        score_vector[time_step] = base_score.max(score_vector[time_step]);
        if time_step < max_steps {
          let addend = current_node.coefficient * (max_steps - (time_step + 1));
          score_vector[time_step + 1] =
            score_vector[time_step + 1].max(base_score + addend);
        }
      }
    }

    let score = dp_table
      .get(&start_node_label)
      .map(|values| values.iter().cloned().max().unwrap_or(0))
      .unwrap_or(0);

    Self::Solution { score }
  }

  fn output(solution: Self::Solution) {
    println!("score {}", solution.score);
  }
}
