use crate::model::stateful;
use crate::model::stateless;
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
#[derive(Default, Clone, Debug)]
pub struct ViewSettings {
    padding: f64,
}

impl ViewSettings {
    pub fn new() -> Self {
        Self { padding: 10.0 }
    }
}

impl View {
    pub fn draw(
        &self,
        stateless_model: &stateless::Model,
        _stateful_model: &stateful::Model,
        context: Context,
        g2d: &mut G2d,
    ) {
        // Draw city
        let _city_geometry = stateless_model.city.geometry();

        // unimplemented
        rectangle(
            [1.0, 0.0, 0.0, 1.0],     // red
            [0.0, 0.0, 350.0, 250.0], // rectangle
            context.transform,
            g2d,
        );
    }
}
