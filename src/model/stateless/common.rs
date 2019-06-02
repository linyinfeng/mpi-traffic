use bitflags::bitflags;

bitflags! {
    pub struct DirectionRule: u8 {
        const STRAIGHT = 0b0000_0001;
        const LEFT     = 0b0000_0010;
        const RIGHT    = 0b0000_0100;
        const BACK     = 0b0000_1000;
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
