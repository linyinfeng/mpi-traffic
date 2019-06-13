use crate::model::{
    board::{IntersectionIndex, RoadIndex},
    common::{AbsoluteDirection, AxisDirection, LaneDirection, LaneIndex},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Car {
    pub location: Location,
    pub velocity: f64,
    pub acceleration: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Location {
    OnLane {
        road_direction: AxisDirection,
        road_index: RoadIndex,
        lane_direction: LaneDirection,
        lane_index: LaneIndex,
        position: f64,
    },
    ChangingLane {
        road_direction: AxisDirection,
        road_index: RoadIndex,
        lane_direction: LaneDirection,
        from_lane_index: LaneIndex,
        to_lane_index: LaneIndex,
        position: f64,
        /// Position in lane changing.
        lane_changed_proportion: f64,
    },
    InIntersection {
        intersection_index: IntersectionIndex,
        from_direction: AbsoluteDirection,
        from_lane_index: LaneIndex,
        to_direction: AbsoluteDirection,
        to_lane_index: LaneIndex,
        total_length: f64,
        position: f64,
    },
}
