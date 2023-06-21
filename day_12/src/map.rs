use bevy::{
  input::mouse::{MouseMotion, MouseWheel},
  prelude::*,
  render::{
    camera::ScalingMode,
    mesh::{Indices, PrimitiveTopology},
  },
  window::PrimaryWindow,
};
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

const DESIRED_VIEW_WIDTH: f32 = 5.0;
const DESIRED_VIEW_HEIGHT: f32 = 5.0;

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
      .add_system(camera_system)
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
            // light
            commands.spawn(PointLightBundle {
              transform: Transform::from_xyz(3.0, 8.0, 5.0),
              ..default()
            });

            // camera
            commands.spawn(Camera3dBundle {
              projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(5.0),
                ..Default::default()
              }
              .into(),
              transform: Transform::from_xyz(5.0, 5.0, 5.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
              ..Default::default()
            });

            // constant plane
            commands.spawn(PbrBundle {
              mesh: meshes.add(shape::Plane::from_size(5.0).into()),
              material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
              ..default()
            });

            // for (index, ch) in map.flat.iter().enumerate() {
            //   let x = index % map.size.1;
            //   let z = index / map.size.1;
            //   let height = ch_to_height(*ch);

            //   let transform =
            //     factory_entity_transform(map.size, (x, height, z));
            //   let custom_mesh = factory_cuboid_mesh(height);

            //   let mesh = meshes.add(custom_mesh.clone());

            //   // Create the PBR bundle for the custom mesh with the material handles
            //   let green = materials.add(StandardMaterial {
            //     base_color: Color::LIME_GREEN,
            //     ..Default::default()
            //   });

            //   let tan = materials.add(StandardMaterial {
            //     base_color: Color::BISQUE,
            //     ..Default::default()
            //   });

            //   let entity = commands
            //     .spawn(PbrBundle {
            //       mesh,
            //       transform,
            //       material: tan.clone(),
            //       ..Default::default()
            //     })
            //     .with_children(|parent| {
            //       if let Some(top_surface_vertices) =
            //         extract_top_surface_vertices(&custom_mesh)
            //       {
            //         let plane_mesh = create_plane_mesh(top_surface_vertices);
            //         let surface_transform =
            //           transform * Transform::from_translation(Vec3::Y * f32::EPSILON);
            //         parent.spawn(PbrBundle {
            //           mesh: meshes.add(plane_mesh),
            //           material: green.clone(),
            //           transform: surface_transform,
            //           ..Default::default()
            //         });
            //       }
            //     })
            //     .id();

            //   local.spawned_entities.push(entity);
            //   local
            //     .original_materials
            //     .insert(entity, (green.clone(), tan.clone()));
            // }

            // // show_map(&mut local, &mut commands);
            // if let Some(gs_map) = game_state.map.clone() {
            //   if gs_map.start.is_some() {
            //     show_start_highlight(&mut local, &game_state, &mut materials);
            //   }

            //   if gs_map.end.is_some() {
            //     show_end_highlight(&mut local, &game_state, &mut materials);
            //   }
            // }
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

fn camera_system(
  game_state: Res<GameState>,
  local: Res<MapState>,
  mut query: Query<&mut Transform, With<Camera>>,
  window_query: Query<&Window, With<PrimaryWindow>>,
  mouse_button_input: Res<Input<MouseButton>>,
  mut mouse_motion_events: EventReader<MouseMotion>,
  mut mouse_wheel_events: EventReader<MouseWheel>,
) {
  if !(game_state.mode == GameMode::Map
    && Some(ModeState::Active) == local.current_state)
  {
    return;
  }

  let window = window_query.get_single().unwrap();
  if window.cursor_position().is_some() {
    for mut transform in query.iter_mut() {
      let window_size = Vec2::new(window.width(), window.height());
      let mut total_rotation = Quat::IDENTITY;
      if mouse_button_input.pressed(MouseButton::Left) {
        eprintln!("left mouse dragged");
        let rotation_speed = 0.3;
        for event in mouse_motion_events.iter() {
          let delta = event.delta / window_size;
          total_rotation *= Quat::from_rotation_y(-delta.y * rotation_speed);
        }
      }

      // Process mouse wheel events
      let zoom_factor = 0.1;
      for event in mouse_wheel_events.iter() {
        eprintln!("mouse wheel rotated");
        // Zoom factor determines how much the camera zooms in or out

        // Adjust the camera's scale based on the scroll direction
        if event.y.abs() > f32::EPSILON {
          let zoom_delta = event.y * zoom_factor;
          let current_scale = transform.scale;
          let new_scale = current_scale * (1.0 + zoom_delta);
          transform.scale = new_scale;
        }
      }

      if total_rotation != Quat::IDENTITY {
        let current_rotation = transform.rotation;
        let new_rotation = current_rotation * total_rotation;
        transform.rotation = new_rotation;
      }
    }
  }
}

