use num_quaternion::{Quaternion, UnitQuaternion, Q32};
use std::f32::consts::PI;
/// Convert feet to meters - helps the assets fit better into Godot's scale standards.
pub const WORLD_SCALE: f32 = 0.30480006096;

pub type Vector3 = (f32, f32, f32);
pub type RGBA = (f32, f32, f32, f32);
pub type QuatVec = (f32, f32, f32, f32);
/// Convert a int16 position value expressed in EQ coordinates into Godot coordinates
pub fn wld_i16_pos_to_py(p: &(i16, i16, i16), scale: f32) -> Vector3 {
    (
        p.0 as f32 * scale * -1. * WORLD_SCALE,
        p.2 as f32 * scale * WORLD_SCALE,
        p.1 as f32 * scale * WORLD_SCALE,
    )
}

/// Convert a float32 position value expressed in EQ coordinates into Godot coordinates
pub fn wld_f32_pos_to_gd(tup: &Vector3) -> Vector3 {
    (
        tup.0 * -1. * WORLD_SCALE,
        tup.2 * WORLD_SCALE,
        tup.1 * WORLD_SCALE,
    )
}

/// Convert an RGBA color value from u32 to Color
pub fn u32_to_color(num: &u32) -> RGBA {
    let red = (((num >> 24) & 0xff) as f32) / 255.0; // red
    let green = (((num >> 16) & 0xff) as f32) / 255.0; // green
    let blue = (((num >> 8) & 0xff) as f32) / 255.0; // blue
    let alpha = ((num & 0xff) as f32) / 255.0; // alpha

    (red, green, blue, alpha)
}

/// Converts a rotation expressed in Euler degrees, in X / 512, to a Godot Quaternion.
/// This is the format used for ActorInstance rotations.
pub fn wld_degrees_rot_to_quat(x: f32, y: f32, z: f32) -> Quaternion<f32> {
    wld_radians_rot_to_quat(
        x / 512. * 360.0 * PI / 180.,
        y / 512. * 360.0 * PI / 180.,
        z / 512. * 360.0 * PI / 180.,
    )
}

/// Converts a rotation expressed in Euler radians to a Godot Quaternion
pub fn wld_radians_rot_to_quat(x: f32, y: f32, z: f32) -> Quaternion<f32> {
    // The quaternion must be created with the native EQ XYZ first, due to rotation order.
    let unit_q = UnitQuaternion::from_euler_angles(x, y, z);
    let q = unit_q.as_quaternion();
    // Then we flip axes
    // FIXME: This can probably be done in a smarter way.
    Q32::new(q.w, -q.x, q.y, -q.z)
}

pub fn quat_to_quat_vec(q: Quaternion<f32>) -> QuatVec {
    (q.x, q.y, q.z, q.w)
}
