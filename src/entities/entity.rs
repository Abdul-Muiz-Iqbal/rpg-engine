#![allow(dead_code)]

use crate::{levels::LevelData, stats::Stats};

#[derive(Builder, Debug, Default, Clone)]
#[builder(pattern = "owned")]
pub struct Entity {
    pub name: &'static str,
    pub id: usize,
    pub stats: Stats,
    pub level_data: LevelData,
    // Actions and Equipment
}