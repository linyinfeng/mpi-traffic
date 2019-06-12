use crate::model::{
    common::{AxisDirection, Geometry, LaneDirection, TurnRule},
    stateful, stateless,
};
use log::trace;
use piston_window::{
    color,
    context::Context,
    polygon, rectangle,
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
    pub road_middle_separator_color: Color,
    pub road_middle_separator_width: f64,
    pub lane_sign_padding: f64, // a lane is typically 3.5 meters wide
    pub intersection_color: Color,
    pub intersection_sign_color: Color,
}

impl ViewSettings {
    pub fn new() -> Self {
        Self {
            padding: 10.0,
            road_color: color::grey(0.4),
            road_sign_color: color::WHITE,
            road_middle_separator_color: color::hex("ffdb4d"),
            road_middle_separator_width: 0.4,
            lane_sign_padding: 0.2,
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
        let lane_number = road.lane_number();
        let center_distance = (lane_number - 1) as f64 * lane_width;
        let mut center_y = -center_distance / 2.0;
        let half_length = length / 2.0;
        let middle = center_y + road.lane_to_high.len() as f64 * lane_width - lane_width / 2.0;
        for direction in [LaneDirection::HighToLow, LaneDirection::LowToHigh].iter() {
            let iter = road.lanes_to_direction(*direction).iter();
            let iter: Box<dyn Iterator<Item = &stateless::Lane>> = match direction {
                LaneDirection::HighToLow => Box::new(iter.rev()),
                LaneDirection::LowToHigh => Box::new(iter),
            };
            for lane in iter {
                self.draw_lane(
                    lane,
                    length,
                    lane_width,
                    transform.trans(0.0, center_y).rot_deg(match direction {
                        LaneDirection::HighToLow => 180.0,
                        LaneDirection::LowToHigh => 0.0,
                    }),
                    g2d,
                );
                center_y += lane_width;
            }
        }
        if !road.is_one_way() {
            // draw middle sperator line
            rectangle(
                self.settings.road_middle_separator_color,
                [
                    -half_length,
                    middle - self.settings.road_middle_separator_width / 2.0,
                    length,
                    self.settings.road_middle_separator_width,
                ],
                transform,
                g2d,
            );
        }
    }

    pub fn draw_lane(
        &self,
        lane: &stateless::Lane,
        length: f64,
        width: f64,
        transform: Matrix2d,
        g2d: &mut G2d,
    ) {
        let half_length = length / 2.0;
        let half_width = width / 2.0;
        rectangle(
            self.settings.road_color,
            [-half_length, -half_width, length, width],
            transform,
            g2d,
        );
        let sign_half_size = (width - self.settings.lane_sign_padding) / 2.0;
        self.draw_turn_rule_as_sign(
            lane.direction_rule,
            self.settings.road_sign_color,
            transform
                .trans(half_length - half_width, 0.0)
                .rot_deg(90.0)
                .zoom(sign_half_size),
            g2d,
        );
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
        );
    }

    /// Draw turn rule in (-1.0, -1.0) to (1.0, 1.0) or top left to down right
    pub fn draw_turn_rule_as_sign(
        &self,
        turn_rule: TurnRule,
        color: Color,
        transform: Matrix2d,
        g2d: &mut G2d,
    ) {
        if turn_rule != TurnRule::empty() {
            let size = 2.0;
            let half_size = size / 2.0;
            let center_size = 0.2;
            let half_center_size = center_size / 2.0;
            let arrow_half_width = 0.3;
            let arrow_height = 0.3;
            let arrow_arm_length = half_size - arrow_height;
            let back_arrow_location = -0.5;
            rectangle(
                color,
                [
                    -half_center_size,
                    -half_center_size,
                    center_size,
                    center_size,
                ],
                transform,
                g2d,
            );
            rectangle(
                color,
                [
                    -half_center_size,
                    -half_center_size,
                    center_size,
                    half_center_size + half_size,
                ],
                transform,
                g2d,
            );
            if turn_rule.intersects(TurnRule::FRONT) {
                rectangle(
                    color,
                    [
                        -half_center_size,
                        -arrow_arm_length,
                        center_size,
                        arrow_arm_length,
                    ],
                    transform,
                    g2d,
                );
                polygon(
                    color,
                    &[
                        [0.0, -half_size],
                        [arrow_half_width, -arrow_arm_length],
                        [-arrow_half_width, -arrow_arm_length],
                    ],
                    transform,
                    g2d,
                );
            }
            if turn_rule.intersects(TurnRule::LEFT) {
                rectangle(
                    color,
                    [
                        -arrow_arm_length,
                        -half_center_size,
                        arrow_arm_length,
                        center_size,
                    ],
                    transform,
                    g2d,
                );
                polygon(
                    color,
                    &[
                        [-half_size, 0.0],
                        [-arrow_arm_length, arrow_half_width],
                        [-arrow_arm_length, -arrow_half_width],
                    ],
                    transform,
                    g2d,
                );
            }
            if turn_rule.intersects(TurnRule::RIGHT) {
                rectangle(
                    color,
                    [0.0, -half_center_size, arrow_arm_length, center_size],
                    transform,
                    g2d,
                );
                polygon(
                    color,
                    &[
                        [half_size, 0.0],
                        [arrow_arm_length, arrow_half_width],
                        [arrow_arm_length, -arrow_half_width],
                    ],
                    transform,
                    g2d,
                );
            }
            if turn_rule.intersects(TurnRule::BACK) {
                rectangle(
                    color,
                    [
                        back_arrow_location - half_center_size,
                        -half_center_size,
                        -(back_arrow_location - half_center_size),
                        center_size,
                    ],
                    transform,
                    g2d,
                );
                rectangle(
                    color,
                    [
                        back_arrow_location - half_center_size,
                        0.0,
                        center_size,
                        arrow_arm_length,
                    ],
                    transform,
                    g2d,
                );
                polygon(
                    color,
                    &[
                        [back_arrow_location, half_size],
                        [back_arrow_location - arrow_half_width, arrow_arm_length],
                        [back_arrow_location + arrow_half_width, arrow_arm_length],
                    ],
                    transform,
                    g2d,
                );
            }
        }
    }
}
