use log::trace;
use mpi_traffic::{
    controller::{Controller, ControllerSettings},
    info::Info,
    model::generate::{self, ModelGenerationSettings},
    view::{View, ViewSettings},
};
use piston_window::{color, Event, EventLoop, EventSettings, Loop, PistonWindow, WindowSettings};
use structopt::StructOpt;

fn main() {
    let settings = MpiTrafficOpt::from_args();
    env_logger::init();

    let mut window: PistonWindow = WindowSettings::new("MPI Traffic", [1000, 500])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("failed to build PistonWindow: {}", e));
    let event_settings = EventSettings::new().ups(60).max_fps(60);
    window.set_event_settings(event_settings);

    let view = {
        let view_settings = ViewSettings::new();
        View::new(view_settings)
    };

    let model = generate::generate_model(settings.model_generation_settings);
    let stateless_model = model.stateless;
    let mut stateful_model = model.stateful;
    let mut info = Info::new();
    let mut controller = Controller::new(settings.controller_settings);

    while let Some(e) = window.next() {
        trace!("event: {:?}", e);
        window.draw_2d(&e, |c, g, _| {
            use piston_window::clear;
            let clear_color = color::BLACK;
            clear(clear_color, g);
            view.draw(&info, &stateless_model, &stateful_model, c, g);
        });
        match e {
            Event::Input(e, _) => {
                controller.input(&mut info, &mut stateful_model, &stateless_model, e);
            },
            Event::Loop(e) => {
                if let Loop::Update(args) = e {
                    controller.update(&mut info, &mut stateful_model, &stateless_model, args);
                }
            },
            _ => {},
        }
    }
}

#[derive(StructOpt)]
#[structopt(name = "mpi-traffic", about = "Simple traffic simulation with MPI.")]
struct MpiTrafficOpt {
    #[structopt(flatten)]
    pub model_generation_settings: ModelGenerationSettings,

    #[structopt(flatten)]
    pub controller_settings: ControllerSettings,
}