fn factory_entity_transform(
  map_size: (usize, usize),
  xyz: (usize, f32, usize),
) -> Transform {
  let entity_dim_x = DESIRED_VIEW_WIDTH / (map_size.1 - 1) as f32;
  let entity_dim_z = DESIRED_VIEW_HEIGHT / (map_size.0 - 1) as f32;

  let scale = Vec3::new(entity_dim_x, xyz.1, entity_dim_z);

  // Calculate the translation based on the entity position relative to the center
  let translation_x =
    (xyz.0 as f32 - (map_size.1 - 1) as f32 / 2.0) * entity_dim_x;
  let translation_z =
    (xyz.2 as f32 - (map_size.0 - 1) as f32 / 2.0) * entity_dim_z;
  let translation = Vec3::new(translation_x, xyz.1, translation_z);

  dbg!(Transform { translation, scale, ..Default::default() });
  Transform { translation, scale, ..Default::default() }
}

fn factory_cuboid_mesh(height: f32) -> Mesh {
  let vertices: [Vec3; 8] = [
    Vec3::new(-0.5, 0.0, -0.5),    // 0: Bottom Left Back
    Vec3::new(0.5, 0.0, -0.5),     // 1: Bottom Right Back
    Vec3::new(0.5, 0.0, 0.5),      // 2: Bottom Right Front
    Vec3::new(-0.5, 0.0, 0.5),     // 3: Bottom Left Front
    Vec3::new(-0.5, height, -0.5), // 4: Top Left Back
    Vec3::new(0.5, height, -0.5),  // 5: Top Right Back
    Vec3::new(0.5, height, 0.5),   // 6: Top Right Front
    Vec3::new(-0.5, height, 0.5),  // 7: Top Left Front
  ];

  let indices: [u32; 36] = [
    0, 3, 2, 0, 2, 1, // Side 1: Bottom
    1, 2, 6, 1, 6, 5, // Side 2: Right
    5, 6, 7, 5, 7, 4, // Side 3: Top
    4, 7, 3, 4, 3, 0, // Side 4: Left
    0, 1, 5, 0, 5, 4, // Side 5: Back
    3, 7, 6, 3, 6, 2, // Side 6: Front
  ];

  let mut custom_mesh = Mesh::new(PrimitiveTopology::TriangleList);
  custom_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.to_vec());
  custom_mesh.set_indices(Some(Indices::U32(indices.to_vec())));

  custom_mesh.duplicate_vertices();
  custom_mesh.compute_flat_normals();

  custom_mesh
}

fn extract_top_surface_vertices(mesh: &Mesh) -> Option<[[Vec3; 3]; 2]> {
  let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION)?;

  let vertex_count = positions.len();
  let indices = (0..vertex_count).collect::<Vec<usize>>();

  let mut top_surface_vertices = [[Vec3::ZERO; 3]; 2];
  let mut surface_count = 0;

  let positions_slice = positions.as_float3()?;

  let top_face_indices = (0..vertex_count)
    .step_by(3) // Assumes triangles
    .filter_map(|i| {
      let idx1 = indices[i];
      let idx2 = indices[i + 1];
      let idx3 = indices[i + 2];

      let p1 = positions_slice.get(idx1)?;
      let p2 = positions_slice.get(idx2)?;
      let p3 = positions_slice.get(idx3)?;

      if p1[1] > 0.0 && p2[1] > 0.0 && p3[1] > 0.0 {
        Some([
          Vec3::new(p1[0], p1[1], p1[2]),
          Vec3::new(p2[0], p2[1], p2[2]),
          Vec3::new(p3[0], p3[1], p3[2]),
        ])
      } else {
        None
      }
    });

  for face in top_face_indices {
    if surface_count >= 2 {
      break;
    }
    top_surface_vertices[surface_count] = face;
    surface_count += 1;
  }

  if surface_count < 2 {
    return None;
  }

  Some(top_surface_vertices)
}

fn create_plane_mesh(vertices: [[Vec3; 3]; 2]) -> Mesh {
  let mut positions = Vec::new();
  let mut indices = Vec::new();

  for triangle_vertices in vertices.iter() {
    let base_index = positions.len() as u32;

    for vertex in triangle_vertices.iter() {
      positions.push(vertex.clone());
      indices.push(base_index + indices.len() as u32);
    }
  }

  let mut plane_mesh = Mesh::new(PrimitiveTopology::TriangleList);
  plane_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
  plane_mesh.set_indices(Some(Indices::U32(indices)));

  plane_mesh
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
