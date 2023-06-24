use bevy::{
  input::mouse::{MouseMotion, MouseWheel},
  prelude::*,
  render::camera::ScalingMode,
  window::PrimaryWindow,
};

use super::MapState;
use crate::{bevy_common::ModeState, GameMode, GameState};


pub fn factory_camera() -> Camera3dBundle {
  Camera3dBundle {
    projection: OrthographicProjection {
      scaling_mode: ScalingMode::FixedVertical(8.0),
      ..Default::default()
    }
    .into(),
    transform: Transform::from_xyz(5.0, 6.0, 5.0)
      .looking_at(Vec3::from([-2.0, 0.0, -2.0]), Vec3::Y),
    ..Default::default()
  }
}

pub fn factory_light() -> PointLightBundle {
  PointLightBundle {
    transform: Transform::from_xyz(3.0, 8.0, 5.0),
    ..default()
  }
}

pub fn camera_system(
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
        let rotation_speed = 0.3;
        for event in mouse_motion_events.iter() {
          let delta = event.delta / window_size;
          total_rotation *= Quat::from_rotation_y(-delta.y * rotation_speed);
        }
      }

      // Process mouse wheel events
      let zoom_factor = 0.1;
      for event in mouse_wheel_events.iter() {
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
