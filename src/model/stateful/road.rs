use crate::model::common::CarIndex;
use serde::{Deserialize, Serialize};
use std::collections::LinkedList;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Road {
    pub lane_to_high: Vec<Lane>,
    pub lane_to_low: Vec<Lane>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lane {
    pub cars: LinkedList<CarIndex>,
}
