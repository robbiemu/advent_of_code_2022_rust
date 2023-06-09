use petgraph::stable_graph::{StableGraph, NodeIndex};
use petgraph::{Directed, Incoming};
use std::collections::HashMap;

use super::problem_solver::ProblemSolver;


pub struct PSInput {
  graph: StableGraph<u32, u32, Directed>
}

pub struct PSSolution {
  sum: u32
}

pub struct ProblemSolverPattern;

impl ProblemSolver for ProblemSolverPattern {
  type Input = PSInput;
  type Solution = PSSolution;
  
  fn initialize(lines: impl Iterator<Item = String>) -> Self::Input {
    let mut graph = StableGraph::<u32, u32, Directed>::new();
    let shadow_root_index = graph.add_node(0);
    let mut cwd: Vec<NodeIndex> = vec![shadow_root_index];
    let mut parent_to_child: HashMap<NodeIndex, Vec<(String, NodeIndex)>> 
    = HashMap::new();
    
    for line in lines {
      match identify_line_type(&line) {
        "Command" => change_directory(
          factory_current_working_directory(&line),
          &mut graph, &mut cwd, &mut parent_to_child,
        ),
        "Directory Response" => process_record(
          factory_directory_node(&line),
          &mut graph, &mut cwd, &mut parent_to_child,
        ),
        "File Response" => process_record(
          factory_file_node(&line),
          &mut graph, &mut cwd, &mut parent_to_child,
        ),
        _ => unreachable!(),
      }
    }
    
    Self::Input { graph }
  }
  
  fn solve(input: Self::Input) -> Self::Solution {
    let mut graph = input.graph.clone();
    let file_indices = input.graph
    .node_indices()
    .filter(|&i| input.graph[i] > 0);
    
    println!("{:?}", graph.edge_indices()
      .map(|i| graph.edge_endpoints(i).unwrap()).collect::<Vec<_>>());
    
    for node in file_indices {
      propagate_values_up_tree(&mut graph, node);
      graph.remove_node(node); // graph becomes only directories
    }
    
    let sum = graph.node_indices().fold(0, |acc, curr| {
      if graph[curr] <= 100_000 && acc < u32::MAX {
        if let Some(new_sum) = acc.checked_add(graph[curr]) {
          new_sum
        } else {
          u32::MAX
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

fn process_record(
  n: Option<(Option<String>, u32)>, 
  graph: &mut StableGraph<u32, u32, Directed>, 
  cwd: &mut Vec<NodeIndex>, 
  parent_to_child: &mut HashMap<NodeIndex, Vec<(String, NodeIndex)>>
) {
  let cwdi = cwd.last().cloned().unwrap();
  if let Some((dopt, size)) = n {
    let name = dopt.unwrap_or(String::new());
    let i = graph.add_node(size);
    graph.add_edge(cwdi, i, size);
    if name.len() > 0 {
      if !parent_to_child.contains_key(&cwdi) {
        parent_to_child.insert(cwdi, Vec::<(String, NodeIndex)>::new());
      }
      parent_to_child.get_mut(&cwdi).unwrap().push((name, i));
    }
  }
}

fn change_directory(
  d: Option<String>, 
  graph: &mut StableGraph<u32, u32, Directed>, 
  cwd: &mut Vec<NodeIndex>, 
  parent_to_child: &mut HashMap<NodeIndex, Vec<(String, NodeIndex)>>
) {
  if let Some(dir) = d {
    if dir == "..".to_string() {
      match cwd.is_empty() {
        false => {
          cwd.pop();
        },
        true => unreachable!(),
      }
    } else {
      let cwdi = cwd.last().cloned().unwrap();
      let child = parent_to_child.get(&cwdi)
      .and_then(|children| 
        children.iter().find(|(dirname, _)| dirname == &dir)
      );
      if let Some((_, i)) = child {
        cwd.push(*i);
      } else {
        let i = graph.add_node(0);
        if !parent_to_child.contains_key(&cwdi) {
          parent_to_child.insert(cwdi, Vec::<(String, NodeIndex)>::new());
        }
        parent_to_child.get_mut(&cwdi).unwrap().push((dir, i));
        cwd.push(i);
      }
    }
  }
}

fn identify_line_type(line: &str) -> &str {
  match line.trim_start().chars().next() {
    Some('$') => "Command",
    Some('d') => "Directory Response",
    Some(c) if c.is_digit(10) => "File Response",
    Some(_) => unreachable!(),
    None => unreachable!(),
  }
}

fn factory_current_working_directory(line: &str) -> Option<String> {
  line.strip_prefix("$ cd ").map(|dir| dir.to_string())
}

fn factory_file_node(line: &str) -> Option<(Option<String>, u32)> {
  let (size_str, _name) = line.split_once(' ')?;
  let size = size_str.parse::<u32>().ok()?;
  Some((None, size))
}

fn factory_directory_node(line: &str) -> Option<(Option<String>, u32)> {
  let name = line.strip_prefix("dir ")?;
  Some((Some(name.to_string()), 0))
}

fn propagate_values_up_tree(graph: &mut StableGraph<u32, u32, Directed>, node: NodeIndex) {
  let mut stack = vec![node];
  
  while let Some(current_node) = stack.pop() {
    let mut edges = graph.neighbors_directed(current_node, Incoming).detach();
    while let Some(edge) = edges.next_edge(graph) {
      let (source, target) = graph.edge_endpoints(edge).unwrap();
      println!("{} | {} : {}", graph[current_node], 
        graph[source], graph[target]);
      if let Some(new_value) = graph[source].checked_add(graph[target]) {
        graph[source] = new_value;
      } else {
        graph[source] = u32::MAX;
      }
      stack.push(source);
    }
  }
}
