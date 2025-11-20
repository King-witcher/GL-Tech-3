#[derive(Debug, Clone, Copy)]
pub struct ViewInfo {
    /// Precomputed value for column height calculation. Represents the column
    /// height at a distance of 1.0 from the camera.
    pub col_height_1: f32,
    /// Precomputed tangent related to FOV and image width
    pub step_0: f32,
}

impl ViewInfo {
    pub fn from_fov(hfov_deg: f32, width: f32) -> Self {
        let tan = f32::tan(0.5 * hfov_deg.to_radians());
        let step_0 = 2.0 * tan / width;
        let col_height_1 = width / (2.0 * tan);

        Self {
            col_height_1,
            step_0,
        }
    }
}
