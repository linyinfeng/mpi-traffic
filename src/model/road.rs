use bitflags::bitflags;

#[derive(Clone, Debug)]
pub struct Road {
    pub lane_to_high: Vec<Lane>,
    pub lane_to_low: Vec<Lane>,
}

bitflags! {
    pub struct LaneDirectionRule: u8 {
        const STRAIGHT = 0b00000001;
        const LEFT = 0b00000010;
        const RIGHT = 0b00000100;
    }
}

#[derive(Clone, Debug)]
pub struct Lane {
    pub direction_rule: LaneDirectionRule,
}
