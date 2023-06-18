use petgraph::prelude::*;
use petgraph::{algo::astar, Graph};

pub fn factory_graph_from_map(
  map: Vec<String>,
) -> (Option<usize>, Option<usize>, Graph<usize, ()>, Vec<char>) {
  let height = map.len();
  let width = map[0].len();
  let mut flattened: Vec<char> = map
    .into_iter()
    .flat_map(|l| l.chars().collect::<Vec<char>>())
    .collect();
  let start = flattened.iter().position(|&ch| ch == 'S');
  if let Some(s) = start {
    flattened[s] = 'a';
  }
  let end = flattened.iter().position(|&ch| ch == 'E');
  if let Some(e) = end {
    flattened[e] = 'z';
  }

  let edges = factory_edges(&flattened, height, width);
  let graph = factory_graph(edges, width * height);

  (start, end, graph, flattened)
}

fn factory_edges(
  flattened: &[char],
  height: usize,
  width: usize,
) -> Vec<(usize, usize)> {
  let mut edges = Vec::new();
  for (index, &ch) in flattened.iter().enumerate() {
    let row = index / width;
    let col = index % width;
    let current_node = row * width + col;
    let next_char = (ch as u8 + 1) as char;
    if row > 0 && flattened[index - width] <= next_char {
      let target_node = (row - 1) * width + col;
      edges.push((current_node, target_node));
    }

    if row < height - 1 && flattened[index + width] <= next_char {
      let target_node = (row + 1) * width + col;
      edges.push((current_node, target_node));
    }

    if col > 0 && flattened[index - 1] <= next_char {
      let target_node = row * width + (col - 1);
      edges.push((current_node, target_node));
    }

    if col < width - 1 && flattened[index + 1] <= next_char {
      let target_node = row * width + (col + 1);
      edges.push((current_node, target_node));
    }
  }

  edges
}

fn factory_graph(
  edges: Vec<(usize, usize)>,
  num_nodes: usize,
) -> Graph<usize, ()> {
  let mut graph = Graph::<_, ()>::with_capacity(num_nodes, edges.len());
  let node_indices: Vec<_> =
    (0..num_nodes).map(|i| graph.add_node(i)).collect();

  for (src, dst) in edges {
    graph.add_edge(node_indices[src], node_indices[dst], ());
  }

  graph
}

pub fn find_path_part1(
  graph: Graph<usize, ()>,
  start_index: usize,
  end_index: usize,
) -> Option<(i32, Vec<NodeIndex>)> {
  let start = graph
    .node_indices()
    .find(|i| graph[*i] == start_index)
    .unwrap();
  let end = graph
    .node_indices()
    .find(|i| graph[*i] == end_index)
    .unwrap();

  dbg!(start, end);

  astar(&graph, start, |curr| curr == end, |_| 1, |_| 0)
}
