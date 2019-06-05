#[derive(Clone, Debug)]
pub enum Intersection {
    Crossroad { switch_state: SwitchState },
    TJunction { switch_state: SwitchState },
    Turn,
    Straight,
    End,
}

#[derive(Clone, Debug)]
pub enum SwitchState {
    LoopTimeout {
        remain_time: f64,
        time_index: usize,
        rule_index: usize,
    },
}