use rand::{self, Rng};

use crate::model::{
    generate::stateless::StatelessModelGenerationSettings,
    stateless::{car::DrivingModel, Car},
};

pub fn generate_cars(settings: &StatelessModelGenerationSettings) -> Vec<Car> {
    (0..settings.initial_car_number)
        .map(|_| generate_car(settings))
        .collect()
}

fn generate_car(settings: &StatelessModelGenerationSettings) -> Car {
    let mut rng = rand::thread_rng();
    Car {
        max_velocity: rng.gen_range(settings.min_max_velocity..=settings.max_max_velocity),
        max_acceleration: rng
            .gen_range(settings.min_max_acceleration..=settings.max_max_acceleration),
        max_break_acceleration: rng
            .gen_range(settings.min_max_break_acceleration..=settings.max_max_break_acceleration),
        lane_change_time: rng
            .gen_range(settings.min_lane_change_time..=settings.max_lane_change_time),
        driving_model: DrivingModel::Normal {
            min_cushion: settings.min_cushion,
            cushion_velocity_factor: rng.gen_range(
                settings.min_cushion_velocity_factor..=settings.max_cushion_velocity_factor,
            ),
            prediction_time: settings.prediction_time,
        },
    }
}
