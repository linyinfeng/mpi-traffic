use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Car {
    pub max_velocity: f64,
    pub max_acceleration: f64,
    pub max_break_acceleration: f64,
    pub lane_change_time: f64,
    pub driving_model: DrivingModel,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DrivingModel {
    Normal {
        /// Min cushion
        min_cushion: f64,
        /// Cushion speed factor
        cushion_velocity_factor: f64,
        /// Prediction time
        prediction_time: f64,
    },
}
