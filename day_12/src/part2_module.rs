use petgraph::graph::Graph;
use petgraph::prelude::*;

use crate::common::{factory_graph_from_map, find_path_part2};
use crate::problem_solver::ProblemSolver;


pub struct PSInput {
  graph: Graph<usize, ()>,
  start: usize,
  end: usize,
  flattened: Vec<char>,
}

#[derive(Debug)]
pub struct PSSolution {
  path: Option<(i32, Vec<NodeIndex>)>,
  //end: NodeIndex,
  flattened: Vec<char>,
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let map: Vec<String> = lines.collect();
    let (start_opt, end_opt, graph, flattened) = factory_graph_from_map(map);

    let start = start_opt.unwrap();
    let end = end_opt.unwrap();

    Self::Input { graph, start, end, flattened }
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let mut flattened = input.flattened.clone();
    let (path, start) = find_path_part2(
      input.graph.clone(),
      input.flattened,
      input.start,
      input.end,
    );

    flattened[start] = 'S';
    flattened[input.end] = 'E';

    Self::Solution { path, /* end, */ flattened }
  }

  fn output(solution: Self::Solution) {
    let Some((cost, path)) = solution.path else {
      println!("no route to end!");
      return;
    };

    println!(
      "total cost: {} {:?}",
      cost,
      path
        .iter()
        .map(|ni| solution.flattened[ni.index()].to_string())
        .collect::<Vec<_>>()
        .join("->")
    );
  }
}
