use dioxus::prelude::*;

use crate::common::prelude::*;


#[inline_props]
pub fn beacon_entity(cx: Scope, beacon: Entity, bounds: Bounds) -> Element {
  let beacon = beacon.get_coord();

  let (bounds_width, bounds_height) = bounds.get_dims();
  let beacon_left =
    ((beacon.0 - bounds.0 .0) as f32 / bounds_width as f32 * 100.0).round();
  let beacon_top =
    ((beacon.1 - bounds.0 .1) as f32 / bounds_height as f32 * 100.0).round();

  cx.render(rsx!(div {
    class: "beacon",

    style: "position: absolute; left: {beacon_left}%; top: {beacon_top}%;"
  },))
}
