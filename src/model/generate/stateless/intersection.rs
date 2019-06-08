use crate::model::board::Board;
use crate::model::board::IntersectionContext;
use crate::model::common::AbsoluteDirection;
use crate::model::common::TurnRule;
use crate::model::stateless::intersection::{CrossroadRule, SwitchRule};
use crate::model::stateless::Intersection;
use crate::model::stateless::intersection::TJunctionRule;
use crate::model::stateless::Road;

pub const TIME_OUT: f64 = 30.0;
pub const WAIT_TIME: f64 = 3.0;
pub const MAX_SPEED: f64 = 60.0;

pub fn generate_intersections(board: &mut Board<Option<Intersection>, Option<Road>>) {
    board.intersections.indices().for_each(|index| {
        board.intersections[index] =
            generate_with_context(board, &board.context_of_intersection(index))
    })
}

fn generate_with_context(
    board: &Board<Option<Intersection>, Option<Road>>,
    context: &IntersectionContext,
) -> Option<Intersection> {
    match context.road_number() {
        0 => None,
        1 => Some(Intersection::End),
        2 => Some(generate_with_2_road(context)),
        3 => Some(generate_with_3_road(board, context)),
        4 => Some(generate_with_4_road(board, context)),
        _ => unreachable!(),
    }
}

fn is_in_same_axis(context: &IntersectionContext) -> bool {
    let mut directions = AbsoluteDirection::directions()
        .filter_map(|&direction| context.get(direction).map(|_| direction));
    directions.next().unwrap().turn_back() == directions.next().unwrap()
}

fn generate_with_2_road(context: &IntersectionContext) -> Intersection {
    if is_in_same_axis(context) {
        Intersection::Straight
    } else {
        Intersection::Turn
    }
}

fn generate_with_3_road(
    _board: &Board<Option<Intersection>, Option<Road>>,
    context: &IntersectionContext,
) -> Intersection {
    let single = AbsoluteDirection::directions()
        .find(|&&direction| context.get(direction).is_none())
        .unwrap().turn_back();
    let rule_set = vec![
        TJunctionRule {
            for_single: TurnRule::LEFT | TurnRule::RIGHT | TurnRule::BACK,
            for_left: TurnRule::RIGHT | TurnRule::BACK,
            for_right: TurnRule::FRONT | TurnRule::BACK,
        },
        TJunctionRule {
            for_single: TurnRule::BACK | TurnRule::RIGHT,
            for_left: TurnRule::RIGHT | TurnRule::BACK,
            for_right: TurnRule::FRONT | TurnRule::BACK | TurnRule::LEFT,
        },
        TJunctionRule {
            for_single: TurnRule::BACK | TurnRule::RIGHT,
            for_left: TurnRule::FRONT | TurnRule::RIGHT | TurnRule::BACK,
            for_right: TurnRule::FRONT | TurnRule::BACK,
        },
    ];
    let switch_rule = SwitchRule::LoopTimeout {
        times: vec![TIME_OUT, WAIT_TIME],
    };

    Intersection::TJunction {
        max_speed: MAX_SPEED,
        single,
        rule_set,
        switch_rule,
    }
}

fn generate_with_4_road(
    _board: &Board<Option<Intersection>, Option<Road>>,
    _context: &IntersectionContext,
) -> Intersection {
    let rules = vec![
        CrossroadRule {
            north: TurnRule::FRONT | TurnRule::RIGHT | TurnRule::BACK,
            south: TurnRule::FRONT | TurnRule::RIGHT | TurnRule::BACK,
            east: TurnRule::RIGHT | TurnRule::BACK,
            west: TurnRule::RIGHT | TurnRule::BACK,
        },
        CrossroadRule {
            east: TurnRule::FRONT | TurnRule::RIGHT | TurnRule::BACK,
            west: TurnRule::FRONT | TurnRule::RIGHT | TurnRule::BACK,
            north: TurnRule::RIGHT | TurnRule::BACK,
            south: TurnRule::RIGHT | TurnRule::BACK,
        },
        CrossroadRule {
            north: TurnRule::LEFT | TurnRule::RIGHT | TurnRule::BACK,
            south: TurnRule::LEFT | TurnRule::RIGHT | TurnRule::BACK,
            east: TurnRule::RIGHT | TurnRule::BACK,
            west: TurnRule::RIGHT | TurnRule::BACK,
        },
        CrossroadRule {
            east: TurnRule::LEFT | TurnRule::RIGHT | TurnRule::BACK,
            west: TurnRule::LEFT | TurnRule::RIGHT | TurnRule::BACK,
            south: TurnRule::RIGHT | TurnRule::BACK,
            north: TurnRule::RIGHT | TurnRule::BACK,
        },
    ];
    let switch_rule = SwitchRule::LoopTimeout {
        times: vec![TIME_OUT, WAIT_TIME],
    };
    Intersection::Crossroad {
        max_speed: MAX_SPEED,
        rules,
        switch_rule,
    }
}
