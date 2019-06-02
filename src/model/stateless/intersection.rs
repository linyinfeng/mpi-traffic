use crate::model::stateless::common::DirectionRule;
use crate::model::stateless::common::Direction;

pub enum Intersection {
    Crossroad {
        rules: Vec<CrossroadRule>,
        switch_rule: SwitchRule,
    },
    TJunction {
        single: Direction,
        rule_set: Vec<TJunctionRule>,
        switch_rule: SwitchRule,
    },
    NoIntersection,
}

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
pub struct TJunctionRule {
    pub for_single: DirectionRule,
    pub for_left: DirectionRule,
    pub for_right: DirectionRule,
}

pub enum SwitchRule {
    LoopTimeout { times: Vec<f64> },
}
