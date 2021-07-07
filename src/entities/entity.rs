#![allow(dead_code)]

use crate::{levels::LevelData, stats::Stats};

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
pub struct Entity {
    stats: Stats,
    level_data: LevelData,
}
