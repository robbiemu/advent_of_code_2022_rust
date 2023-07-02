use std::collections::HashSet;

use dioxus::prelude::*;

use super::AppState;
use crate::common::{derive_bounds, parse_line, prelude::*, manhattan_distance};
use super::{sensor_and_container::sensor_and_container, beacon::beacon_entity};


pub struct Part2State {
  min_bound: isize,
  max_bound: isize,
  solution: Option<(isize, isize)>
}

pub fn part_2(cx: Scope) -> Element {
  let app_state = use_shared_state::<AppState>(cx).unwrap();
  let min_bound_input = use_ref(cx, || isize::MIN);
  let max_bound_input = use_ref(cx, || isize::MAX);

  let input = &app_state.read().puzzle;
  let records: Vec<Record> = input
    .clone()
    .lines()
    .filter_map(|l| parse_line(l.to_string()))
    .collect();
  let bounds = derive_bounds(&records);
  let (min_x, min_y) = bounds.0;
  let (max_x, max_y) = bounds.1;
  let min_bound = min_x.max(min_y) as f64;
  let max_bound = max_x.max(max_y) as f64;
  let state = use_ref(cx, || Part2State {
    min_bound: min_bound as isize,
    max_bound: max_bound as isize,
    solution: None
  });

  let solution: Option<(isize, isize)>;
  {
    solution = state.read().solution;
  }
  let tuning_frequency: Option<i64> = solution.map(|(x,y)| x as i64 * 4_000_000_i64 + y as i64);

  let style = ".part-2 div { font-family: sans-serif; font-size: 6pt \
               }\n.part-2  .sensor::after { content: 'S' }\n.part-2  \
               .beacon::after { content: 'B' }";

  cx.render(rsx!(
    section {
      class: "part-2",
      style: "display: flex; flex-direction: column; align-items: center;",

      header {
        h2 {
          "Part 2"
        }
      }

      div {
        class: "control",
        style: "display: flex; flex-direction: row",

        div {
          label {
            "for": "min-bound"
          }
          input {
            name: "min-bound",
            "type": "number",
            pattern: r"\d*",
            "min": min_bound,
            "max": max_bound,
            onchange: move |e: FormEvent| min_bound_input.with_mut(|bound| *bound = e.value.parse::<isize>().unwrap()) 
          }
          button {
            onclick: move |_| {
              let min_bound = *min_bound_input.read();
              let max_bound = *max_bound_input.read();
              if max_bound > min_bound {
                state.with_mut(|s| s.min_bound = min_bound);
              }            
            },

            "set"
          }
        }
        div {
          label {
            "for": "max-bound"
          }
          input {
            name: "max-bound",
            "type": "number",
            pattern: r"\d*",
            "min": min_bound,
            "max": max_bound,
            onchange: move |e: FormEvent| max_bound_input.with_mut(|bound| *bound = e.value.parse::<isize>().unwrap()) 
          }
          button {
            onclick: move |_| {
              let max_bound = *max_bound_input.read();
              let min_bound = *min_bound_input.read();
              if max_bound > min_bound {
                state.with_mut(|s| s.max_bound = max_bound);
              }
            },

            "set"
          }
        }
      }

      div {
        class: "control cta",
        style: "min-height: 18.5px",

        if *max_bound_input.read() < isize::MAX && *min_bound_input.read() > isize::MIN {
          let rc = records.clone();
          rsx!(
            button {
              onclick: move |_| {
                let max_bound = *max_bound_input.read();
                let min_bound = *min_bound_input.read();
                state.with_mut(|s| s.solution = solve(min_bound, max_bound, &rc))
              },
      
              "solve"
            }
          )
        }
      }

      section {
        class: "solution",

        header {
          h3 { "Solution" }
        }

        div {
          style: "min-height: 18.5px; font-size: 1rem",

          if let Some((x,y)) = solution {
            let frequency = tuning_frequency.unwrap();
            rsx!(
              strong {
                "tuning frequency {frequency} = x:{x} * 4e6 + y:{y}"
              }
            )
          } else {
            rsx!( span { style: "opacity: 0.4", "no solution found" })
          }
        }
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
      }
    }
  ))
}

fn solve(min_bound: isize, max_bound: isize, records: &[Record]) -> Option<(isize, isize)> {
  /* remembering that we can skip points that are close enough to a sensor 
  to detect. */
  let sensors_at_distance: Vec<(Entity, usize)> = records.iter().map(|record| (record.0.clone(), manhattan_distance(record))).collect();

  let results = sensors_at_distance.iter().flat_map(|(sensor_left, distance_left)| {
    sensors_at_distance
      .iter()
      .flat_map(move |(sensor_right, distance_right)| 
        /* get the outermost points at manhattan_distance + 1 for this sensor, 
        intersected with the same from the other sensor. 

        These sensors weren't chosen by distance, so we are going to get a bunch
        of nonsense points as well. Not to worry, if a point is still in the 
        range, and no sensor can detect it, that will suffice. we will check 
        that next. */
        get_intersection_points(sensor_left.get_coord(), (distance_left + 1) as isize, sensor_right.get_coord(), (distance_right + 1) as isize)
      )
      .filter(|point| 
        /* we only care about valid points in the search space */
        point.0 >= min_bound && point.0 <= max_bound && point.1 >= min_bound && point.1 <= max_bound)
      .filter_map(|point| {
        /* and we only want a point that no sensor is close enough to detect */
        if sensors_at_distance.iter().all(|(s, d)|    
      manhattan_distance(&(s.clone(), Entity::Beacon(point))) > *d) {
          Some(point.to_owned())
        } else {
          None
        }
      }).collect::<Vec<(isize, isize)>>()
  }).collect::<HashSet<_>>();

  log::info!("{:?}", results);

  match results.len() {
    1 => Some(*results.iter().next().unwrap()),
    _ => None
  }
}

fn get_intersection_points(sensor_left: (isize, isize), distance_left: isize, sensor_right: (isize, isize), distance_right: isize) -> Vec<(isize, isize)> {
  let left_points = intersection_points(sensor_left.0, sensor_left.1, distance_left, sensor_right.0, sensor_right.1, distance_right);
  let right_points = intersection_points(sensor_right.0, sensor_right.1, distance_right, sensor_left.0, sensor_left.1, distance_left);
  
  [left_points, right_points].concat()
}

fn intersection_points(s1x: isize, s1y: isize, d1: isize, s2x: isize, s2y: isize, d2: isize) -> Vec<(isize, isize)> {
  /* we are returning the interesection of lines around two points. the equation
  of a line is y = m x + c. the intersection of two lines y = mx + c and 
  z = -mx + d intersect is (mx+c/2, -mx+d/2) 
  
  in our case, we don't have slopes we have a distance to offset by and a sensor
  s(x,y). so we have lines:
    y = x - sy-sx+d - 1
    y = x - sy-sx+d + 1
    y = x + sy-sx+d - 1
    y = x + sy-sx+d + 1
  */
  let variants = vec![
      (s1x - d1, s2x - d2),
      (s1x - d1, s2x + d2),
      (s1x + d1, s2x - d2),
      (s1x + d1, s2x + d2),
  ];
  
  /* lines y=x+a and y=-x+b intersect at ((b - a)/2 , (b + a)/2). in this case,
  we invert the sign of s1y to "walk" 1 step around the rotation (the lines in
  question are at 90 degree intervals) */
  variants
    .into_iter()
    .map(|(x1, x2)| ((x2 + s2y + x1 - s1y) / 2, (x2 + s2y - x1 + s1y) / 2))
    .collect()
}

