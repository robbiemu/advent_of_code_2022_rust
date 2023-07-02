use dioxus::prelude::*;

use crate::{app::AppState, common::{prelude::*, parse_line, derive_bounds}};
use super::{count_row, Part1State};


pub fn rows_input(cx: Scope) -> Element {
  let app_state = use_shared_state::<AppState>(cx).unwrap();
  let part1_state = use_shared_state::<Part1State>(cx).unwrap();

  let row_selected; 
  {
    row_selected = part1_state.read().row_selected;
  }

  let input = &app_state.read().puzzle;
  let records: Vec<Record> = input
    .clone()
    .lines()
    .filter_map(|l| parse_line(l.to_string()))
    .collect();
  let bounds = derive_bounds(&records);
  let b_min = bounds.0.1;
  let b_max = bounds.1.1 - 1;

  cx.render(
  rsx!( 
    div {
      class: "control",

      input {
        style: "min-width: 15em",
        "type": "number",
        placeholder: "between {b_min} and {b_max}",
        min: b_min as f64,
        max: b_max as f64,
        onchange: move |e: FormEvent| part1_state.write().row_selected = Some((e.value.parse::<isize>().unwrap() - b_min) as usize)
      }
      button {
        onclick: move |_| {
          log::info!("clicked");
          if let Some(j) = row_selected {
            log::info!("selected: {j}");
            let count = count_row(j, bounds, &records);
            part1_state.write().row_count = Some(count);
          }
        },

        "go"
      }
    }
  ))
}
