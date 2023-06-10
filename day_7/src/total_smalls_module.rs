use petgraph::stable_graph::StableGraph;
use petgraph::Directed;

use super::du_directories::du_directories;
use super::fs_graph::factory_fs_graph;
use super::problem_solver::ProblemSolver;


pub struct PSInput {
  graph: StableGraph<u64, u64, Directed>,
}

pub struct PSSolution {
  sum: u64,
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

    let sum = graph.node_indices().fold(0, |acc, curr| {
      if graph[curr] <= 100_000 && acc < u64::MAX {
        if let Some(new_sum) = acc.checked_add(graph[curr]) {
          new_sum
        } else {
          u64::MAX
        }
      } else {
        acc
      }
    });

    Self::Solution { sum }
  }

  fn output(solution: Self::Solution) {
    println!("{}", solution.sum)
  }
}
