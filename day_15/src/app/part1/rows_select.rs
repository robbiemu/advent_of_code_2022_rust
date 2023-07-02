use dioxus::prelude::*;

use crate::{app::AppState, common::{derive_bounds, parse_line, prelude::*}};
use super::{Part1State, count_row};


pub fn rows_select(cx: Scope) -> Element {
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
  let bounds_height = bounds.get_height() as usize;
  let b_min = bounds.0.1;

  cx.render(
    rsx! (
      ul {
        style: "margin: 0; padding: 0; list-style: none; position: absolute; width: 100%; height: 100%; display: flex; flex-direction: column",
        
        (0..bounds_height).map(|i| {
          let selection_style = match row_selected{
            Some(j) if j == i => "background-color: rgba(255, 255, 128, 0.2)",
            _ => ""
          };
          let rc = records.clone();
          rsx!(
            li {
              key: "urow_{i}",
              style: "cursor: pointer; flex: 1; display: flex; align-items: center; justify-content: center; {selection_style}",
              onclick: move |_| {
                let mut writer = part1_state.write();
                match row_selected {
                  Some(_) => {
                    writer.row_selected = None;
                    writer.row_count = None;
                  },
                  None => {
                    let count = count_row(i, bounds, &rc);
                    writer.row_count = Some(count);
                    writer.row_selected = Some(i);
                  }
                }
              },
              
              if row_selected.is_some() && row_selected.unwrap() == i {
                rsx!( 
                  strong { 
                    style: "color: khaki; font-size: 12pt;",
                    
                    "row: {i as isize + b_min}" 
                  } 
                )  
              }
            }
          )
        })
      }
    )
  )
}
