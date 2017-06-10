#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VeilStepInfo {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub min: f64,
    pub max: f64,
}

impl Default for VeilStepInfo {
    fn default() -> Self {
        VeilStepInfo {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            min: 0.0,
            max: 0.0,
        }
    }
}
