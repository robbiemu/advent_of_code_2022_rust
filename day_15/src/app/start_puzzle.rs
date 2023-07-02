use dioxus::prelude::*;

use super::{AppMode, AppState};


pub fn start_puzzle(cx: Scope) -> Element {
  let app_state = use_shared_state::<AppState>(cx).unwrap();

  cx.render(rsx!(
    section {
      class: "start",

      header {
        h2 {
          "Beacon Exclusion Zone"
        }
      }
      div {
        class: "control",

        label {
          "for": "cta-start-puzzle",

          "Start a puzzle"
        }
        button {
          name: "cta-start-puzzle",
          onclick: |_| app_state.write().mode = AppMode::Modal,

          "input"
        }
      }
    }
  ))
}
