pub mod marble;

pub mod prelude {
    use bevy::math::Vec2;

    /// The physics scale
    pub const PHYSICS_SCALE: f32 = 100.0;
    pub const PHYSICS_SCALE_INV: f32 = 1.0 / PHYSICS_SCALE;

    /// Convert `f32` from pixel scale (1.0 == one pixel) to the chosen Rapier physics scale.
    pub fn scale(x: f32) -> f32 {
        x * PHYSICS_SCALE_INV
    }

    /// Convert `Vec2` from pixel scale (1.0 == one pixel) to the chosen Rapier physics scale.
    pub fn scale_vec2(v: Vec2) -> Vec2 {
        Vec2::new(v.x * PHYSICS_SCALE_INV, v.y * PHYSICS_SCALE_INV)
    }

    /// Convert `f32` from the chosen Rapier physics scale to pixel scale (1.0 == one pixel).
    pub fn unscale(x: f32) -> f32 {
        x * PHYSICS_SCALE
    }

    /// Convert `Vec2` from the chosen Rapier physics scale to pixel scale (1.0 == one pixel).
    pub fn unscale_vec2(v: Vec2) -> Vec2 {
        Vec2::new(v.x * PHYSICS_SCALE, v.y * PHYSICS_SCALE)
    }
}
