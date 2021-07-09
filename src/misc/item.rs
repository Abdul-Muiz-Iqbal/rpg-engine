use super::ItemKind;
use crate::{entities::Entity, stats::Stats};

#[derive(Builder, Debug, Clone, PartialEq)]
#[builder(pattern = "owned")]
/// Represents an Item in the game. An item can have a name, an id,
/// a kind, a restriction, description, and stats.
/// The kind determines when and how the item can be used.
/// restriction determines who can use it.
/// stats determine the effect that this item has on the stats.
pub struct Item {
    pub name: &'static str,
    pub id: usize,
    pub kind: ItemKind,
    pub restriction: Option<Vec<Entity>>,
    pub desc: &'static str,
    pub stats: Stats,
}
