#![allow(dead_code)]

use super::{Item, ItemKind};

#[derive(Debug, Clone, PartialEq)]
/// Represents the five possible equipment that an entity can have. This includes:
/// Accessory
/// Weapon
/// Armour
/// Legs
/// Feet
/// Note: Currently there is no check to make sure that each Equipment piece is a unique
/// EquipmentType
pub struct Equipment(Vec<Item>);

impl Equipment {
    /// Create a new instance of equipment given a list of equipment. This list must
    /// be less than or equal to 5, and must only contain items where Item.kind == Equipment.
    pub fn new(equipment: Vec<Item>) -> Self {
        if equipment
            .iter()
            .filter(|item| !matches!(item.kind, ItemKind::Equipment(_)))
            .count()
            > 0
        {
            panic!("Equipment must be initialized with items that have the ItemKind of Equipment");
        } else if equipment.len() > 5 {
            panic!("There can be no more than five equipment on an entity.");
        }
        Self(equipment)
    }

    /// Adds an item to the entity's equipment. Given that the item.kind == Equipment and
    /// that the current equipment equipped is less than 5.
    pub fn add(&mut self, item: Item) {
        if !matches!(item.kind, ItemKind::Equipment(_)) {
            panic!("Equipment must be initialized with items that have the ItemKind of Equipment");
        } else if self.0.len() >= 5 {
            panic!("There can be no more than five equipment on an entity.");
        }
        self.0.push(item);
    }

    /// Given an id, returns a new vector with the elements with that id removed.
    pub fn remove_by_id(&mut self, id: usize) -> Vec<Item> {
        self.0
            .clone()
            .into_iter()
            .filter(|item| item.id != id)
            .collect::<Vec<Item>>()
    }

    /// Removes the specified item from the equipment.
    pub fn remove(&mut self, item: Item) {
        self.0 = self.remove_by_id(item.id);
    }
}
