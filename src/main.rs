#[macro_use]
extern crate derive_builder;
mod entities;
mod levels;
mod stats;

use crate::{entities::EntityBuilder, levels::LevelData, stats::Stats};

// TODO:
// UnitTests, Integration Tests
// Increase tightness of all types
// Better error handling via thiserror
// Better documentation with examples
// Organization of various models (Especially create_stat_table and Entity)

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let player = EntityBuilder::default()
        .stats(Stats::default())
        .level_data(LevelData::new(45, 10))
        .build()?;

    dbg!(player);

    Ok(())
}
