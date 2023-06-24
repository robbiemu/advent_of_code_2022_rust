use bevy::{
  input::mouse::MouseMotion, prelude::*, render::mesh::Mesh,
  time::TimerMode::Repeating,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_mod_picking::prelude::*;
use std::collections::HashMap;

use super::{GameMode, GameState};
use crate::bevy_common::{Clear, ModeState};
use crate::common::find_path_part1;
mod constants;
use constants::*;
mod entity;
use entity::*;
mod camera;
use camera::*;
mod path;
use path::*;


#[derive(Debug)]
pub enum Event {
  MapStart,
  MapEnd,
  HideMap,
  ShowStartHighlight,
  ShowEndHighlight,
  HideStartHighlight,
  HideEndHighlight,
  FindPath,
}

#[derive(Resource, Default, Debug)]
pub struct MapState {
  spawned_entities: Vec<Entity>,
  original_materials:
    HashMap<Entity, (Handle<StandardMaterial>, Handle<StandardMaterial>)>,
  path_entities: Vec<Entity>,
  current_state: Option<ModeState>,
  is_showing_map: bool,
  is_rendering_path: bool,
  is_showing_no_path_ui: bool,
  toggle_selected: Option<char>,
  debounce_timer: Timer,
}

pub struct OnClickCover((usize, usize));

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<MapState>()
      .add_plugins(
        DefaultPickingPlugins
          .build()
          .disable::<DebugPickingPlugin>(),
      )
      .add_plugin(EguiPlugin)
      .add_startup_system(plugin_init)
      .add_system(camera_system)
      .add_system(map_system)
      .add_system(ui_system)
      .add_system(render_ui_system)
      .add_system(map_events)
      .add_event::<MouseMotion>()
      .add_event::<OnClickCover>()
      .add_event::<Event>();
  }
}

fn plugin_init(mut map_state: ResMut<MapState>, mut commands: Commands) {
  map_state.debounce_timer = Timer::from_seconds(0.3, Repeating);
  commands.spawn((factory_camera(), RaycastPickCamera::default()));
}

fn map_events(
  mut game_state: ResMut<GameState>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut local: ResMut<MapState>,
  mut event_reader: EventReader<Event>,
  mut commands: Commands,
) {
  for event in event_reader.iter() {
    dbg!(event);
    match event {
      Event::MapStart => {
        local.clear();
        local.current_state = Some(ModeState::Active);

        if game_state.mode == GameMode::Map {
          if let Some(map) = game_state.map.clone() {
            commands.spawn(factory_light());

            for (index, ch) in map.flat.iter().enumerate() {
              let x = index % map.size.1;
              let z = index / map.size.1;
              let height = ch_to_height(*ch);

              let (entity, mats) = factory_heightmap_entity(
                //  &mut entities_query,
                &mut commands,
                &mut meshes,
                &mut materials,
                &map,
                x,
                height,
                z,
              );

              local.spawned_entities.push(entity);
              local.original_materials.insert(entity, mats);
            }

            // show_map(&mut local, &mut commands);
            if let Some(gs_map) = game_state.map.clone() {
              if gs_map.start.is_some() {
                show_start_highlight(&mut local, &game_state, &mut materials);
              }

              if gs_map.end.is_some() {
                show_end_highlight(&mut local, &game_state, &mut materials);
              }
            }
          }
        } else if local.is_showing_map {
          hide_map(&mut local, &mut commands);
        }
      }
      Event::MapEnd => {
        local.clear();
        local.current_state = Some(ModeState::Inactive);
        if game_state.mode == GameMode::Map {
          game_state.mode = GameMode::default();
        }
      }
      Event::HideMap => {
        hide_map(&mut local, &mut commands);
      }
      Event::ShowStartHighlight => {
        show_start_highlight(&mut local, &game_state, &mut materials);
      }
      Event::ShowEndHighlight => {
        show_end_highlight(&mut local, &game_state, &mut materials);
      }
      Event::HideStartHighlight => {
        hide_start_highlight(&mut local, &game_state, &mut materials);
      }
      Event::HideEndHighlight => {
        hide_end_highlight(&mut local, &game_state, &mut materials);
      }
      Event::FindPath => {
        if let Some(gs_map) = game_state.map.as_ref() {
          if !(gs_map.start.is_none() || gs_map.end.is_none()) {
            let solution_opt = find_path_part1(
              gs_map.graph.clone(),
              gs_map.start.unwrap(),
              gs_map.end.unwrap(),
            );
            if let Some((distance, path)) = solution_opt {
              let mut updated_map = gs_map.clone();
              updated_map.solution = Some((distance, path));
              game_state.map = Some(updated_map);

              if distance > 0 {
                local.is_showing_no_path_ui = false;
                if let Some(map) = game_state.map.clone() {
                  render_path(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut local,
                    &map,
                  );
                }
              } else {
                clear_path(&mut commands, &mut local);
                local.is_showing_no_path_ui = true;
              }
            }
          }
        }
      }
    }
  }
}

