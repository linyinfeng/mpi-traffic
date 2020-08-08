use log::trace;
use mpi::topology::{Communicator, Rank};
use mpi_traffic::{
    communication,
    controller::{Controller, ControllerSettings, UpdateController},
    info::Info,
    model::generate::{self, ModelGenerationSettings},
    view::{View, ViewSettings},
};
use piston_window::{
    color, Event, EventLoop, EventSettings, Loop, PistonWindow, UpdateArgs, WindowSettings,
};
use structopt::StructOpt;

fn main() {
    env_logger::init();
    let settings = MpiTrafficOpt::from_args();

    // Initialize MPI
    let universe = mpi::initialize().unwrap();
    const ROOT: Rank = 0;
    let world = universe.world();
    let root = world.process_at_rank(ROOT);

    let mut model = if world.rank() == ROOT {
        generate::generate_model(settings.model_generation_settings)
    } else {
        Default::default()
    };
    communication::bincode_broadcast(world.rank(), root, &mut model).unwrap();
    let stateless_model = model.stateless;
    let mut stateful_model = model.stateful;

    if world.rank() == ROOT {
        let mut window: PistonWindow = WindowSettings::new("MPI Traffic", [1000, 500])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("failed to build PistonWindow: {}", e));
        let event_settings = EventSettings::new().ups(60).ups_reset(10).max_fps(30);
        window.set_event_settings(event_settings);

        let view = View::new(settings.view_settings);
        let mut info = Info::new();
        let update_controller = UpdateController::new();
        let mut controller = Controller::new(update_controller, settings.controller_settings);

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
                }
                Event::Loop(e) => {
                    if let Loop::Update(args) = e {
                        let mut send_args = Some(args);
                        communication::bincode_broadcast(world.rank(), root, &mut send_args)
                            .unwrap();
                        controller.update(
                            ROOT,
                            world,
                            &mut info,
                            &mut stateful_model,
                            &stateless_model,
                            args,
                        );
                    }
                }
                _ => {}
            }
        }
        communication::bincode_broadcast::<_, Option<UpdateArgs>>(
            world.rank(),
            root,
            &mut Option::None,
        )
        .unwrap();
    } else {
        let mut controller = UpdateController::new();
        loop {
            let mut args: Option<UpdateArgs> = None;
            communication::bincode_broadcast(world.rank(), root, &mut args).unwrap();
            if let Some(args) = args {
                controller.update(ROOT, world, &mut stateful_model, &stateless_model, args);
            } else {
                break;
            }
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

    #[structopt(flatten)]
    pub view_settings: ViewSettings,
}
