use bevy::{
  prelude::*,
  render::mesh::{Indices, Mesh, PrimitiveTopology, VertexAttributeValues},
};
use bevy_mod_picking::prelude::*;

use super::{constants::*, MapState, OnClickCover};
use crate::{bevy_common::Map, GameState};


pub fn factory_heightmap_entity(
  commands: &mut Commands,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<StandardMaterial>>,
  map: &Map,
  x: usize,
  height: f32,
  z: usize,
) -> (
  Entity,
  Transform,
  (Handle<StandardMaterial>, Handle<StandardMaterial>),
) {
  let transform = factory_entity_transform(map.size, (x, height, z));
  let custom_mesh = factory_cuboid_mesh(height);

  let mesh = meshes.add(custom_mesh.clone());

  let green = materials.add(StandardMaterial {
    base_color: Color::LIME_GREEN,
    ..Default::default()
  });

  let tan = materials
    .add(StandardMaterial { base_color: Color::BISQUE, ..Default::default() });

  let entity = commands
    .spawn(PbrBundle {
      mesh,
      transform,
      material: tan.clone(),
      ..Default::default()
    })
    .with_children(|parent| {
      if let Some(top_surface_vertices) =
        extract_top_surface_vertices(&custom_mesh)
      {
        let plane_mesh = create_plane_mesh(top_surface_vertices);
        let surface_transform = Transform::from_translation(Vec3::Y * 0.001);
        parent.spawn((
          PbrBundle {
            mesh: meshes.add(plane_mesh),
            material: green.clone(),
            transform: surface_transform,
            ..Default::default()
          },
          RaycastPickTarget::default(),
          OnPointer::<Click>::run_callback(
            move |In(_): In<ListenedEvent<Click>>,
                  mut cover_events: EventWriter<OnClickCover>|
                  -> Bubble {
              cover_events.send(OnClickCover((z, x)));
              Bubble::Up
            },
          ),
        ));
      }
    })
    .id();

  (entity, transform, (green, tan))
}

fn factory_cuboid_mesh(height: f32) -> Mesh {
  let mut positions = Vec::<[f32; 3]>::new();
  let mut indices = Vec::<u32>::new();

  // Front face
  positions.push([-0.5, 0.0, -0.5]);
  positions.push([0.5, 0.0, -0.5]);
  positions.push([0.5, height, -0.5]);
  positions.push([-0.5, height, -0.5]);
  indices.push(0);
  indices.push(1);
  indices.push(2);
  indices.push(2);
  indices.push(3);
  indices.push(0);

  // Back face
  positions.push([-0.5, 0.0, 0.5]);
  positions.push([0.5, 0.0, 0.5]);
  positions.push([0.5, height, 0.5]);
  positions.push([-0.5, height, 0.5]);
  indices.push(4);
  indices.push(5);
  indices.push(6);
  indices.push(6);
  indices.push(7);
  indices.push(4);

  // Left face
  positions.push([-0.5, 0.0, 0.5]);
  positions.push([-0.5, 0.0, -0.5]);
  positions.push([-0.5, height, -0.5]);
  positions.push([-0.5, height, 0.5]);
  indices.push(8);
  indices.push(9);
  indices.push(10);
  indices.push(10);
  indices.push(11);
  indices.push(8);

  // Right face
  positions.push([0.5, 0.0, 0.5]);
  positions.push([0.5, 0.0, -0.5]);
  positions.push([0.5, height, -0.5]);
  positions.push([0.5, height, 0.5]);
  indices.push(12);
  indices.push(13);
  indices.push(14);
  indices.push(14);
  indices.push(15);
  indices.push(12);

  // Top face
  positions.push([-0.5, height, -0.5]);
  positions.push([0.5, height, -0.5]);
  positions.push([0.5, height, 0.5]);
  positions.push([-0.5, height, 0.5]);
  indices.push(16);
  indices.push(18);
  indices.push(17);
  indices.push(16);
  indices.push(19);
  indices.push(18);

  // Bottom face
  positions.push([-0.5, 0.0, -0.5]);
  positions.push([0.5, 0.0, -0.5]);
  positions.push([0.5, 0.0, 0.5]);
  positions.push([-0.5, 0.0, 0.5]);
  indices.push(19);
  indices.push(21);
  indices.push(20);
  indices.push(19);
  indices.push(22);
  indices.push(21);

  let mut mesh = bevy::render::mesh::Mesh::new(PrimitiveTopology::TriangleList);
  mesh.insert_attribute(
    Mesh::ATTRIBUTE_POSITION,
    VertexAttributeValues::from(positions),
  );
  mesh.set_indices(Some(Indices::U32(indices)));
  mesh.duplicate_vertices();
  mesh.compute_flat_normals();
  mesh
}

pub fn extract_top_surface_vertices(mesh: &Mesh) -> Option<[[Vec3; 3]; 2]> {
  let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION)?;

  let vertex_count = positions.len();
  let indices = (0..vertex_count).collect::<Vec<usize>>();

  let mut top_surface_vertices = [[Vec3::ZERO; 3]; 2];
  let mut surface_count = 0;

  let positions_slice = positions.as_float3()?;

  let top_face_indices = (0..vertex_count)
    .step_by(3) // everything is triangles
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
  let positions = vertices.iter().flatten().copied().collect::<Vec<Vec3>>();
  let indices = vec![0, 1, 2, 3, 4, 5];
  let mut plane_mesh = Mesh::new(PrimitiveTopology::TriangleList);
  plane_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
  plane_mesh.set_indices(Some(Indices::U32(indices)));
  plane_mesh.duplicate_vertices();
  plane_mesh.compute_flat_normals();

  plane_mesh
}

fn factory_entity_transform(
  map_size: (usize, usize),
  xyz: (usize, f32, usize),
) -> Transform {
  let entity_dim_x = DESIRED_VIEW_WIDTH / (map_size.1 - 1) as f32;
  let entity_dim_z = DESIRED_VIEW_HEIGHT / (map_size.0 - 1) as f32;

  let scale = Vec3::new(entity_dim_x, xyz.1, entity_dim_z);

  // Calculate the translation based on the entity position relative to the center
  let translation_x = (xyz.0 as f32 - (map_size.1 - 1) as f32 / 2.0)
    * entity_dim_x
    - entity_dim_x / DESIRED_VIEW_WIDTH;
  let translation_z = (xyz.2 as f32 - (map_size.0 - 1) as f32 / 2.0)
    * entity_dim_z
    - entity_dim_z / DESIRED_VIEW_HEIGHT;
  let translation = Vec3::new(translation_x, 0.0, translation_z);

  // dbg!(Transform { translation, scale, ..Default::default() });
  Transform { translation, scale, ..Default::default() }
}

pub fn show_start_highlight(
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

pub fn hide_start_highlight(
  local: &mut MapState,
  game_state: &GameState,
  materials: &mut ResMut<Assets<StandardMaterial>>,
) {
  if let Some(start_entity) = get_start_entity(local, game_state) {
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

pub fn show_end_highlight(
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

pub fn hide_end_highlight(
  local: &mut MapState,
  game_state: &GameState,
  materials: &mut ResMut<Assets<StandardMaterial>>,
) {
  if let Some(end_entity) = get_end_entity(local, game_state) {
    if let Some((end_material, _)) = local.original_materials.get(&end_entity) {
      if let Some(mut material) = materials.get_mut(end_material) {
        material.base_color = Color::rgb(0.0, 0.8, 0.0);
        material.emissive = Color::BLACK;
      }
    }
  }
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
