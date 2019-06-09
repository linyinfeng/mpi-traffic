use rand::distributions::{Distribution, Standard};
use rand::Rng;

use bitflags::bitflags;

pub type CarIndex = usize;
pub type LaneIndex = usize;

bitflags! {
    pub struct TurnRule: u8 {
        const FRONT = 0b0000_0001;
        const LEFT  = 0b0000_0010;
        const RIGHT = 0b0000_0100;
        const BACK  = 0b0000_1000;
        const ALL   = Self::FRONT.bits | Self::LEFT.bits | Self::RIGHT.bits | Self::BACK.bits;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AbsoluteDirection {
    North,
    West,
    South,
    East,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RelativeDirection {
    Front,
    Right,
    Back,
    Left,
}

impl AbsoluteDirection {
    pub fn turn_back(self) -> AbsoluteDirection {
        use AbsoluteDirection::*;
        match self {
            East => West,
            West => East,
            North => South,
            South => North,
        }
    }

    pub fn turn_left(self) -> AbsoluteDirection {
        use AbsoluteDirection::*;
        match self {
            East => South,
            West => North,
            North => East,
            South => West,
        }
    }

    pub fn turn_right(self) -> AbsoluteDirection {
        use AbsoluteDirection::*;
        match self {
            East => North,
            West => South,
            North => West,
            South => East,
        }
    }

    pub fn turn(self, t: RelativeDirection) -> AbsoluteDirection {
        use RelativeDirection::*;

        match t {
            Left => self.turn_left(),
            Right => self.turn_right(),
            Front => self,
            Back => self.turn_back(),
        }
    }

    pub fn should_turn(self, other: AbsoluteDirection) -> RelativeDirection {
        use RelativeDirection::*;

        if self == other {
            Front
        } else if self.turn_left() == other {
            Left
        } else if self.turn_right() == other {
            Right
        } else {
            Back
        }
    }

    pub fn axis_direction(self) -> AxisDirection {
        use AbsoluteDirection::*;
        match self {
            East | West => AxisDirection::Horizontal,
            North | South => AxisDirection::Vertical,
        }
    }
}

impl AbsoluteDirection {
    pub fn directions() -> std::slice::Iter<'static, AbsoluteDirection> {
        use AbsoluteDirection::*;
        static DIRECTIONS: [AbsoluteDirection; 4] = [North, South, East, West];
        DIRECTIONS.iter()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AxisDirection {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LaneDirection {
    LowToHigh,
    HighToLow,
}

impl LaneDirection {
    pub fn lane_directions() -> std::slice::Iter<'static, LaneDirection> {
        use LaneDirection::*;
        static LANE_DIRECTIONS: [LaneDirection; 2] = [LowToHigh, HighToLow];
        LANE_DIRECTIONS.iter()
    }

    pub fn absolute_in_out_to_lane(
        absolute_direction: AbsoluteDirection,
        in_out: InOutDirection,
    ) -> Self {
        use AbsoluteDirection::*;
        use InOutDirection::*;
        use LaneDirection::*;
        match absolute_direction {
            North | East => match in_out {
                In => HighToLow,
                Out => LowToHigh,
            },
            South | West => match in_out {
                In => LowToHigh,
                Out => HighToLow,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum InOutDirection {
    In,
    Out,
}

impl InOutDirection {
    pub fn in_or_out(absolute_direction: AbsoluteDirection, lane_direction: LaneDirection) -> Self {
        use AbsoluteDirection::*;
        use InOutDirection::*;
        use LaneDirection::*;
        match absolute_direction {
            North | East => match lane_direction {
                LowToHigh => Out,
                HighToLow => In,
            },
            South | West => match lane_direction {
                LowToHigh => In,
                HighToLow => Out,
            },
        }
    }
}

impl AbsoluteDirection {
    pub fn of_lane(axis: AxisDirection, lane_direction: LaneDirection) -> AbsoluteDirection {
        use AbsoluteDirection::*;
        use AxisDirection::*;
        use LaneDirection::*;

        match (axis, lane_direction) {
            (Horizontal, LowToHigh) => West,
            (Horizontal, HighToLow) => East,
            (Vertical, LowToHigh) => South,
            (Vertical, HighToLow) => North,
        }
    }
}

impl Distribution<AbsoluteDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AbsoluteDirection {
        use AbsoluteDirection::*;
        match rng.gen_range(0, 4) {
            0 => North,
            1 => South,
            2 => East,
            3 => West,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Around<T> {
    pub north: T,
    pub west: T,
    pub south: T,
    pub east: T,
}

impl<T> Around<T> {
    pub fn get(&self, direction: AbsoluteDirection) -> &T {
        use AbsoluteDirection::*;
        match direction {
            North => &self.north,
            West => &self.west,
            South => &self.south,
            East => &self.east,
        }
    }

    pub fn get_mut(&mut self, direction: AbsoluteDirection) -> &mut T {
        use AbsoluteDirection::*;
        match direction {
            North => &mut self.north,
            West => &mut self.west,
            South => &mut self.south,
            East => &mut self.east,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Geometry {
    pub width: f64,
    pub height: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[cfg(test)]
mod test {
    use AbsoluteDirection::*;
    use AxisDirection::*;
    use LaneDirection::*;
    use RelativeDirection::*;

    use super::*;

    #[test]
    fn turn() {
        let cases = vec![
            ((North, East), Left),
            ((East, South), Left),
            ((South, West), Left),
            ((West, North), Left),
            ((North, West), Right),
            ((West, South), Right),
            ((South, East), Right),
            ((East, North), Right),
            ((North, North), Front),
            ((West, West), Front),
            ((South, South), Front),
            ((East, East), Front),
            ((North, South), Back),
            ((West, East), Back),
            ((South, North), Back),
            ((East, West), Back),
        ];
        for ((from, to), relative) in cases.into_iter() {
            assert_eq!(from.turn(relative), to);
            assert_eq!(from.should_turn(to), relative);
        }
    }

    #[test]
    fn absolute_direction_of_lane() {
        let cases = vec![
            ((Vertical, HighToLow), North),
            ((Vertical, LowToHigh), South),
            ((Horizontal, HighToLow), East),
            ((Horizontal, LowToHigh), West),
        ];
        for ((axis_direction, lane_direction), absolute) in cases.into_iter() {
            assert_eq!(
                AbsoluteDirection::of_lane(axis_direction, lane_direction),
                absolute
            );
        }
    }
}
