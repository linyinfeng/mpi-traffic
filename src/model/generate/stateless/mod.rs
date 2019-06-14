use crate::model::stateless::Model;

use structopt::StructOpt;

pub mod car;
pub mod city;

#[derive(Clone, Debug, StructOpt)]
pub struct StatelessModelGenerationSettings {
    #[structopt(
        name = "stateless-model-generation-board-shape-rows",
        default_value = "3",
        long = "stateless-model-generation-board-shape-rows"
    )]
    pub board_shape_rows: usize,
    #[structopt(
        name = "stateless-model-generation-board-shape-cols",
        default_value = "4",
        long = "stateless-model-generation-board-shape-cols"
    )]
    pub board_shape_cols: usize,

    #[structopt(
        name = "stateless-model-generation-min-road-length",
        default_value = "20",
        long = "stateless-model-generation-min-road-length"
    )]
    pub min_road_length: f64,
    #[structopt(
        name = "stateless-model-generation-max-road-length",
        default_value = "100",
        long = "stateless-model-generation-max-road-length"
    )]
    pub max_road_length: f64,
    #[structopt(
        name = "stateless-model-generation-lane-width",
        default_value = "3.5",
        long = "stateless-model-generation-lane-width"
    )]
    pub lane_width: f64,
    #[structopt(
        name = "stateless-model-generation-initial-car-number",
        default_value = "20",
        long = "stateless-model-generation-initial-car-number"
    )]
    pub initial_car_number: usize,
    #[structopt(
        name = "stateless-model-generation-min-max-velocity",
        default_value = "200.0",
        long = "stateless-model-generation-min-max-velocity"
    )]
    pub min_max_velocity: f64,
    #[structopt(
        name = "stateless-model-generation-max-max-velocity",
        default_value = "260.0",
        long = "stateless-model-generation-max-max-velocity"
    )]
    pub max_max_velocity: f64,
    #[structopt(
        name = "stateless-model-generation-min-max-acceleration",
        default_value = "30.0",
        long = "stateless-model-generation-min-max-acceleration"
    )]
    min_max_acceleration: f64,
    #[structopt(
        name = "stateless-model-generation-max-max-acceleration",
        default_value = "60.0",
        long = "stateless-model-generation-max-max-acceleration"
    )]
    pub max_max_acceleration: f64,
    #[structopt(
        name = "stateless-model-generation-min-max-break-acceleration",
        default_value = "30.0",
        long = "stateless-model-generation-min-max-break-acceleration"
    )]
    pub min_max_break_acceleration: f64,
    #[structopt(
        name = "stateless-model-generation-max-max-break-acceleration",
        default_value = "50.0",
        long = "stateless-model-generation-max-max-break-acceleration"
    )]
    pub max_max_break_acceleration: f64,
    #[structopt(
        name = "stateless-model-generation-min-lane-change-time",
        default_value = "15.0",
        long = "stateless-model-generation-min-lane-change-time"
    )]
    pub min_lane_change_time: f64,
    #[structopt(
        name = "stateless-model-generation-max-lane-change-time",
        default_value = "20.0",
        long = "stateless-model-generation-max-lane-change-time"
    )]
    pub max_lane_change_time: f64,
    #[structopt(
        name = "stateless-model-generation-min-cushion",
        default_value = "8.0",
        long = "stateless-model-generation-min-cushion"
    )]
    pub min_cushion: f64,
    #[structopt(
        name = "stateless-model-generation-min-cushion-velocity-factor",
        default_value = "2.0",
        long = "stateless-model-generation-min-cushion-velocity-factor"
    )]
    pub min_cushion_velocity_factor: f64,
    #[structopt(
        name = "stateless-model-generation-max-cushion-velocity-factor",
        default_value = "3.0",
        long = "stateless-model-generation-max-cushion-velocity-factor"
    )]
    pub max_cushion_velocity_factor: f64,
    #[structopt(
        name = "stateless-model-generation-prediction-time",
        default_value = "5.0",
        long = "stateless-model-generation-prediction-time"
    )]
    pub prediction_time: f64,
    #[structopt(
        name = "stateless-model-generation-time-out",
        default_value = "5.0",
        long = "stateless-model-generation-time-out"
    )]
    pub time_out: f64,
    #[structopt(
        name = "stateless-model-generation-intersection-max-speed",
        default_value = "20.0",
        long = "stateless-model-generation-intersection-max-speed"
    )]
    pub intersection_max_speed: f64,
    #[structopt(
        name = "stateless-model-generation-lane-max-speed",
        default_value = "60.0",
        long = "stateless-model-generation-lane-max-speed"
    )]
    pub lane_max_speed: f64,
    #[structopt(
        name = "stateless-model-generation-straight-long-way-proportion",
        default_value = "0.5",
        long = "stateless-model-generation-straight-long-way-proportion"
    )]
    pub straight_long_way_proportion: f64,
    #[structopt(
        name = "stateless-model-generation-one-way-proportion",
        default_value = "0.1",
        long = "stateless-model-generation-one-way-proportion"
    )]
    pub one_way_proportion: f64,
    #[structopt(
        name = "stateless-model-generation-empty-proportion",
        default_value = "0.05",
        long = "stateless-model-generation-empty-proportion"
    )]
    pub empty_proportion: f64,
    #[structopt(
        name = "stateless-model-generation-one-way-lane-num",
        default_value = "1",
        long = "stateless-model-generation-one-way-lane-num"
    )]
    pub one_way_lane_num: usize,
    #[structopt(
        name = "stateless-model-generation-default-lane-num",
        default_value = "1",
        long = "stateless-model-generation-default-lane-num"
    )]
    pub default_lane_num: usize,
    #[structopt(
        name = "stateless-model-generation-straight-long-way-lane-num",
        default_value = "2",
        long = "stateless-model-generation-straight-long-way-lane-num"
    )]
    pub straight_long_way_lane_num: usize,

    #[structopt(
        name = "stateless-model-generation-car-out-min-distance",
        default_value = "8",
        long = "stateless-model-generation-car-out-min-distance"
    )]
    pub car_out_min_distance: f64,
}

pub fn generate_stateless_model(settings: StatelessModelGenerationSettings) -> Model {
    Model {
        city: city::generate_city(&settings),
        cars: car::generate_cars(&settings),
    }
}
