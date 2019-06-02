pub struct Car {
    max_velocity: f64,
    max_acceleration: f64,
    max_break_acceleration: f64,
    driving_model: DrivingModel,
}

pub enum DrivingModel {
    Normal {
        /// Min cushion to front car
        cushion: f64,
    },
}
