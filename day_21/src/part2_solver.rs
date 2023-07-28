use petgraph::prelude::*;
use std::collections::HashMap;

use super::problem_solver::ProblemSolver;
use crate::common::{apply_edges, parse_nodes, prelude::*};
use crate::simple_parser::resolve_equation;

const HUMAN_NAME: &str = "humn";
const HUMAN_VARIABLE: &str = "a";

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

    let root_id = MonkeyBusiness::id_from_str(ROOT_NAME);
    let mut root = graph
      .nodes()
      .find(|n| n.id == root_id)
      .unwrap_or_else(|| panic!("invalid input, no root node"));
    graph.remove_node(root);
    let participants = root.fn_type.get_participants();
    root.fn_type = FnType::Eq(participants);
    graph.add_node(root);

    apply_edges(records.iter().map(|n| n.to_owned()), &mut graph);

    Self::Input { graph }
  }

  fn solve(input: Self::Input) -> Self::Solution {
    let root_id = MonkeyBusiness::id_from_str(ROOT_NAME);
    let root = input.graph.nodes().find(|n| n.id == root_id).unwrap();

    let equation =
      convert_graph_to_equation(&root, &input.graph, &mut HashMap::new());
    dbg!(&equation);

    let value = resolve_equation(equation.as_str());

    Self::Solution { value }
  }

  fn output(solution: Self::Solution) {
    match solution.value {
      Some(n) => println!("solution found {}", n),
      None => println!("no solution found"),
    }
  }
}

fn convert_graph_to_equation(
  node: &MonkeyBusiness,
  graph: &GraphMap<MonkeyBusiness, usize, Directed>,
  calculated_representations: &mut HashMap<MonkeyBusiness, String>,
) -> String {
  if let Some(representation) = calculated_representations.get(node) {
    return representation.clone();
  }

  if let FnType::Value(n) = node.fn_type {
    let node_name = MonkeyBusiness::string_from_id(node.id);
    let value_str = if matches!(node_name.as_str(), HUMAN_NAME) {
      HUMAN_VARIABLE.to_string()
    } else {
      n.to_string()
    };
    calculated_representations.insert(*node, value_str.clone());
    return value_str;
  }

  let op: &str = node.fn_type.try_into().unwrap_or("");

  let participants = match node.fn_type.get_participants() {
    Some(participants) => participants,
    None => return "".to_string(),
  };

  let mut left_side = String::new();
  let mut right_side = String::new();

  for neighbor in graph.neighbors_directed(*node, Incoming) {
    if !calculated_representations.contains_key(&neighbor) {
      let representation =
        convert_graph_to_equation(&neighbor, graph, calculated_representations);
      calculated_representations.insert(neighbor, representation.clone());
    }
  }

  if let Some(left) = graph.nodes().find(|n| n.id == participants.left) {
    left_side = calculated_representations.get(&left).unwrap().clone();
  }

  if let Some(right) = graph.nodes().find(|n| n.id == participants.right) {
    right_side = calculated_representations.get(&right).unwrap().clone();
  }

  if matches!(node.fn_type, FnType::Eq(_)) {
    format!("{} {} {}", left_side, op, right_side)
  } else {
    format!("({} {} {})", left_side, op, right_side)
  }
}
