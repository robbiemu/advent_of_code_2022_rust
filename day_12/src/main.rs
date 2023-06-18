use bevy::{
  app::AppExit,
  prelude::{App, ClearColor, Color, EventWriter, Res, Resource},
  DefaultPlugins,
};

mod bevy_common;
use bevy_common::Map;
mod common;
mod map;
use map::MapPlugin;
mod menu;
use menu::MenuPlugin;


fn main() {
  App::new()
    .init_resource::<GameState>()
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .add_plugins(DefaultPlugins)
    .add_plugin(MenuPlugin)
    .add_plugin(MapPlugin)
    .add_system(exit_system)
    .run();
}

fn exit_system(game_state: Res<GameState>, mut exit: EventWriter<AppExit>) {
  if game_state.mode == GameMode::Exit {
    exit.send(AppExit);
  }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum GameMode {
  #[default]
  Menu,
  Map,
  Exit,
}

#[derive(Resource, Clone, Default, Debug)]
pub struct GameState {
  mode: GameMode,
  map: Option<Map>,
}

const _APP_NAME: &str = "hill-climbing";
const APP_TITLE: &str =
  "Advent of Code 2022 - day 12 - hill climbing algorithm";
const _PART1_NAME: &str = "part 1";
const _PART1_TITLE: &str = "fewest steps";
