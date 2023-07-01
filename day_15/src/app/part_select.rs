use dioxus::prelude::*;

use super::{AppMode, AppState};


pub fn part_select(cx: Scope) -> Element {
  let app_state = use_shared_state::<AppState>(cx).unwrap();

  cx.render(rsx!(
    section {
      class: "part-select",
      style: "display: flex; flex-direction: column; align-items: center;",

      header {
        h2 {
          "Select a puzzle visualization mode"
        }
      }
      div {
        class: "control",
        style: "display: flex; justify-content: space-around; padding: 1em; border: solid thin whitesmoke",

        button {
          name: "cta-part1",
          onclick: |_| app_state.write().mode = AppMode::Part1,

          "part 1"
        }
        button {
          name: "cta-part2",
          onclick: |_| app_state.write().mode = AppMode::Part2,

          "part 2"
        }
      }
    }
  ))
}