fn map_system(
  local: ResMut<MapState>,
  game_state: ResMut<GameState>,
  mut events: EventWriter<Event>,
) {
  if game_state.mode == GameMode::Map {
    if Some(ModeState::Active) != local.current_state {
      events.send(Event::MapStart);
    }
  } else if Some(ModeState::Active) == local.current_state {
    events.send(Event::HideMap);
    events.send(Event::MapEnd);
  }
}

fn ui_system(
  mut local: ResMut<MapState>,
  mut game_state: ResMut<GameState>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut ui_events: EventReader<OnClickCover>,
  time: Res<Time>,
  mut events: EventWriter<Event>,
) {
  if !(game_state.mode == GameMode::Map
    && Some(ModeState::Active) == local.current_state)
  {
    return;
  }
  local.debounce_timer.tick(time.delta());

  let mut gs_map = game_state
    .map
    .clone()
    .expect("[MapPlugin::ui_system] map missing while active in MapPlugin");

  if local.debounce_timer.finished() {
    local.debounce_timer.reset();

    if let Some(&OnClickCover((z, x))) = ui_events.iter().next() {
      if let Some(toggle_selected) = local.toggle_selected {
        let index = z * gs_map.size.1 + x;
        if toggle_selected == 'S' {
          if gs_map.start == Some(index) {
            gs_map.start = None;
            events.send(Event::HideStartHighlight);
          } else {
            gs_map.start = Some(index);
            hide_start_highlight(&mut local, &game_state, &mut materials);
            events.send(Event::ShowStartHighlight);
          }
        } else if toggle_selected == 'E' {
          if gs_map.end == Some(index) {
            gs_map.end = None;
            events.send(Event::HideEndHighlight);
          } else {
            gs_map.end = Some(index);
            hide_end_highlight(&mut local, &game_state, &mut materials);
            events.send(Event::ShowEndHighlight);
          }
        }
        game_state.map = Some(gs_map);
      }
    }
  }
}

fn render_ui_system(
  mut contexts: EguiContexts,
  game_state: ResMut<GameState>,
  mut ui_state: ResMut<MapState>,
  mut events: EventWriter<Event>,
) {
  if !(game_state.mode == GameMode::Map
    && Some(ModeState::Active) == ui_state.current_state)
  {
    return;
  }

  egui::SidePanel::left("ui_panel").show(contexts.ctx_mut(), |ui| {
    if ui
      .button(CTA_FIND_PATH)
      .on_hover_text(FIND_PATH_TOOLTIP)
      .clicked()
    {
      events.send(Event::FindPath);
    };

    if ui
      .button(CTA_TOGGLE_START)
      .on_hover_text(TOGGLE_START_TOOLTIP)
      .clicked()
    {
      ui_state.toggle_selected = Some('S');
    };
    if ui
      .button(CTA_TOGGLE_END)
      .on_hover_text(TOGGLE_END_TOOLTIP)
      .clicked()
    {
      ui_state.toggle_selected = Some('E');
    };
  });

  if ui_state.is_showing_no_path_ui {
    egui::Window::new(NO_PATH_TITLE)
      .default_pos(egui::pos2(0.5, 0.5) - egui::vec2(0.5, 0.5))
      .resizable(false)
      .collapsible(false)
      .open(&mut ui_state.is_showing_no_path_ui)
      .show(contexts.ctx_mut(), |ui| {
        ui.label(NO_PATH_LABEL);
      });
  }
}

fn ch_to_height(ch: char) -> f32 {
  let min_height = 0.0;
  let max_height = 1.0;
  let ascii_min = b'a' as u32;
  let ascii_max = (b'Z' + b'a' - b'A') as u32;

  let ch_value = match ch {
    'S' => b'a' as u32,
    'E' => b'z' as u32,
    _ => (ch as u8 + b'a' - b'A') as u32,
  };

  let height_ratio =
    (ch_value - ascii_min) as f32 / (ascii_max - ascii_min) as f32;

  min_height + height_ratio * (max_height - min_height)
}

fn hide_map(local: &mut MapState, commands: &mut Commands) {
  for &entity in &local.spawned_entities {
    commands.entity(entity).remove::<PbrBundle>();
  }
  local.is_showing_map = false;
}
