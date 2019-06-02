use log::trace;
use mpi_traffic::app::App;
use mpi_traffic::app_view::AppView;
use piston_window::Event;
use piston_window::EventLoop;
use piston_window::EventSettings;
use piston_window::Loop;
use piston_window::PistonWindow;
use piston_window::WindowSettings;

fn main() {
    env_logger::init();

    let mut window: PistonWindow = WindowSettings::new("MPI Traffic", [700, 500])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("failed to build PistonWindow: {}", e));
    let event_settings = EventSettings::new().ups(120).max_fps(60);
    window.set_event_settings(event_settings);

    let mut app = App;
    let app_view = AppView;

    while let Some(e) = window.next() {
        trace!("event: {:?}", e);
        window.draw_2d(&e, |c, g, _| {
            app_view.draw(&app, c, g);
        });
        match e {
            Event::Input(e) => match e {
                _ => {},
            },
            Event::Loop(e) => {
                if let Loop::Update(args) = e {
                    app.update(args);
                }
            },
            _ => {},
        }
    }
}
