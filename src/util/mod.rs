use pyo3::prelude::*;
use pyo3::types::PyTuple;
/// Convert feet to meters - helps the assets fit better into Godot's scale standards.
pub const WORLD_SCALE: f32 = 0.30480006096;

/// Convert a int16 position value expressed in EQ coordinates into Godot coordinates
pub fn wld_i16_pos_to_py(p: &(i16, i16, i16), scale: f32) -> (f32, f32, f32) {
    (
        p.0 as f32 * scale * -1. * WORLD_SCALE,
        p.2 as f32 * scale * WORLD_SCALE,
        p.1 as f32 * scale * WORLD_SCALE,
    )
}

/// Convert a float32 position value expressed in EQ coordinates into Godot coordinates
pub fn wld_f32_pos_to_gd(tup: &(f32, f32, f32)) -> (f32, f32, f32) {
    (
        tup.0 * -1. * WORLD_SCALE,
        tup.2 * WORLD_SCALE,
        tup.1 * WORLD_SCALE,
    )
}

/// Convert an RGBA color value from u32 to Color
pub fn u32_to_color(num: &u32) -> (f32, f32, f32, f32) {
    let red = (((num >> 24) & 0xff) as f32) / 255.0; // red
    let green = (((num >> 16) & 0xff) as f32) / 255.0; // green
    let blue = (((num >> 8) & 0xff) as f32) / 255.0; // blue
    let alpha = ((num & 0xff) as f32) / 255.0; // alpha

    (red, green, blue, alpha)
}
