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
