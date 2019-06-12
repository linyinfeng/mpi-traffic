use crate::model::{
    board::{IntersectionIndex, RoadIndex},
    common::{AbsoluteDirection, LaneDirection, LaneIndex},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Car {
    pub velocity: f64,
    pub heading: f64,
    pub acceleration: f64,
    pub location: Location,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Location {
    OnLane {
        road: RoadIndex,
        direction: LaneDirection,
        lain: LaneIndex,
        position: f64,
    },
    ChangingLane {
        road: RoadIndex,
        direction: LaneDirection,
        from: LaneIndex,
        to: LaneIndex,
        position: f64,
        lane_change_position: f64,
    },
    InIntersection {
        intersection: IntersectionIndex,
        from_direction: AbsoluteDirection,
        from_lane: LaneIndex,
        to_direction: AbsoluteDirection,
        to_lane: LaneIndex,
        /// Position in intersection.
        ///
        /// Assume the path in intersection is a straight line.
        position: f64,
    },
}
