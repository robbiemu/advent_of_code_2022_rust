use petgraph::stable_graph::{StableGraph, NodeIndex};
use petgraph::{Directed, Incoming};


pub fn du_directories(
  g: StableGraph<u64, u64, Directed>
) -> StableGraph<u64, u64, Directed> {
  let mut graph = g.clone();
  let file_indices = g.node_indices().filter(|&i| g[i] > 0);
  for node in file_indices {
    propagate_values_up_tree(&mut graph, node);
    graph.remove_node(node); // graph becomes only directories
  }

  graph
}

fn propagate_values_up_tree(
  graph: &mut StableGraph<u64, u64, Directed>, 
  node: NodeIndex,
) {
  let mut stack = vec![node];
  
  while let Some(current_node) = stack.pop() {
    let mut edges = graph.neighbors_directed(current_node, Incoming).detach();
    while let Some(edge) = edges.next_edge(graph) {
      let (source, target) = graph.edge_endpoints(edge).unwrap();
      println!("{} | {} : {}", graph[current_node], 
        graph[source], graph[target]);
      if let Some(new_value) = graph[source].checked_add(graph[node]) {
        graph[source] = new_value;
      } else {
        graph[source] = u64::MAX;
      }
      stack.push(source);
    }
  }
}
