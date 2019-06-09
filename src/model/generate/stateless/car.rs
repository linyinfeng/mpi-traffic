use rand;
use rand::Rng;

use crate::model::generate::stateless::StatelessModelGenerationSettings;
use crate::model::stateless::car::DrivingModel;
use crate::model::stateless::Car;

pub fn generate_cars(settings: &StatelessModelGenerationSettings) -> Vec<Car> {
    (0..settings.initial_car_number)
        .map(|_| generate_car(settings))
        .collect()
}

fn generate_car(settings: &StatelessModelGenerationSettings) -> Car {
    let mut rng = rand::thread_rng();
    Car {
        max_velocity: rng.gen_range(settings.min_cushion, settings.max_max_velocity),
        max_acceleration: rng
            .gen_range(settings.min_max_acceleration, settings.max_max_acceleration),
        max_break_acceleration: rng.gen_range(
            settings.min_max_break_acceleration,
            settings.max_max_break_acceleration,
        ),
        lane_change_time: rng
            .gen_range(settings.min_lane_change_time, settings.max_lane_change_time),
        driving_model: DrivingModel::Normal {
            cushion: rng.gen_range(settings.min_cushion, settings.max_cushion),
        },
    }
}
