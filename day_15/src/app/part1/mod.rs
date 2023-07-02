use dioxus::prelude::*;
use std::collections::HashSet;

use super::beacon::beacon_entity;
use super::sensor_and_container::sensor_and_container;
use super::AppState;
use crate::common::{
  derive_bounds, extend_coord_ranges, get_bounded_coordinate_indices,
  manhattan_distance, parse_line, prelude::*, solve_to,
};
mod rows_select;
use rows_select::rows_select;
mod rows_input;
use rows_input::rows_input;


#[derive(Default)]
pub struct Part1State {
  row_count: Option<usize>,
  row_selected: Option<usize>,
}

pub fn part_1(cx: Scope) -> Element {
  let app_state = use_shared_state::<AppState>(cx).unwrap();
  use_shared_state_provider(cx, Part1State::default);

  let row_count: Option<usize>;
  {
    let part1_state = use_shared_state::<Part1State>(cx).unwrap();
    let state = part1_state.read();
    row_count = state.row_count;
  }

  let input = &app_state.read().puzzle;
  let records: Vec<Record> = input
    .clone()
    .lines()
    .filter_map(|l| parse_line(l.to_string()))
    .collect();
  let bounds = derive_bounds(&records);
  log::info!("{:?}", bounds);
  let bounds_height = bounds.get_height() as usize;

  let style = ".part-1 div { font-family: sans-serif; font-size: 6pt \
               }\n.part-1  .sensor::after { content: 'S' }\n.part-1  \
               .beacon::after { content: 'B' }";

  cx.render(rsx!(
    section {
      class: "part-1",
      style: "display: flex; flex-direction: column; align-items: center;",

      header {
        h2 {
          "Part 1"
        }
      }

      if let Some(c) = row_count {
        rsx!( p { "row count {c}" } )
      } else {
        rsx!( p {
          style: "min-height: 18.5px"
        } )
      }
      if bounds_height >= 40 {
        rsx!( rows_input {} )
      }

      div {
        class: "content",
        style: "position: relative; aspect-ratio: {bounds.1.0 - bounds.0.0}/{bounds.1.1 - bounds.0.1}; width: 50vw; overflow: hidden; border: solid thin gray;",

        style {
          style
        }

        records.iter().map(|record| rsx!(
          sensor_and_container { record: record.clone(), bounds: bounds }
        ))
        records
          .iter()
          .map(|(_sensor, beacon)| beacon.clone())
          .collect::<HashSet<Entity>>()
          .into_iter()
          .map(|entity| rsx!(beacon_entity { beacon: entity, bounds: bounds }))

        if bounds_height < 40 {
          rsx!( rows_select {} )
        }
      }
    }
  ))
}

pub fn count_row(index: usize, bounds: Bounds, records: &[Record]) -> usize {
  let mut beacons: HashSet<usize> = HashSet::new();
  let mut ranges: Vec<Coord> = Vec::new();
  records.iter().for_each(|record| {
    let (sensor, beacon) = record;
    let (bx, by) = get_bounded_coordinate_indices(&bounds, beacon).unwrap();
    if by == index {
      beacons.insert(bx);
    }

    let path_length = manhattan_distance(record);
    let (x, y) = get_bounded_coordinate_indices(&bounds, sensor).unwrap();
    if y + path_length >= index && y - path_length <= index {
      //aco((x, y), path_length, &mut map);
      let range = solve_to((x, y), index, path_length, bounds);
      ranges = extend_coord_ranges(range, &mut ranges);
      log::info!("{:?}", ranges);
    }
  });

  log::info!(
    "ranges found: {:?} beacons found: {:?}",
    ranges,
    beacons.len()
  );

  (ranges.iter().fold(0, |acc, cur| acc + cur.1 - cur.0 + 1)
    - beacons.len() as isize) as usize
}
