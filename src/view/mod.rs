use crate::{
    info::Info,
    model::{
        board::{IntersectionIndex, RoadIndex},
        common::{
            AbsoluteDirection, AxisDirection, Geometry, InOutDirection, LaneDirection, LaneIndex,
            Position, RelativeDirection, TurnRule,
        },
        stateful, stateless,
    },
};
use log::trace;
use piston_window::{
    context::Context,
    polygon, rectangle,
    types::{Color, Matrix2d},
    G2d, Transformed,
};
use structopt::StructOpt;

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
#[derive(StructOpt, Default, Clone, Debug)]
pub struct ViewSettings {
    #[structopt(name = "view-padding", long, default_value = "10.0")]
    pub padding: f64,
    #[structopt(
        name = "view-road-color",
        long = "view-road-color",
        default_value = "666666",
        parse(from_str = "piston_window::color::hex")
    )]
    pub road_color: Color,
    #[structopt(
        name = "view-road-sign-color",
        long = "view-road-sign-color",
        default_value = "ffffff",
        parse(from_str = "piston_window::color::hex")
    )]
    pub road_sign_color: Color,
    #[structopt(
        name = "view-road-middle-separator-color",
        long = "view-road-middle-separator-color",
        default_value = "ffdb4d",
        parse(from_str = "piston_window::color::hex")
    )]
    pub road_middle_separator_color: Color,
    #[structopt(
        name = "view-road-middle-separator-width",
        long = "view-road-middle-separator-width",
        default_value = "0.4"
    )]
    pub road_middle_separator_width: f64,
    #[structopt(
        name = "view-lane-sign-padding",
        long = "view-lane-sign-padding",
        default_value = "0.2"
    )]
    pub lane_sign_padding: f64,
    #[structopt(
        name = "view-intersection-color",
        long = "view-intersection-color",
        default_value = "737373",
        parse(from_str = "piston_window::color::hex")
    )]
    pub intersection_color: Color,
    #[structopt(
        name = "view-intersection-sign-color",
        long = "view-intersection-sign-color",
        default_value = "66ff33",
        parse(from_str = "piston_window::color::hex")
    )]
    pub intersection_sign_color: Color,
    #[structopt(
        name = "view-car-color",
        long = "view-car-color",
        default_value = "ff0066",
        parse(from_str = "piston_window::color::hex")
    )]
    pub car_color: Color,
    #[structopt(
        name = "view-car-length",
        long = "view-car-length",
        default_value = "4.5"
    )]
    pub car_length: f64,
    #[structopt(
        name = "view-car-width",
        long = "view-car-width",
        default_value = "1.7"
    )]
    pub car_width: f64,
}

