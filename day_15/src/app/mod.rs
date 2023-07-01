use dioxus::prelude::*;
use std::fmt::{Display, Formatter};

mod input_puzzle;
use input_puzzle::input_puzzle;
mod start_puzzle;
use start_puzzle::start_puzzle;
mod part_select;
use part_select::part_select;
mod part1;
use part1::part_1;
mod beacon;
mod sensor_and_container;


#[derive(Clone, Default, Debug, PartialEq)]
enum AppMode {
  #[default]
  Start,
  Modal,
  PartSelection,
  Part1,
  Part2,
}

impl Display for AppMode {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    match self {
      AppMode::Start => f.write_str("Start"),
      AppMode::Modal => f.write_str("Modal"),
      AppMode::PartSelection => f.write_str("PartSelection"),
      AppMode::Part1 => f.write_str("Part1"),
      AppMode::Part2 => f.write_str("Part2"),
    }
  }
}

#[derive(Default)]
struct AppState {
  mode: AppMode,
  puzzle: String,
}

pub fn app(cx: Scope) -> Element {
  use_shared_state_provider(cx, AppState::default);
  let mode: AppMode;
  {
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    let state = app_state.read();
    mode = state.mode.clone();
  }

  cx.render(rsx! (
    main {
      class: "app",
      style: "text-align: center;",

      header {
        h1 { "🌗 Day 15" }
        h3 { "Advent of code 2022" }
        p { "🚀 by Robbiemu" }
      }

      match mode {
        AppMode::Start => rsx!(start_puzzle {} ),
        AppMode::Modal => rsx!(input_puzzle {}),
        AppMode::PartSelection => rsx!(part_select {}),
        AppMode::Part1 => rsx!(part_1 {}),
        AppMode::Part2 => unimplemented!(),
      }
    }
  ))
}
