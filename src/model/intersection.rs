use crate::model::road::LaneDirectionRule;

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

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct CrossroadRule {
    pub for_up: LaneDirectionRule,
    pub for_down: LaneDirectionRule,
    pub for_left: LaneDirectionRule,
    pub for_right: LaneDirectionRule,
}

/// T-junction intersection has 3 arms denoted with "left", "right" and "single"
///
/// For all T-junction, denote the single arm with no more road straight ahead
/// as "single". Denote the left arm of "single" as "left", the right arm of
/// "single" as "right".
pub struct TJunctionRule {
    pub for_single: LaneDirectionRule,
    pub for_left: LaneDirectionRule,
    pub for_right: LaneDirectionRule,
}

pub enum SwitchRule {
    LoopTimeout { times: Vec<f64> },
}
