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
}

#[derive(Clone, Debug)]
pub struct Lane {
    pub max_speed: f64,
    pub direction_rule: TurnRule,
}
