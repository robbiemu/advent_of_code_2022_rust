use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::Directed;

use super::du_directories::du_directories;
use super::fs_graph::factory_fs_graph;
use super::problem_solver::ProblemSolver;


const TOTAL_SPACE: u64 = 70_000_000;
const THRESHOLD: u64 = 30_000_000;

pub struct PSInput {
  graph: StableGraph<u64, u64, Directed>,
}

pub struct PSSolution {
  smallest: u64,
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let graph = factory_fs_graph(lines);

    Self::Input { graph }
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let graph = du_directories(input.graph);

    let free_space = TOTAL_SPACE - graph[NodeIndex::new(1)];
    let target = THRESHOLD - free_space;

    let smallest_large = graph
      .node_indices()
      .filter(|&node| graph[node] > target)
      .min_by_key(|&node| graph[node])
      .unwrap();

    let smallest = graph[smallest_large];

    Self::Solution { smallest }
  }

  fn output(solution: Self::Solution) {
    println!("{}", solution.smallest)
  }
}
