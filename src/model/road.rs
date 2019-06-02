use bitflags::bitflags;

#[derive(Clone, Debug)]
pub struct Road {
    pub lane_to_high: Vec<Lane>,
    pub lane_to_low: Vec<Lane>,
}

bitflags! {
    pub struct LaneDirectionRule: u8 {
        const STRAIGHT = 0b0000_0001;
        const LEFT     = 0b0000_0010;
        const RIGHT    = 0b0000_0100;
        const BACK     = 0b0000_1000;
    }
}

#[derive(Clone, Debug)]
pub struct Lane {
    pub direction_rule: LaneDirectionRule,
}
