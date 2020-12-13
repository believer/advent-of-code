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

/// Extended Euclidean algorithm
/// Find the the greatest common denominator of two integers a,b
/// https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
#[allow(clippy::many_single_char_names)]
pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

/// Finds the modular multiplicative inverse
/// https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
pub fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);

    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

/// Chinese Remainder Theorem
/// https://brilliant.org/wiki/chinese-remainder-theorem/
pub fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * p * mod_inv(p, modulus)?;
    }

    Some(sum % prod)
}
