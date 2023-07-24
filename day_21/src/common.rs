pub mod prelude {
  use std::convert::TryInto;
  use std::hash::{Hash, Hasher};


  pub const ROOT_NAME: &str = "root";

  #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
  pub struct Participants {
    pub left: i64,
    pub right: i64,
  }

  #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
  pub enum FnType {
    Add(Option<Participants>),
    Div(Option<Participants>),
    Sub(Option<Participants>),
    Mul(Option<Participants>),
    Eq(Option<Participants>),
    Value(i64),
  }

  impl TryInto<&'static str> for FnType {
    type Error = ();

    fn try_into(self) -> Result<&'static str, ()> {
      match self {
        FnType::Add(_) => Ok("+"),
        FnType::Div(_) => Ok("/"),
        FnType::Sub(_) => Ok("-"),
        FnType::Mul(_) => Ok("*"),
        FnType::Eq(_) => Ok("="),
        FnType::Value(_) => Err(()),
      }
    }
  }


  impl Hash for FnType {
    fn hash<H: Hasher>(&self, state: &mut H) {
      core::mem::discriminant(self).hash(state);
    }
  }

  impl FnType {
    pub fn from_components(components: &[&str]) -> FnType {
      let len = components.len();
      if len > 3 || len == 2 {
        panic!("invalid function type {:?}", components)
      }
      if len == 1 {
        return FnType::Value(components[0].parse().unwrap_or_else(|_| {
          panic!("invalid function type '{}'", components[0])
        }));
      }

      let mut t = FnType::get_ops_type_from_str(components[1]);
      let left = MonkeyBusiness::id_from_str(components[0]);
      let right = MonkeyBusiness::id_from_str(components[2]);
      let participants = Participants { left, right };
      t.set_participants(participants);

      t
    }

    pub fn get_ops_type_from_str(value: &str) -> FnType {
      match value {
        "+" => FnType::Add(None),
        "/" => FnType::Div(None),
        "-" => FnType::Sub(None),
        "*" => FnType::Mul(None),
        "=" => FnType::Eq(None),
        _ => panic!("invalid function operation type '{value}'"),
      }
    }

    pub fn get_value(&self) -> Option<i64> {
      match self {
        FnType::Value(n) => Some(*n),
        _ => None,
      }
    }

    pub fn set_participants(&mut self, participants: Participants) {
      match self {
        FnType::Add(ref mut opt)
        | FnType::Div(ref mut opt)
        | FnType::Sub(ref mut opt)
        | FnType::Mul(ref mut opt)
        | FnType::Eq(ref mut opt) => *opt = Some(participants),
        FnType::Value(_) => {
          panic!("FnType::Value does not accept participants")
        }
      }
    }

    #[allow(dead_code)]
    pub fn get_participants(&self) -> Option<Participants> {
      match self {
        FnType::Add(opt)
        | FnType::Div(opt)
        | FnType::Sub(opt)
        | FnType::Mul(opt)
        | FnType::Eq(opt) => *opt,
        FnType::Value(_) => None,
      }
    }

    pub fn apply(
      fn_type: FnType,
      inputs_1: MonkeyBusiness,
      inputs_2: MonkeyBusiness,
    ) -> i64 {
      let left = inputs_1
        .fn_type
        .get_value()
        .unwrap_or_else(|| panic!("unexpected empty value"));
      let right = inputs_2
        .fn_type
        .get_value()
        .unwrap_or_else(|| panic!("unexpected empty value"));
      match fn_type {
        FnType::Add(_) => left + right,
        FnType::Div(_) => left / right,
        FnType::Sub(_) => left - right,
        FnType::Mul(_) => left * right,
        FnType::Eq(_) => {
          if left == right {
            1
          } else {
            0
          }
        }
        _ => panic!("invalid function operation type '{:?}'", fn_type),
      }
    }
  }

  #[derive(Clone, Copy, Debug, Eq, PartialOrd, Ord)]
  pub struct MonkeyBusiness {
    pub id: i64,
    pub fn_type: FnType,
  }

  impl Hash for MonkeyBusiness {
    fn hash<H: Hasher>(&self, state: &mut H) {
      self.id.hash(state);
      self.fn_type.hash(state);
    }
  }

  impl PartialEq for MonkeyBusiness {
    fn eq(&self, other: &Self) -> bool {
      self.id == other.id
    }
  }

  impl MonkeyBusiness {
    pub fn id_from_str(name_str: &str) -> i64 {
      str_to_base26_number(name_str)
        .unwrap_or_else(|| panic!("invalid monkey name {name_str}"))
    }

    pub fn string_from_id(id: i64) -> String {
      base26_number_to_str(id)
        .unwrap_or_else(|| panic!("invalid monkey ID {}", id))
    }
  }

  fn str_to_base26_number(s: &str) -> Option<i64> {
    let base: i64 = 26;
    let mut result: i64 = 0;

    for c in s.chars() {
      let digit = (c as u8).to_ascii_lowercase() - b'a';
      if !(0..26).contains(&digit) {
        return None;
      }
      result = result * base + i64::from(digit);
    }

    Some(result)
  }

  fn base26_number_to_str(mut n: i64) -> Option<String> {
    const BASE: i64 = 26;
    const A: u8 = b'a';

    if n < 0 {
      return None;
    }

    let mut result = String::new();
    while n > 0 {
      let digit = (n % BASE) as u8;
      let char_digit = char::from(A + digit);
      result.insert(0, char_digit);
      n /= BASE;
    }

    Some(result)
  }

  pub enum NodeReductionLimit {
    Some(usize),
    All,
  }
}

