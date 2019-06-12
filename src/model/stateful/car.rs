use crate::model::{
    board::{IntersectionIndex, RoadIndex},
    common::{AbsoluteDirection, LaneDirection, LaneIndex},
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
        road: RoadIndex,
        direction: LaneDirection,
        lane: LaneIndex,
        position: f64,
    },
    ChangingLane {
        road: RoadIndex,
        direction: LaneDirection,
        from: LaneIndex,
        to: LaneIndex,
        position: f64,
        /// Position in lane changing.
        lane_changed_proportion: f64,
    },
    InIntersection {
        intersection: IntersectionIndex,
        from_direction: AbsoluteDirection,
        from_lane: LaneIndex,
        to_direction: AbsoluteDirection,
        to_lane: LaneIndex,
        /// Proportion in intersection.
        ///
        /// Assume the path in intersection is a straight line.
        in_intersection_proportion: f64,
    },
}
