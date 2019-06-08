use crate::model::common::AbsoluteDirection;
use crate::model::common::Around;
use crate::model::common::TurnRule;

#[derive(Clone, Debug)]
pub enum Intersection {
    Crossroad {
        max_speed: f64,
        rules: Vec<CrossroadRule>,
        switch_rule: SwitchRule,
    },
    TJunction {
        max_speed: f64,
        single: AbsoluteDirection,
        rule_set: Vec<TJunctionRule>,
        switch_rule: SwitchRule,
    },
    Turn,
    Straight,
    End, // only one road connected in and out
}

impl Intersection {
    pub fn default_crossroad() -> Self {
        Intersection::Crossroad {
            max_speed: unimplemented!(),
            rules: unimplemented!(),
            switch_rule: unimplemented!(),
        }
    }
}

pub type CrossroadRule = Around<TurnRule>;

/// T-junction intersection has 3 arms denoted with "left", "right" and "single"
///
/// For all T-junction, denote the single arm with no more road straight ahead
/// as "single". Denote the left arm of "single" as "left", the right arm of
/// "single" as "right".
#[derive(Clone, Debug)]
pub struct TJunctionRule {
    pub for_single: TurnRule,
    pub for_left: TurnRule,
    pub for_right: TurnRule,
}

#[derive(Clone, Debug)]
pub enum SwitchRule {
    LoopTimeout { times: Vec<f64> },
}
