use bevy::prelude::{ResMut, Resource};
use petgraph::prelude::*;
use petgraph::Graph;

use crate::common::factory_graph_from_map;


#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum ModeState {
  #[default]
  Inactive,
  Active,
}

#[derive(Default, Debug)]
pub struct Map {
  pub graph: Graph<usize, ()>,
  pub size: (usize, usize),
  pub flat: Vec<char>,
  pub start: Option<usize>,
  pub end: Option<usize>,
  pub solution: Option<(i32, Vec<NodeIndex>)>,
}

impl Clone for Map {
  fn clone(&self) -> Self {
    let solution = self.solution.as_ref().map(|(distance, path)| {
      let cloned_path = path
        .iter()
        .map(|path_node| {
          self
            .graph
            .node_indices()
            .find(|i| self.graph[*i] == path_node.index())
            .unwrap()
        })
        .collect::<Vec<NodeIndex>>();
      (*distance, cloned_path)
    });
    Map {
      graph: self.graph.clone(),
      size: self.size,
      flat: self.flat.clone(),
      start: self.start,
      end: self.end,
      solution,
    }
  }
}

pub fn factory_map(input: String) -> Option<Map> {
  if !verify_map_input(input.clone()) {
    return None;
  }
  let map: Vec<String> = input.lines().map(|l| l.to_string()).collect();
  let size = (map.len(), map[0].len());
  let (start, end, graph, flat) = factory_graph_from_map(map);

  Some(Map { graph, size, flat, start, end, solution: None })
}

pub trait DataEvent<T, U> {
  fn get_event_type(&self) -> T;
  fn get_data(&self) -> Option<U>;
}

pub trait Clear {
  fn clear(&mut self);
}

impl<'a, T> Clear for ResMut<'a, T>
where
  T: Resource + Default,
{
  fn clear(&mut self) {
    let data = &mut **self;
    *data = T::default();
  }
}

pub fn verify_map_input(input: String) -> bool {
  let mut has_start = false;
  let mut has_end = false;
  let mut width = None;

  for line in input.lines() {
    if line.is_empty() {
      continue;
    }

    if width.is_none() {
      width = Some(line.len());
    } else if line.len() != width.unwrap() {
      return false; // Return false if the line width doesn't match the previous lines
    }

    for c in line.chars() {
      match c {
        'S' => {
          if has_start {
            return false; // Return false if there is more than one start position
          }
          has_start = true;
        }
        'E' => {
          if has_end {
            return false; // Return false if there is more than one end position
          }
          has_end = true;
        }
        _ => {
          if !c.is_ascii_lowercase() {
            return false; // Return false if any character is not lowercase
          }
        }
      }
    }
  }

  has_start == has_end // Return true if the number of start and end positions match
}
