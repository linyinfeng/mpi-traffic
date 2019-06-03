use crate::model::board::IntersectionIndex;
use crate::model::board::RoadIndex;
use crate::model::common::Direction;
use crate::model::common::LaneDirection;
use crate::model::common::LaneIndex;

#[derive(Clone, Debug)]
pub struct Car {
    pub velocity: f64,
    pub heading: f64,
    pub acceleration: f64,
    pub location: Location,
}

#[derive(Clone, Debug)]
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
        from_direction: Direction,
        from_lane: LaneIndex,
        to_direction: Direction,
        to_lane: LaneIndex,
        /// Position in intersection.
        ///
        /// Assume the path in intersection is a straight line.
        position: f64,
    },
}
