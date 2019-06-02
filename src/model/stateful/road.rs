use crate::model::common::CarIndex;
use std::collections::LinkedList;

#[derive(Clone, Debug)]
pub struct Road {
    pub lane_to_high: Vec<Lane>,
    pub lane_to_low: Vec<Lane>,
}

#[derive(Clone, Debug)]
pub struct Lane {
    pub cars: LinkedList<CarIndex>,
}
