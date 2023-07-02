use dioxus::prelude::*;

use super::{AppMode, AppState};
use crate::common::validate_puzzle;


pub fn input_puzzle(cx: Scope) -> Element {
  let app_state = use_shared_state::<AppState>(cx).unwrap();

  let lines = use_state(cx, || app_state.read().puzzle.to_owned());
  let puzzle: String;
  {
    puzzle = lines.get().clone();
  }

  cx.render(rsx!(
    section {
      class: "input-puzzle",

      header {
        h2 {
          "Deploy sensors"
        }
      }
      div {
        label {
          "for": "puzzle-input",

          "Puzzle input"
        }
        textarea {
          name: "puzzle-input",
          value: "{lines}",
          oninput: move |event: FormEvent| lines.set(event.value.clone())
        }
        button {
          name: "cta-submit-puzzle",
          onclick: move |_| {
            if
            validate_puzzle(puzzle.clone()) {
              let mut setter = app_state.write();
              setter.puzzle = puzzle.clone();
              setter.mode = AppMode::PartSelection;
            }
          },

          "submit"
        }
      }
    }
  ))
}
