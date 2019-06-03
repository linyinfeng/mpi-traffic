use crate::model::Model;
use piston_window::context::Context;
use piston_window::rectangle;
use piston_window::G2d;

#[derive(Clone, Debug)]
pub struct View {
    pub settings: ViewSettings,
}

impl View {
    pub fn new(settings: ViewSettings) -> Self {
        Self { settings }
    }
}

/// Store settings of `ModelView`.
#[derive(Clone, Debug)]
pub struct ViewSettings {}

impl Default for ViewSettings {
    fn default() -> Self {
        Self {}
    }
}

impl View {
    pub fn draw(&self, _model: &Model, context: Context, g2d: &mut G2d) {
        // unimplemented
        rectangle(
            [1.0, 0.0, 0.0, 1.0],     // red
            [0.0, 0.0, 350.0, 250.0], // rectangle
            context.transform,
            g2d,
        );
    }
}
