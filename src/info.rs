#[derive(Debug, Clone, Default)]
pub struct Info {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Info {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
        }
    }
}