impl View {
    pub fn draw(
        &self,
        info: &Info,
        stateless_model: &stateless::Model,
        stateful_model: &stateful::Model,
        context: Context,
        g2d: &mut G2d,
    ) {
        let context = context.trans(info.x, info.y).zoom(info.zoom);

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
                let length = stateless_model.city.road_length(direction, (i, j));
                self.draw_road(
                    lane_width,
                    length,
                    road,
                    self.transform_to_road_center(
                        model_context.transform,
                        &stateless_model.city,
                        direction,
                        (i, j),
                    ),
                    g2d,
                );
            }
        }
        for (((i, j), intersection), state) in stateless_model
            .city
            .board
            .intersections
            .enumerate()
            .zip(stateful_model.city.board.intersections.iter())
        {
            if let Some(intersection) = intersection.as_ref() {
                let geometry = stateless_model.city.intersection_geometry((i, j));
                self.draw_intersection(
                    geometry,
                    intersection,
                    state.as_ref().unwrap(),
                    self.transform_to_intersection_center(
                        model_context.transform,
                        &stateless_model.city,
                        (i, j),
                    ),
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
        state: &stateful::Intersection,
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
        let sign_size = if half_height < half_width {
            half_height
        } else {
            half_width
        };
        let half_sign_size = sign_size / 2.0;
        let sign_x = half_width - half_sign_size;
        let sign_y = half_height - half_sign_size;
        let draws = [
            (AbsoluteDirection::North, -sign_x, -sign_y, 180.0),
            (AbsoluteDirection::East, sign_x, -sign_y, 270.0),
            (AbsoluteDirection::South, sign_x, sign_y, 0.0),
            (AbsoluteDirection::West, -sign_x, sign_y, 90.0),
        ];
        if let Some(current) = match state {
            stateful::Intersection::Crossroad { current, .. } => Some(current),
            stateful::Intersection::TJunction { current, .. } => Some(current),
            _ => None,
        } {
            for &(d, x, y, rot) in draws.iter() {
                self.draw_turn_rule_as_sign(
                    *current.get(d),
                    self.settings.intersection_sign_color,
                    transform.trans(x, y).zoom(half_sign_size).rot_deg(rot),
                    g2d,
                );
            }
        }
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

    pub fn draw_car(
        &self,
        _stateless: &stateless::Car,
        stateful: &stateful::Car,
        city: &stateless::City,
        transform: Matrix2d,
        g2d: &mut G2d,
    ) {
        match stateful.location {
            stateful::car::Location::OnLane {
                road_direction,
                road_index,
                lane_direction,
                lane_index,
                position,
            } => {
                let length = city.road_length(road_direction, road_index);
                let x = -length / 2.0 + position;
                let heading = self.car_heading_deg_on_road(road_direction, lane_direction);
                self.draw_car_only(
                    self.transform_to_lane_center(
                        transform,
                        city,
                        road_direction,
                        road_index,
                        lane_direction,
                        lane_index,
                    )
                    .trans(x, 0.0)
                    .rot_deg(heading),
                    g2d,
                );
            },
            stateful::car::Location::ChangingLane {
                road_direction,
                road_index,
                lane_direction,
                from_lane_index,
                to_lane_index,
                position,
                lane_changed_proportion,
            } => {
                let length = city.road_length(road_direction, road_index);
                let x = -length / 2.0 + position;
                let lane_changed_offset = lane_changed_proportion *
                    city.lane_width *
                    (to_lane_index - from_lane_index) as f64 *
                    match lane_direction {
                        LaneDirection::HighToLow => 1.0,
                        LaneDirection::LowToHigh => -1.0,
                    };
                let heading = self.car_heading_deg_on_road(road_direction, lane_direction);
                self.draw_car_only(
                    self.transform_to_lane_center(
                        transform,
                        city,
                        road_direction,
                        road_index,
                        lane_direction,
                        from_lane_index,
                    )
                    .trans(x, lane_changed_offset)
                    .rot_deg(heading),
                    g2d,
                );
            },
            stateful::car::Location::InIntersection {
                intersection_index,
                from_direction,
                from_lane_index,
                to_direction,
                to_lane_index,
                total_length,
                position,
            } => {
                let Position {
                    x: from_x,
                    y: from_y,
                } = city
                    .intersection_road_join_position(
                        intersection_index,
                        from_direction,
                        InOutDirection::In,
                        from_lane_index,
                    )
                    .unwrap();
                let Position { x: to_x, y: to_y } = city
                    .intersection_road_join_position(
                        intersection_index,
                        to_direction,
                        InOutDirection::Out,
                        to_lane_index,
                    )
                    .unwrap();
                let proportion = position / total_length;
                let dx = to_x - from_x;
                let dy = to_y - from_y;
                let x = dx * proportion + from_x;
                let y = dy * proportion + from_y;
                let turn_direction = from_direction.turn_back() // convert to driver's direction
                    .should_turn(to_direction);
                let origin_heading = self.car_heading_deg_on_road(
                    from_direction.axis_direction(),
                    LaneDirection::absolute_in_out_to_lane(from_direction, InOutDirection::In),
                );
                let turn_heading = self.car_heading_offset_deb_to_turn(turn_direction);
                let heading = origin_heading + turn_heading * proportion;
                self.draw_car_only(
                    self.transform_to_intersection_center(transform, city, intersection_index)
                        .trans(x, y)
                        .rot_deg(heading),
                    g2d,
                );
            },
        }
    }

    /// Draw a car under centralized coordinate system.
    ///
    /// The car is heading to north.
    pub fn draw_car_only(&self, transform: Matrix2d, g2d: &mut G2d) {
        let height = self.settings.car_length;
        let width = self.settings.car_width;
        let half_height = height / 2.0;
        let half_width = width / 2.0;
        rectangle(
            self.settings.car_color,
            [-half_width, -half_height, width, height],
            transform,
            g2d,
        );
    }

    fn transform_to_road_center(
        &self,
        transform: Matrix2d,
        city: &stateless::City,
        direction: AxisDirection,
        index: RoadIndex,
    ) -> Matrix2d {
        use AxisDirection::*;
        let center = city.road_center(direction, index);
        transform
            .trans(center.x, center.y)
            .rot_deg(match direction {
                Horizontal => 0.0,
                Vertical => 90.0,
            })
    }

    fn transform_to_lane_center(
        &self,
        transform: Matrix2d,
        city: &stateless::City,
        road_direction: AxisDirection,
        road_index: RoadIndex,
        lane_direction: LaneDirection,
        lane_index: LaneIndex,
    ) -> Matrix2d {
        let road = city
            .board
            .get_road(road_direction, road_index)
            .unwrap()
            .as_ref()
            .unwrap();
        let offset = city.lane_center_offset(road, lane_direction, lane_index);
        self.transform_to_road_center(transform, city, road_direction, road_index)
            .trans(0.0, offset)
    }

    fn transform_to_intersection_center(
        &self,
        transform: Matrix2d,
        city: &stateless::City,
        index: IntersectionIndex,
    ) -> Matrix2d {
        let center = city.intersection_center(index);
        transform.trans(center.x, center.y)
    }

    fn car_heading_deg_on_road(
        &self,
        road_direction: AxisDirection,
        lane_direction: LaneDirection,
    ) -> f64 {
        use AxisDirection::*;
        use LaneDirection::*;
        match (road_direction, lane_direction) {
            (Horizontal, HighToLow) => 270.0,
            (Horizontal, LowToHigh) => 90.0,
            (Vertical, HighToLow) => 0.0,
            (Vertical, LowToHigh) => 180.0,
        }
    }

    fn car_heading_offset_deb_to_turn(&self, direction: RelativeDirection) -> f64 {
        use RelativeDirection::*;
        match direction {
            Front => 0.0,
            Back => -180.0,
            Left => -90.0,
            Right => 90.0,
        }
    }
}
