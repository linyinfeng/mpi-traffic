use log::trace;
use piston_window::color;
use piston_window::Event;
use piston_window::EventLoop;
use piston_window::EventSettings;
use piston_window::Loop;
use piston_window::PistonWindow;
use piston_window::WindowSettings;

use mpi_traffic::controller::Controller;
use mpi_traffic::model::generate;
use mpi_traffic::model::generate::ModelGenerationSettings;
use mpi_traffic::model::generate::stateless::{GenerationStrategy, LaneAddStrategy};
use mpi_traffic::model::generate::stateless::StatelessModelGenerationSettings;
use mpi_traffic::view::{View, ViewSettings};

fn main() {
    env_logger::init();

    let mut window: PistonWindow = WindowSettings::new("MPI Traffic", [1000, 500])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("failed to build PistonWindow: {}", e));
    let event_settings = EventSettings::new().ups(120).max_fps(60);
    window.set_event_settings(event_settings);

    let view = {
        let view_settings = ViewSettings::new();
        View::new(view_settings)
    };
    let settings = ModelGenerationSettings {
        stateless_model_settings: StatelessModelGenerationSettings {
            lane_width: 3.5,
            board_shape: (4, 4),
            road_generation_strategy: GenerationStrategy::Random,
            lane_add_strategy: LaneAddStrategy::Base,
        },
    };
    let mut model = generate::generate_model(settings);
    let stateless_model = model.stateless;
    let mut stateful_model = model.stateful;
    let controller = Controller;

    while let Some(e) = window.next() {
        trace!("event: {:?}", e);
        window.draw_2d(&e, |c, g, _| {
            use piston_window::clear;
            let clear_color = color::BLACK;
            clear(clear_color, g);
            view.draw(&stateless_model, &stateful_model, c, g);
        });
        match e {
            Event::Input(e, _) => {
                controller.input(&stateless_model, &mut stateful_model, e);
            }
            Event::Loop(e) => {
                if let Loop::Update(args) = e {
                    controller.update(&stateless_model, &mut stateful_model, args);
                }
            }
            _ => {}
        }
    }
}
