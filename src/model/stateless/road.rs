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

    pub fn empty_side(&mut self) -> Option<&mut Vec<Lane>> {
        if self.lane_to_high.is_empty() {
            Some(&mut self.lane_to_high)
        } else if self.lane_to_low.is_empty() {
            Some(&mut self.lane_to_low)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Lane {
    pub max_speed: f64,
    pub direction_rule: TurnRule,
}