use petgraph::prelude::*;
use std::collections::HashMap;

use prelude::*;


pub fn parse_nodes(
  lines: impl Iterator<Item = String>,
) -> GraphMap<MonkeyBusiness, usize, Directed> {
  lines
    .map(|l| {
      let (name_str, fn_str) = l.split_once(':').unwrap();
      let name = MonkeyBusiness::id_from_str(name_str);
      let fn_components: Vec<&str> = fn_str.split_whitespace().collect();
      let fn_type = FnType::from_components(&fn_components);

      MonkeyBusiness { id: name, fn_type }
    })
    .fold(
      GraphMap::<MonkeyBusiness, usize, Directed>::new(),
      |mut acc, cur| {
        acc.add_node(cur);

        acc
      },
    )
}

pub fn apply_edges(
  lines: impl Iterator<Item = String>,
  graph: &mut GraphMap<MonkeyBusiness, usize, Directed>,
) {
  let nodes: HashMap<i64, MonkeyBusiness> =
    graph.nodes().map(|mb| (mb.id, mb)).collect();

  lines
    .filter_map(|l| {
      let (name_str, fn_str) = l.split_once(':').unwrap();

      let fn_components: Vec<String> =
        fn_str.split_whitespace().map(|s| s.to_owned()).collect();
      let fn_components_slice: Vec<&str> =
        fn_components.iter().map(|s| s.as_str()).collect();
      let fn_type = FnType::from_components(&fn_components_slice);

      match fn_type {
        FnType::Value(_) => None,
        _ => {
          let left = fn_components.first().unwrap().to_owned();
          let right = fn_components.last().unwrap().to_owned();
          Some((name_str.to_owned(), (left, right)))
        }
      }
    })
    .for_each(|(to, (left, right))| {
      let c = nodes[&MonkeyBusiness::id_from_str(&to)];
      let a = nodes[&MonkeyBusiness::id_from_str(&left)];
      let b = nodes[&MonkeyBusiness::id_from_str(&right)];
      graph.add_edge(a, c, 0);
      graph.add_edge(b, c, 0);
    });
}

pub fn reduce_nodes(
  graph: GraphMap<MonkeyBusiness, usize, Directed>,
  limit: NodeReductionLimit,
) -> Vec<MonkeyBusiness> {
  let nodes: HashMap<i64, MonkeyBusiness> =
    graph.nodes().map(|mb| (mb.id, mb)).collect();
  let root = *nodes.get(&MonkeyBusiness::id_from_str(ROOT_NAME)).unwrap();
  println!("root {:?}", root);

  let mut g: Graph<MonkeyBusiness, usize, Directed, _> =
    graph.clone().into_graph();
  g.reverse();
  let mut search_nodes: Vec<MonkeyBusiness> = Vec::new();
  let root_index = g
    .node_indices()
    .find(|i: &NodeIndex| g[*i] == root)
    .unwrap();
  let mut bfs = Bfs::new(&g, root_index);
  while let Some(mbi) = bfs.next(&g) {
    search_nodes.push(g[mbi]);
  }

  match limit {
    NodeReductionLimit::All => search_nodes,
    NodeReductionLimit::Some(limit) => {
      search_nodes_to_limit(search_nodes, limit, graph, nodes)
    }
  }
}

fn search_nodes_to_limit(
  mut search_nodes: Vec<MonkeyBusiness>,
  limit: usize,
  graph: GraphMap<MonkeyBusiness, usize, Directed>,
  mut nodes: HashMap<i64, MonkeyBusiness>,
) -> Vec<MonkeyBusiness> {
  let mut values: HashMap<MonkeyBusiness, i64> = HashMap::new();
  let mut nodes_to_remove = Vec::new();

  while search_nodes.len() > limit {
    search_nodes.iter_mut().for_each(|mb| {
      let inputs: Vec<MonkeyBusiness> = graph
        .edges_directed(*mb, Incoming)
        .filter_map(|(n, _, _)| {
          let from = nodes[&n.id];
          match from.fn_type {
            FnType::Value(_) => Some(from),
            _ => None,
          }
        })
        .collect();

      if inputs.len() != 2 {
        return;
      }
      println!("inputs {:?}", inputs);

      let value = FnType::apply(mb.fn_type, inputs[0], inputs[1]);
      mb.fn_type = FnType::Value(value);
      if let Some(n) = nodes.get_mut(&mb.id) {
        n.fn_type = mb.fn_type;
        println!("changed Ops Node {:?}", nodes[&mb.id]);
      }
      values.insert(*mb, value);
      nodes_to_remove.push(inputs[0]);
      nodes_to_remove.push(inputs[1]);
    });

    if nodes_to_remove.is_empty() {
      break;
    }

    for mb in nodes_to_remove.iter().rev() {
      let i = search_nodes
        .iter()
        .position(|n| *n == *mb)
        .unwrap_or_else(|| panic!("node is not in search nodes"));
      search_nodes.remove(i);
    }
    nodes_to_remove.clear();
  }

  search_nodes
}
