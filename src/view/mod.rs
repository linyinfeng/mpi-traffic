use crate::model::{
    common::{AxisDirection, Geometry},
    stateful, stateless,
};
use log::trace;
use piston_window::{
    color,
    context::Context,
    rectangle,
    types::{Color, Matrix2d},
    G2d, Transformed,
};

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
    pub padding: f64,
    pub road_color: Color,
    pub road_sign_color: Color,
    pub intersection_color: Color,
    pub intersection_sign_color: Color,
}

impl ViewSettings {
    pub fn new() -> Self {
        Self {
            padding: 10.0,
            road_color: color::grey(0.4),
            road_sign_color: color::WHITE,
            intersection_color: color::grey(0.5),
            intersection_sign_color: color::WHITE,
        }
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
        // Model logical width and model height
        let Geometry {
            width: mw,
            height: mh,
        } = stateless_model.city.geometry();
        // Window width and window height
        let [ww, wh] = context
            .viewport
            .expect("no viewport info provided")
            .window_size;
        // Model container width and model container height
        let (cw, ch) = (
            ww - 2.0 * self.settings.padding,
            wh - 2.0 * self.settings.padding,
        );
        let (cx, cy) = (self.settings.padding, self.settings.padding);

        let model_context = {
            let model_ratio = mw / mh;
            let container_ratio = cw / ch;
            let zoom = if model_ratio > container_ratio {
                cw / mw
            } else {
                ch / mh
            };
            let (zw, zh) = (mw * zoom, mh * zoom);
            let (x, y) = if model_ratio > container_ratio {
                (cx, cy + (ch - zh) / 2.0)
            } else {
                (cx + (cw - zw) / 2.0, cy)
            };
            // Transform from model coordinates to model container coordinates
            context.trans(x, y).zoom(zoom)
        };
        // Draw horizontal roads
        trace!("start draw roads");
        let lane_width = stateless_model.city.lane_width;
        for ((i, j), (direction, road)) in stateless_model.city.board.enumerate_roads() {
            if let Some(road) = road.as_ref() {
                use AxisDirection::*;
                let length = stateless_model.city.road_length(direction, (i, j));
                let center = stateless_model.city.road_center(direction, (i, j));
                self.draw_road(
                    lane_width,
                    length,
                    road,
                    model_context
                        .transform
                        .trans(center.x, center.y)
                        .rot_deg(match direction {
                            Horizontal => 0.0,
                            Vertical => 90.0,
                        }),
                    g2d,
                );
            }
        }
        for ((i, j), intersection) in stateless_model.city.board.intersections.enumerate() {
            if let Some(intersection) = intersection.as_ref() {
                let geometry = stateless_model.city.intersection_geometry((i, j));
                let center = stateless_model.city.intersection_center((i, j));
                self.draw_intersection(
                    geometry,
                    intersection,
                    model_context.transform.trans(center.x, center.y),
                    g2d,
                );
            }
        }
    }

    /// Draw a horizontal road.
    pub fn draw_road(
        &self,
        lane_width: f64,
        length: f64,
        road: &stateless::Road,
        transform: Matrix2d,
        g2d: &mut G2d,
    ) {
        let width = road.lane_number() as f64 * lane_width;
        let half_width = width / 2.0;
        let half_length = length / 2.0;
        rectangle(
            self.settings.road_color,
            [-half_length, -half_width, length, width],
            transform,
            g2d,
        )
    }

    pub fn draw_intersection(
        &self,
        g: Geometry,
        _intersection: &stateless::Intersection,
        transform: Matrix2d,
        g2d: &mut G2d,
    ) {
        let half_width = g.width / 2.0;
        let half_height = g.height / 2.0;
        rectangle(
            self.settings.intersection_color,
            [-half_width, -half_height, g.width, g.height],
            transform,
            g2d,
        )
    }
}
