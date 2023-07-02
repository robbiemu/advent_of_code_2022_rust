use dioxus::prelude::*;

use crate::common::prelude::*;


#[inline_props]
pub fn sensor_and_container(
  cx: Scope,
  record: Record,
  bounds: Bounds,
) -> Element {
  let sensor = record.0.get_coord();
  let beacon = record.1.get_coord();

  let dx = (beacon.0 - sensor.0).abs();
  let dy = (beacon.1 - sensor.1).abs();
  let radius = dx + dy;
  let diagonal = radius as f32 * 2.0;

  let (bounds_width, bounds_height) = bounds.get_dims();
  let bounds_min = bounds_width.min(bounds_height);

  let side = diagonal / (2.0_f32.sqrt()) / bounds_min as f32 * 100.0;

  let container_left =
    (sensor.0 - bounds.0 .0) as f32 / bounds_width as f32 * 100.0 - side / 2.0;
  let container_top =
    (sensor.1 - bounds.0 .1) as f32 / bounds_height as f32 * 100.0 - side / 2.0;

  let container_style = format!(
    "border: solid 1pt black; opacity: 0.4; width: {}%; height: {}%; \
     transform: rotate(45deg); position: absolute; left: {}%; top: {}%;",
    side, side, container_left, container_top
  );
  let sensor_style =
    "position: absolute; top: calc(50% - 3pt); left: calc(50% - 0.5ex);";

  cx.render(rsx!(
      div {
          class: "sensor-and-container",
          style: "{container_style}",
          "data-dim": "sensor:{sensor.0},{sensor.1} beacon:{beacon.0},{beacon.1}",

          div {
            class: "wrapper",
            style: "width: 100%; height: 100%; position: relative",

            div {
                class: "sensor",
                style: "{sensor_style}",
            },
          }
      }
  ))
}
