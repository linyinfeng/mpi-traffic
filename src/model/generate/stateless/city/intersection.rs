use crate::model::{
    board::{Board, IntersectionContext},
    common::{AbsoluteDirection, TurnRule},
    generate::stateless::StatelessModelGenerationSettings,
    stateless::{
        intersection::{CrossroadRule, SwitchRule, TJunctionRule},
        Intersection, Road,
    },
};

pub const TIME_OUT: f64 = 30.0;
pub const MAX_SPEED: f64 = 60.0;

pub fn generate_intersections(
    board: &mut Board<Option<Intersection>, Option<Road>>,
    settings: &StatelessModelGenerationSettings,
) {
    for index in board.intersections.indices() {
        let context = board.context_of_intersection(index);
        if context.road_number() != 0 {
            board.intersections[index] = Some(generate_with_context(&context, settings));
        }
    }
}

fn generate_with_context(
    context: &IntersectionContext,
    settings: &StatelessModelGenerationSettings,
) -> Intersection {
    match context.road_number() {
        1 => Intersection::End,
        2 => generate_with_2_road(context),
        3 => generate_with_3_road(context, settings),
        4 => generate_with_4_road(settings),
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
    context: &IntersectionContext,
    settings: &StatelessModelGenerationSettings,
) -> Intersection {
    let single = AbsoluteDirection::directions()
        .find(|&&direction| context.get(direction).is_none())
        .unwrap()
        .turn_back();
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
        times: vec![settings.time_out],
    };

    Intersection::TJunction {
        max_speed: settings.intersection_max_speed,
        single,
        rule_set,
        switch_rule,
    }
}

fn generate_with_4_road(settings: &StatelessModelGenerationSettings) -> Intersection {
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
        times: vec![settings.time_out],
    };
    Intersection::Crossroad {
        max_speed: settings.intersection_max_speed,
        rules,
        switch_rule,
    }
}
