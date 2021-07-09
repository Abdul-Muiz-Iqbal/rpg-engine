#![allow(dead_code)]

use crate::{
    levels::LevelData,
    misc::{Equipment, Item},
    stats::Stats,
};

#[derive(Builder, Debug, Clone, PartialEq)]
#[builder(pattern = "owned")]
/// Represents an entity in the world. This entity can be a user controller player,
/// an npc, an enemy or any other thing.
pub struct Entity {
    /// The name of the entity as displayed in the game.
    pub name: &'static str,
    /// The id of the entity assigned on creation.
    pub id: usize,
    /// The stats of an entity in battle. Only useful for players and enemies.
    pub stats: Stats,
    /// The level data of an entity. This stores the current level and all metadata
    /// associated with it. Not as useful for enemies, but still useful for specific
    /// actions.
    pub level_data: LevelData,
    /// The equipment worn by an entity. This determines certain bonuses in stats for an
    /// entity in battle.
    pub equipment: Equipment,
}

impl Entity {
    /// Determines whether an entity can use a specific item or not. This is determined
    /// by checking if the entity is in item.restriction.
    pub fn can_use(&self, item: Item) -> bool {
        if let Some(restrictions) = item.restriction {
            restrictions.contains(&self)
        } else {
            true
        }
    }
}
