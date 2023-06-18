use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use std::collections::HashMap;

use super::{GameMode, GameState};
use crate::bevy_common::{Clear, Map, ModeState};
use crate::common::find_path_part1;


const CTA_FIND_PATH: &str = "Go!";
const FIND_PATH_TOOLTIP: &str = "find the shortest path!";
const CTA_TOGGLE_START: &str = "start";
const TOGGLE_START_TOOLTIP: &str =
  "Click on the green surface to place the start point.";
const CTA_TOGGLE_END: &str = "end";
const TOGGLE_END_TOOLTIP: &str =
  "Click on the green surface to place the end point.";
const NO_PATH_LABEL: &str = "No path found!";
const NO_PATH_TITLE: &str = "No Path";

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
struct MapState {
  spawned_entities: Vec<Entity>,
  original_materials:
    HashMap<Entity, (Handle<StandardMaterial>, Handle<StandardMaterial>)>,
  path_entities: Vec<Entity>,
  current_state: Option<ModeState>,
  is_showing_map: bool,
  is_rendering_path: bool,
  is_showing_no_path_ui: bool,
  toggle_selected: Option<char>,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<MapState>()
      .add_plugin(EguiPlugin)
      .add_system(map_system)
      .add_system(ui_system)
      .add_system(render_ui_system)
      .add_system(map_events)
      .add_event::<Event>();
  }
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
            // let rows = map.size.0;
            // let columns = map.size.1;
            // let rotation_angle = std::f32::consts::PI / 2.0
            //   - (rows as f32 / columns as f32).atan();

            // Define the desired camera distance from the target position
            let camera_distance = map.size.1.max(map.size.0) as f32 * 1.5;
            let light_distance = map.size.1.max(map.size.0) as f32 * 3.0;

            // Calculate the camera position based on the target position and distance
            // Define the target position at the center of the map
            let target_position =
              Vec3::new(map.size.1 as f32 / 2.0, map.size.0 as f32 / 2.0, 0.0);
            let camera_position = target_position
              + Vec3::new(-4.0, -4.0, 3.0).normalize() * camera_distance;
            let light_position = target_position
              + Vec3::new(-4.0, -4.0, 3.0).normalize() * light_distance;

            // Create the transform for the camera
            let rotation_angle = std::f32::consts::PI / 4.0;
            let camera_transform = Transform::from_xyz(
              camera_position.x,
              camera_position.y,
              camera_position.z,
            )
            .looking_at(target_position, Vec3::Z)
            .mul_transform(Transform::from_rotation(Quat::from_rotation_y(
              rotation_angle,
            )));
            let light_transform = Transform::from_xyz(
              light_position.x,
              light_position.y,
              light_position.z,
            )
            .looking_at(target_position, Vec3::Z)
            .mul_transform(Transform::from_rotation(Quat::from_rotation_y(
              rotation_angle,
            )));

            // light
            commands.spawn(PointLightBundle {
              point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
              },
              transform: light_transform,
              ..default()
            });

            commands.spawn(Camera3dBundle {
              projection: OrthographicProjection {
                scale: 3.0,
                scaling_mode: ScalingMode::FixedVertical(5.0),
                ..Default::default()
              }
              .into(),
              transform: camera_transform,
              ..Default::default()
            });

            let green_material_handle = materials.add(StandardMaterial {
              base_color: Color::LIME_GREEN,
              ..Default::default()
            });

            let tan_material_handle = materials.add(StandardMaterial {
              base_color: Color::BISQUE,
              ..Default::default()
            });

            commands.spawn(PbrBundle {
              mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
              material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
              transform: Transform::from_xyz(1.5, 0.5, 1.5),
              ..default()
            });
            commands.spawn(PbrBundle {
              mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
              material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
              transform: Transform::from_xyz(1.5, 0.5, -1.5),
              ..default()
            });
            commands.spawn(PbrBundle {
              mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
              material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
              transform: Transform::from_xyz(-1.5, 0.5, 1.5),
              ..default()
            });
            commands.spawn(PbrBundle {
              mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
              material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
              transform: Transform::from_xyz(-1.5, 0.5, -1.5),
              ..default()
            });

            // Define the desired aspect ratio (e.g., 16:9)
            let aspect_ratio = 16.0 / 9.0;

            // Define the desired width of the view
            let desired_view_width = 16.0;

            // Calculate the height based on the aspect ratio
            let desired_view_height = desired_view_width / aspect_ratio;

            for (index, ch) in map.flat.iter().enumerate() {
              let x = index % map.size.1;
              let y = index / map.size.1;

              let height = ch_to_height(*ch);

              // Create the vertices of the custom 3D rectangle
              let vertices: [Vec3; 4] = [
                Vec3::new(-0.5, -0.5, 0.0),
                Vec3::new(0.5, -0.5, 0.0),
                Vec3::new(0.5, 0.5, height),
                Vec3::new(-0.5, 0.5, height),
              ];

              // Create the indices of the custom 3D rectangle
              let indices: [u32; 6] = [0, 1, 2, 0, 2, 3];

              // Create a new mesh with the custom vertices and indices
              let mut custom_mesh = Mesh::new(PrimitiveTopology::TriangleList);
              custom_mesh
                .insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.to_vec());
              custom_mesh.set_indices(Some(Indices::U32(indices.to_vec())));

              // Add the custom mesh to the mesh assets
              let custom_mesh_handle = meshes.add(custom_mesh);

              // Create the transform for the custom mesh
              let map_width = map.size.1 as f32;
              let map_height = map.size.0 as f32;
              let scale_x = desired_view_width / map_width;
              let scale_y = desired_view_height / map_height;

              // Adjust the scale of the objects relative to the map size
              let scale = Vec3::new(scale_x, scale_y, 1.0);
              let translation = Vec3::new(x as f32, y as f32, height);
              let transform =
                Transform { translation, scale, ..Default::default() };

              // Create the PBR bundle for the custom mesh with the material handles
              let pbr_bundle = PbrBundle {
                mesh: custom_mesh_handle,
                material: green_material_handle.clone(),
                transform,
                ..Default::default()
              };

              // Spawn the entity with the PBR bundle
              let entity = commands
                .spawn(pbr_bundle)
                .with_children(|parent| {
                  // Attach the tan material to the custom mesh
                  parent.spawn(PbrBundle {
                    material: tan_material_handle.clone(),
                    ..Default::default()
                  });
                })
                .id();

              local.spawned_entities.push(entity);
              local.original_materials.insert(
                entity,
                (green_material_handle.clone(), tan_material_handle.clone()),
              );
            }

            show_map(&mut local, &mut commands);
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
        if let Some(start_entity) = get_start_entity(&local, &game_state) {
          if let Some((start_material, _)) =
            local.original_materials.get(&start_entity)
          {
            if let Some(mut material) = materials.get_mut(start_material) {
              material.base_color = Color::rgb(0.0, 0.8, 0.0);
              material.emissive = Color::BLACK;
            }
          }
        }
      }
      Event::HideEndHighlight => {
        if let Some(end_entity) = get_end_entity(&local, &game_state) {
          if let Some((end_material, _)) =
            local.original_materials.get(&end_entity)
          {
            if let Some(mut material) = materials.get_mut(end_material) {
              material.base_color = Color::rgb(0.0, 0.8, 0.0);
              material.emissive = Color::BLACK;
            }
          }
        }
      }
      Event::FindPath => {
        if let Some(gs_map) = game_state.map.as_ref() {
          if !(gs_map.start.is_none() || gs_map.end.is_none()) {
            let solution_opt = find_path_part1(
              gs_map.graph.clone(), // Pass a reference to the graph
              gs_map.start.unwrap(),
              gs_map.end.unwrap(),
            );
            if let Some((distance, path)) = solution_opt {
              let mut updated_map = gs_map.clone(); // Clone the Map struct
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

#[allow(clippy::too_many_arguments)]
fn ui_system(
  local: ResMut<MapState>,
  mut game_state: ResMut<GameState>,
  window_query: Query<&Window, With<PrimaryWindow>>,
  camera_query: Query<&OrthographicProjection, With<Camera>>,
  transforms: Query<&Transform>,
  mouse_input: Res<Input<MouseButton>>,
  mut cursor_events: EventReader<CursorMoved>,
  mut events: EventWriter<Event>,
) {
  if !(game_state.mode == GameMode::Map
    && Some(ModeState::Active) == local.current_state)
  {
    return;
  }

  let Some(mut gs_map) = game_state.map.clone() else {
    return;
  };

  for event in cursor_events.iter() {
    if mouse_input.just_pressed(MouseButton::Left) {
      let mouse_position = event.position;

      if is_mouse_within_green_surface(
        mouse_position,
        local.as_ref(),
        &transforms,
      ) {
        if let Some(toggle_selected) = local.toggle_selected {
          let window = window_query.get_single().unwrap();
          let projection = camera_query.get_single().unwrap();
          let (x, y) = convert_mouse_position_to_map_coordinates(
            mouse_position,
            (window.width(), window.height()).into(),
            projection.scale,
          );
          if toggle_selected == 'S' {
            let index = y as usize * gs_map.size.1 + x as usize;
            if gs_map.start == Some(index) {
              gs_map.start = None;
              events.send(Event::HideStartHighlight);
            } else {
              gs_map.start = Some(index);
              events.send(Event::HideStartHighlight);
              events.send(Event::ShowStartHighlight);
            }
          } else if toggle_selected == 'E' {
            let index = y as usize * gs_map.size.1 + x as usize;
            if gs_map.end == Some(index) {
              gs_map.end = None;
              events.send(Event::HideEndHighlight);
            } else {
              gs_map.end = Some(index);
              events.send(Event::HideStartHighlight);
              events.send(Event::ShowEndHighlight);
            }
          }
          game_state.map = Some(gs_map.clone());
        }
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

fn is_mouse_within_green_surface(
  mouse_position: Vec2,
  local: &MapState,
  transforms: &Query<&Transform>,
) -> bool {
  for entity in &local.spawned_entities {
    if let Ok(transform) = transforms.get(*entity) {
      let bounds = calculate_entity_bounds(transform);
      if mouse_position.x >= bounds.min.x
        && mouse_position.x <= bounds.max.x
        && mouse_position.y >= bounds.min.y
        && mouse_position.y <= bounds.max.y
      {
        return true; // Mouse is within the green surface
      }
    }
  }

  false // Mouse is not within any of the green surfaces
}

fn calculate_entity_bounds(transform: &Transform) -> Rect {
  // Calculate the bounds of the entity based on its transform
  let scale = transform.scale;
  let size = Vec2::new(scale.x, scale.y);
  let position = transform.translation;
  let half_size = size * 0.5;

  let left = position.x - half_size.x;
  let right = position.x + half_size.x;
  let bottom = position.y - half_size.y;
  let top = position.y + half_size.y;

  Rect::new(left, right, bottom, top)
}

fn convert_mouse_position_to_map_coordinates(
  mouse_position: Vec2,
  window_size: Vec2,
  projection_scale: f32,
) -> (f32, f32) {
  // Calculate the normalized device coordinates (-1.0 to 1.0)
  let normalized_coords =
    (mouse_position - window_size / 2.0) / (window_size / 2.0);
  let adjusted_coords = normalized_coords / projection_scale;

  (adjusted_coords.x, -adjusted_coords.y)
}

fn get_start_entity(
  local: &MapState,
  game_state: &GameState,
) -> Option<Entity> {
  if let Some(gs_map) = game_state.map.as_ref() {
    if let Some(start_index) = gs_map.start {
      return Some(local.spawned_entities[start_index]);
    }
  }

  None
}

fn get_end_entity(local: &MapState, game_state: &GameState) -> Option<Entity> {
  if let Some(gs_map) = game_state.map.as_ref() {
    if let Some(end_index) = gs_map.end {
      return Some(local.spawned_entities[end_index]);
    }
  }

  None
}

fn clear_path(commands: &mut Commands, local: &mut ResMut<MapState>) {
  for entity in local.path_entities.iter() {
    commands.entity(*entity).despawn_recursive();
  }
  local.path_entities.clear();
  local.is_rendering_path = false;
}

fn render_path(
  commands: &mut Commands,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<StandardMaterial>>,
  local: &mut ResMut<MapState>,
  map: &Map,
) {
  clear_path(commands, local);

  if let Some((_, path)) = &map.solution {
    let grey_material_handle = materials.add(StandardMaterial {
      base_color: Color::SILVER,
      ..Default::default()
    });

    let half_width = 0.25; // Adjust this value as needed

    // Render the path
    for &node_index in path.iter() {
      let index = map.graph[node_index];
      let x = index % map.size.1;
      let y = index / map.size.1;

      let height = ch_to_height(map.flat[index]);

      // Create the vertices of the custom 3D circle
      let vertices: Vec<Vec3> = (0..=360)
        .step_by(10)
        .map(|degrees| {
          let radians = degrees as f32 * std::f32::consts::PI / 180.0;
          let x = half_width * radians.cos();
          let y = half_width * radians.sin();
          Vec3::new(x, y, height)
        })
        .collect();

      // Create the indices of the custom 3D circle
      let indices: Vec<u32> = (1..vertices.len() - 1)
        .flat_map(|i| vec![0, i as u32, i as u32 + 1])
        .collect();

      // Create a new mesh with the custom vertices and indices
      let mut custom_mesh = Mesh::new(PrimitiveTopology::TriangleList);
      custom_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
      custom_mesh.set_indices(Some(Indices::U32(indices)));

      // Add the custom mesh to the mesh assets
      let custom_mesh_handle = meshes.add(custom_mesh);

      // Create the transform for the custom mesh
      let translation = Vec3::new(x as f32, y as f32, height);
      let transform = Transform {
        translation,
        scale: Vec3::new(1.0, 1.0, 1.0),
        ..Default::default()
      };

      // Create the PBR bundle for the custom mesh with the material handle
      let pbr_bundle = PbrBundle {
        mesh: custom_mesh_handle,
        material: grey_material_handle.clone(),
        transform,
        ..Default::default()
      };

      // Spawn the entity with the PBR bundle
      let entity = commands.spawn(pbr_bundle).id();
      local.path_entities.push(entity);
    }

    local.is_rendering_path = true;
  }
}

fn show_map(local: &mut MapState, commands: &mut Commands) {
  for &entity in &local.spawned_entities {
    if let Some((green_material, tan_material)) =
      local.original_materials.get(&entity)
    {
      commands.entity(entity).insert(green_material.clone());
      commands.entity(entity).with_children(|parent| {
        parent.spawn(PbrBundle {
          material: tan_material.clone(),
          ..Default::default()
        });
      });
    }
  }
  local.is_showing_map = true;
}

fn hide_map(local: &mut MapState, commands: &mut Commands) {
  for &entity in &local.spawned_entities {
    commands.entity(entity).remove::<PbrBundle>();
  }
  local.is_showing_map = false;
}

fn show_start_highlight(
  local: &mut MapState,
  game_state: &GameState,
  materials: &mut ResMut<Assets<StandardMaterial>>,
) {
  if let Some(start_entity) = get_start_entity(local, game_state) {
    if let Some((start_material, _)) =
      local.original_materials.get(&start_entity)
    {
      if let Some(mut material) = materials.get_mut(start_material) {
        // Adjust the material properties to create the highlight effect
        material.base_color = Color::GREEN;
        material.emissive = Color::YELLOW;
      }
    }
  }
}

fn show_end_highlight(
  local: &mut MapState,
  game_state: &GameState,
  materials: &mut ResMut<Assets<StandardMaterial>>,
) {
  if let Some(end_entity) = get_end_entity(local, game_state) {
    if let Some((end_material, _)) = local.original_materials.get(&end_entity) {
      if let Some(mut material) = materials.get_mut(end_material) {
        // Adjust the material properties to create the highlight effect
        material.base_color = Color::TOMATO;
        material.emissive = Color::YELLOW;
      }
    }
  }
}
