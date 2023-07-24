use petgraph::prelude::*;

use super::problem_solver::ProblemSolver;
use crate::common::{apply_edges, parse_nodes, prelude::*, reduce_nodes};


pub struct PSInput {
  graph: GraphMap<MonkeyBusiness, usize, Directed>,
}

pub struct PSSolution {
  value: Option<i64>,
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;

  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let records: Vec<String> = lines.collect();
    let mut graph = parse_nodes(records.iter().map(|n| n.to_owned()));
    apply_edges(records.iter().map(|n| n.to_owned()), &mut graph);

    Self::Input { graph }
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let mut nodes = reduce_nodes(input.graph, NodeReductionLimit::Some(1));

    let value = if let Some(root) = nodes.pop() {
      root.fn_type.get_value()
    } else {
      None
    };

    Self::Solution { value }
  }

  fn output(solution: Self::Solution) {
    match solution.value {
      Some(n) => println!("solution found {}", n),
      None => println!("no solution found"),
    }
  }
}
