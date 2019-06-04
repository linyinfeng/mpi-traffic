use crate::model::common::TurnRule;

#[derive(Clone, Debug)]
pub struct Road {
    pub lane_to_high: Vec<Lane>,
    pub lane_to_low: Vec<Lane>,
}

#[derive(Clone, Debug)]
pub struct Lane {
    pub max_speed: f64,
    pub direction_rule: TurnRule,
}
