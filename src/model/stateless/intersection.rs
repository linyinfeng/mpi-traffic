use crate::model::common::Direction;
use crate::model::common::DirectionRule;

#[derive(Clone, Debug)]
pub enum Intersection {
    Crossroad {
        max_speed: f64,
        rules: Vec<CrossroadRule>,
        switch_rule: SwitchRule,
    },
    TJunction {
        max_speed: f64,
        single: Direction,
        rule_set: Vec<TJunctionRule>,
        switch_rule: SwitchRule,
    },
    NoIntersection,
}

#[derive(Clone, Debug)]
pub struct CrossroadRule {
    pub for_up: DirectionRule,
    pub for_down: DirectionRule,
    pub for_left: DirectionRule,
    pub for_right: DirectionRule,
}

/// T-junction intersection has 3 arms denoted with "left", "right" and "single"
///
/// For all T-junction, denote the single arm with no more road straight ahead
/// as "single". Denote the left arm of "single" as "left", the right arm of
/// "single" as "right".
#[derive(Clone, Debug)]
pub struct TJunctionRule {
    pub for_single: DirectionRule,
    pub for_left: DirectionRule,
    pub for_right: DirectionRule,
}

#[derive(Clone, Debug)]
pub enum SwitchRule {
    LoopTimeout {
        times: Vec<f64>,
        lane_change_yaw: f64,
    },
}
