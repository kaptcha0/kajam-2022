#[allow(dead_code)]
pub fn lerp(a: f32, b: f32, x: f32) -> f32 {
    a + (b - a) * x
}

#[allow(dead_code)]
pub fn inv_lerp(x: f32, a: f32, b: f32) -> f32 {
    (x - a) / (b - a)
}
