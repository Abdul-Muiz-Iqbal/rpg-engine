#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
/// Represents the possible kinds of items there can be in the game.
pub enum ItemKind {
    /// Key items cannot ever be thrown away, they are only affected by how the story
    /// progresses.
    KeyItem,
    /// Items that are ONLY usable in battle. This can include expendable throwing items
    /// such as shurikens.
    UsableInBattle,
    /// Items that can ONLY be used in the field. Examples of this can include the Phone.
    UsableInField,
    /// Items that can be used anywhere, whether in the field or battle. This includes
    /// healing items such as potions.
    UsableEverywhere,
    /// Items that can be equipped onto entities. They usually provide some bonus to the
    /// stats.
    Equipment(EquipmentType),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
/// Represents the types of equipment an entity can have. They can only have one of each.
pub enum EquipmentType {
    Accessory,
    Armour,
    Legs,
    Feet,
    Head,
}
