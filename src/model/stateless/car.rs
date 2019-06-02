pub struct Car {
    pub max_velocity: f64,
    pub max_acceleration: f64,
    pub max_break_acceleration: f64,
    pub driving_model: DrivingModel,
}

pub enum DrivingModel {
    Normal {
        /// Min cushion to front car
        cushion: f64,
    },
}
