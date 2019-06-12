//! Currently nothing


use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Road {
    pub lane_to_high: Vec<Lane>,
    pub lane_to_low: Vec<Lane>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lane;
