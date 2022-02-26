pub mod marble;

pub mod prelude {
    /// The physics scale
    pub const PHYSICS_SCALE: f32 = 100.0;
    pub const PHYSICS_SCALE_INV: f32 = 1.0 / PHYSICS_SCALE;

    /// Convert constant from pixel scale (1.0 == one pixel) to the chosen Rapier physics scale. Now we
    /// can use the same scale for our graphics and our physics (at least for the literals)
    pub fn scale(x: f32) -> f32 {
        x * PHYSICS_SCALE_INV
    }
}
