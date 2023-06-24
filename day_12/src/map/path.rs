use bevy::{
  prelude::*,
  render::mesh::{Indices, Mesh, PrimitiveTopology},
};

use super::{ch_to_height, MapState};
use crate::bevy_common::Map;


pub fn render_path(
  //  entities_query: &mut Query<Entity, With<Transform>>,
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
    let half_width = 0.25; // Adjustable
    for &node_index in path.iter() {
      let index = map.graph[node_index];
      let height = ch_to_height(map.flat[index]);

      // Create the vertices of the custom 3D circle
      let vertices: Vec<Vec3> = (0..=360)
        .step_by(10)
        .map(|degrees| {
          let radians = -degrees as f32 * std::f32::consts::PI / 180.0;
          let x = half_width * radians.cos();
          let y = height;
          let z = half_width * radians.sin();
          Vec3::new(x, y, z)
        })
        .collect();

      // Create the indices of the custom 3D circle
      let indices: Vec<u32> = (1..vertices.len() - 1)
        .flat_map(|i| vec![0, i as u32, i as u32 + 1])
        .collect();

      // Create a new mesh with the custom vertices and indices
      let mut custom_mesh = Mesh::new(PrimitiveTopology::TriangleList);
      custom_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.clone());
      custom_mesh.set_indices(Some(Indices::U32(indices)));
      custom_mesh.duplicate_vertices();
      custom_mesh.compute_flat_normals();
      let custom_mesh_handle = meshes.add(custom_mesh.clone());

      // Create the PBR bundle for the custom mesh with the material handle
      let pbr_bundle = PbrBundle {
        mesh: custom_mesh_handle,
        material: grey_material_handle.clone(),
        transform: Transform::from_translation(Vec3::Y * 0.002),
        ..Default::default()
      };

      // Create the child entity
      let parent_entity = local.spawned_entities[index];

      let mut child_entity_commands = commands.spawn(pbr_bundle);
      child_entity_commands.set_parent(parent_entity);
      let child_entity = child_entity_commands.id();
      commands
        .entity(parent_entity)
        .push_children(&[child_entity]);
      local.path_entities.push(child_entity);
    }

    local.is_rendering_path = true;
  }
}

pub fn clear_path(commands: &mut Commands, local: &mut ResMut<MapState>) {
  for entity in local.path_entities.iter() {
    commands.entity(*entity).despawn_recursive();
  }
  local.path_entities.clear();
  local.is_rendering_path = false;
}
