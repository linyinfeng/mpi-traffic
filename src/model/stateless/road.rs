use crate::model::common::LaneDirection;
use crate::model::common::TurnRule;

#[derive(Clone, Debug)]
pub struct Road {
    pub lane_to_high: Vec<Lane>,
    pub lane_to_low: Vec<Lane>,
}

impl Road {
    pub fn is_one_way(&self) -> bool {
        self.lane_to_high.is_empty() || self.lane_to_low.is_empty()
    }

    pub fn lane_number(&self) -> usize {
        self.lane_to_high.len() + self.lane_to_low.len()
    }

    pub fn lanes_to_direction(&self, lane_direction: LaneDirection) -> &Vec<Lane> {
        match lane_direction {
            LaneDirection::HighToLow => &self.lane_to_low,
            LaneDirection::LowToHigh => &self.lane_to_high,
        }
    }

    pub fn lanes_to_direction_mut(&mut self, lane_direction: LaneDirection) -> &mut Vec<Lane> {
        match lane_direction {
            LaneDirection::HighToLow => &mut self.lane_to_low,
            LaneDirection::LowToHigh => &mut self.lane_to_high,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Lane {
    pub max_speed: f64,
    pub direction_rule: TurnRule,
}
