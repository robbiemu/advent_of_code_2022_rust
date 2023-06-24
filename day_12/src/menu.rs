use bevy::prelude::{
  App, EventReader, EventWriter, Plugin, Res, ResMut, Resource,
};
use bevy_egui::{egui, EguiContexts};

use super::{GameMode, GameState, APP_TITLE};
use crate::{
  bevy_common::{factory_map, Clear, Map, ModeState},
  SolveMode, PART1_NAME, PART1_TITLE, PART2_NAME, PART2_TITLE,
};


const MENU_TITLE: &str = "Menu";
const CTA_PLAY: &str = "Play";
const CTA_EXIT: &str = "Exit";
const LOAD_MAP_MODAL_TITLE: &str = "Load Map";
const ENTER_MAP_PROMPT: &str = "Enter map data here.";

#[derive(Debug, Eq, PartialEq)]
pub enum Event {
  MenuStart,
  MenuEnd,
  PlayClicked,
  ExitClicked,
  GoPart1Clicked,
  GoPart2Clicked,
}

#[derive(Resource, Clone, Default, Debug)]
struct MenuState {
  current_state: Option<ModeState>,
  map: Option<String>,
  is_showing_load_map_ui: bool,
  raw_map: String,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<MenuState>()
      .add_system(menu_system)
      .add_system(load_map_system)
      .add_system(menu_events)
      .add_event::<Event>();
  }
}

fn menu_system(
  game_state: Res<GameState>,
  local: ResMut<MenuState>,
  mut contexts: EguiContexts,
  mut events: EventWriter<Event>,
) {
  if game_state.mode == GameMode::Menu {
    if Some(ModeState::Active) != local.current_state {
      events.send(Event::MenuStart);
      return;
    }
    egui::TopBottomPanel::top(MENU_TITLE).show(contexts.ctx_mut(), |ui| {
      ui.horizontal(|ui| ui.heading(APP_TITLE));
      ui.horizontal(|ui| {
        ui.label(MENU_TITLE);
      });
      ui.horizontal(|ui| {
        if ui.button(CTA_PLAY).clicked() {
          events.send(Event::PlayClicked);
        };
        if ui.button(CTA_EXIT).clicked() {
          events.send(Event::ExitClicked);
        };
      })
    });
  } else if Some(ModeState::Active) == local.current_state {
    events.send(Event::MenuEnd)
  }
}

fn load_map_system(
  game_state: Res<GameState>,
  mut local: ResMut<MenuState>,
  mut contexts: EguiContexts,
  mut events: EventWriter<Event>,
) {
  if !(game_state.mode == GameMode::Menu
    && Some(ModeState::Active) == local.current_state)
  {
    return;
  }
  let mut is_showing_load_map_ui = local.is_showing_load_map_ui;
  let load_map_ui = egui::Window::new(LOAD_MAP_MODAL_TITLE)
    .collapsible(false)
    .open(&mut is_showing_load_map_ui);
  load_map_ui.show(contexts.ctx_mut(), |ui| {
    ui.horizontal(|ui| {
      ui.heading(LOAD_MAP_MODAL_TITLE);
    });
    ui.vertical(|ui| {
      ui.label(ENTER_MAP_PROMPT);
      ui.text_edit_multiline(&mut local.raw_map);
      ui.vertical(|ui| {
        ui.vertical(|ui| {
          ui.label(PART1_TITLE);
          if ui.button(PART1_NAME).clicked() {
            local.map = Some(local.raw_map.to_string());
            events.send(Event::GoPart1Clicked);
          };
        });
        ui.vertical(|ui| {
          ui.label(PART2_TITLE);
          if ui.button(PART2_NAME).clicked() {
            local.map = Some(local.raw_map.to_string());
            events.send(Event::GoPart2Clicked);
          };
        });
      });
    })
  });
  local.is_showing_load_map_ui = is_showing_load_map_ui;
}

fn menu_events(
  mut game_state: ResMut<GameState>,
  mut local: ResMut<MenuState>,
  mut menu_event_reader: EventReader<Event>,
) {
  for event in menu_event_reader.iter() {
    match event {
      Event::MenuStart => {
        local.clear();
        local.current_state = Some(ModeState::Active);
      }
      Event::MenuEnd => {
        leave_menu_mode(&mut local, &mut game_state);
        break;
      }
      Event::PlayClicked => local.is_showing_load_map_ui = true,
      Event::ExitClicked => {
        local.clear();
        local.is_showing_load_map_ui = false;
        game_state.mode = GameMode::Exit;
      }
      Event::GoPart1Clicked => {
        if let Some(input) = &local.map {
          let map = factory_map(input.clone());
          if map.is_some() {
            go(&mut game_state, map, SolveMode::Part1);
            break;
          }
        }
      }
      Event::GoPart2Clicked => {
        if let Some(input) = &local.map {
          let map = factory_map(input.clone());
          if map.is_some() {
            go(&mut game_state, map, SolveMode::Part2);
            break;
          }
        }
      }
    }
  }
}

fn leave_menu_mode(local: &mut ResMut<MenuState>, game_state: &mut GameState) {
  local.clear();
  local.current_state = Some(ModeState::Inactive);
  if game_state.mode == GameMode::Menu {
    game_state.mode = GameMode::Exit;
  }
}

fn go(game_state: &mut ResMut<GameState>, map: Option<Map>, mode: SolveMode) {
  game_state.map = map;
  game_state.solve_mode = mode;
  game_state.mode = GameMode::Map;
}
