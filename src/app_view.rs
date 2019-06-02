use crate::app::App;
use piston_window::context::Context;
use piston_window::rectangle;
use piston_window::G2d;

#[derive(Clone, Debug)]
pub struct AppView;

impl AppView {
    pub fn draw(&self, _app: &App, context: Context, g2d: &mut G2d) {
        // unimplemented
        rectangle(
            [1.0, 0.0, 0.0, 1.0],     // red
            [0.0, 0.0, 100.0, 100.0], // rectangle
            context.transform,
            g2d,
        );
    }
}
