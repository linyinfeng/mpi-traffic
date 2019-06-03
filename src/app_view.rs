use crate::app::App;
use piston_window::context::Context;
use piston_window::rectangle;
use piston_window::G2d;

#[derive(Clone, Debug)]
pub struct AppView {
    pub settings: AppViewSettings,
}

impl AppView {
    pub fn new(settings: AppViewSettings) -> Self {
        Self { settings }
    }
}

/// Store settings of `AppView`.
#[derive(Clone, Debug)]
pub struct AppViewSettings {}

impl Default for AppViewSettings {
    fn default() -> Self {
        Self {}
    }
}

impl AppView {
    pub fn draw(&self, _app: &App, context: Context, g2d: &mut G2d) {
        // unimplemented
        rectangle(
            [1.0, 0.0, 0.0, 1.0],     // red
            [0.0, 0.0, 350.0, 250.0], // rectangle
            context.transform,
            g2d,
        );
    }
}
