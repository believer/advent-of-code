use std::f32::consts::PI;

/// The sum of the absolute values of two points
pub fn manhattan_distance(x: i32, y: i32) -> u32 {
    (x.abs() + y.abs()) as u32
}

/// 2D rotation of a point
pub fn rotation_2d(x: i32, y: i32, angle: i32) -> (i32, i32) {
    let radians = angle as f32 * PI / 180_f32;

    let nx = (x as f32 * radians.cos() - y as f32 * radians.sin()).round();
    let ny = (y as f32 * radians.cos() + x as f32 * radians.sin()).round();

    (nx as i32, ny as i32)
}

/// Convert an angle from degrees to radians
pub fn degrees_to_radians(angle: i32) -> f32 {
    angle as f32 * PI / 180_f32
}
