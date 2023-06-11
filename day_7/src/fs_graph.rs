use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::Directed;
use std::collections::HashMap;


pub fn factory_fs_graph(
  lines: impl Iterator<Item = String>,
) -> StableGraph<u64, u64, Directed> {
  let mut graph = StableGraph::<u64, u64, Directed>::new();
  let shadow_root_index = graph.add_node(0);
  let mut cwd: Vec<NodeIndex> = vec![shadow_root_index];
  let mut parent_to_child: HashMap<NodeIndex, Vec<(String, NodeIndex)>> =
    HashMap::new();

  for line in lines {
    match identify_line_type(&line) {
      "Command" => change_directory(
        factory_current_working_directory(&line),
        &mut graph,
        &mut cwd,
        &mut parent_to_child,
      ),
      "Directory Response" => process_record(
        factory_directory_node(&line),
        &mut graph,
        &mut cwd,
        &mut parent_to_child,
      ),
      "File Response" => process_record(
        factory_file_node(&line),
        &mut graph,
        &mut cwd,
        &mut parent_to_child,
      ),
      _ => unreachable!(),
    }
  }

  println!(
    "{:?}",
    graph
      .edge_indices()
      .map(|i| graph.edge_endpoints(i).unwrap())
      .collect::<Vec<_>>()
  );

  /*let directory_names: HashMap<NodeIndex, String> = parent_to_child
  .values()
  .flat_map(|vec| vec.iter().cloned().map(|(name, index)| (index, name)))
  .collect();*/

  graph
}

fn identify_line_type(line: &str) -> &str {
  match line.trim_start().chars().next() {
    Some('$') => "Command",
    Some('d') => "Directory Response",
    Some(c) if c.is_ascii_digit() => "File Response",
    Some(_) => unreachable!(),
    None => unreachable!(),
  }
}

fn change_directory(
  d: Option<String>,
  graph: &mut StableGraph<u64, u64, Directed>,
  cwd: &mut Vec<NodeIndex>,
  parent_to_child: &mut HashMap<NodeIndex, Vec<(String, NodeIndex)>>,
) {
  if let Some(dir) = d {
    if dir == *".." {
      match cwd.is_empty() {
        false => {
          cwd.pop();
        }
        true => unreachable!(),
      }
    } else {
      let cwdi = cwd.last().cloned().unwrap();
      let child = parent_to_child.get(&cwdi).and_then(|children| {
        children.iter().find(|(dirname, _)| dirname == &dir)
      });
      if let Some((_, i)) = child {
        cwd.push(*i);
      } else {
        let i = graph.add_node(0);
        parent_to_child
          .entry(cwdi)
          .or_insert_with(Vec::<(String, NodeIndex)>::new);
        parent_to_child.get_mut(&cwdi).unwrap().push((dir, i));
        cwd.push(i);
      }
    }
  }
}

fn process_record(
  n: Option<(Option<String>, u64)>,
  graph: &mut StableGraph<u64, u64, Directed>,
  cwd: &mut [NodeIndex],
  parent_to_child: &mut HashMap<NodeIndex, Vec<(String, NodeIndex)>>,
) {
  let cwdi = cwd.last().cloned().unwrap();
  if let Some((dopt, size)) = n {
    let name = dopt.unwrap_or(String::new());
    let i = graph.add_node(size);
    graph.add_edge(cwdi, i, size);
    if !name.is_empty() {
      parent_to_child
        .entry(cwdi)
        .or_insert_with(Vec::<(String, NodeIndex)>::new);
      parent_to_child.get_mut(&cwdi).unwrap().push((name, i));
    }
  }
}

fn factory_current_working_directory(line: &str) -> Option<String> {
  line.strip_prefix("$ cd ").map(|dir| dir.to_string())
}

fn factory_file_node(line: &str) -> Option<(Option<String>, u64)> {
  let (size_str, _name) = line.split_once(' ')?;
  let size = size_str.parse::<u64>().ok()?;
  Some((None, size))
}

fn factory_directory_node(line: &str) -> Option<(Option<String>, u64)> {
  let name = line.strip_prefix("dir ")?;
  Some((Some(name.to_string()), 0))
}
