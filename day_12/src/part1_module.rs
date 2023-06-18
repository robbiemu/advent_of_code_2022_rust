use petgraph::graph::Graph;
use petgraph::prelude::*;
//use std::collections::HashMap;

use crate::common::{factory_graph_from_map, find_path_part1};
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
    let path = find_path_part1(input.graph, input.start, input.end);
    //let path = dijkstra(&input.graph, start, Some(end), |_| 1);

    let mut flattened = input.flattened;
    flattened[input.start] = 'S';
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
