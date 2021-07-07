#![allow(dead_code)]

use crate::stats::{Modifier, Modifiers, StatKind, StatName};

use super::StatGrowth;

#[derive(Debug, PartialEq, PartialOrd)]
#[allow(dead_code)]
/// Represents a single stat of character. It contains the name, kind,
/// value and modifiers present on that stat. There are numerous ways
/// to modify stats, and this struct provides the necessary methods for
/// easy application and removal of certain stats. Modifiers are stored
/// in stacks, meaning multiple modifiers are condensed into a single
/// modifier.
pub struct Stat {
    /// The ingame name of the stat.
    pub name: StatName,
    /// The kind of the stat. This also indirectly holds the value of the stat.
    pub kind: StatKind,
    /// A list of modifiers that are applied to the stat. Stored as stacks.
    pub modifiers: Modifiers,
    /// Determines how fast the value of the stat increases on level up.
    pub stat_growth: StatGrowth,
}

impl Stat {
    #[allow(dead_code)]
    /// Create a base stat struct given its name, kind and modifiers present.
    pub fn new(name: StatName, kind: StatKind, stat_growth: StatGrowth) -> Self {
        Self {
            name,
            kind,
            modifiers: Modifiers::default(),
            stat_growth,
        }
    }

    /// Get the base value of this stat. This is the value without any modifiers applied
    /// to it. It is returned as a usize instead of an f64 because base values cannot
    /// be floating point numbers ever.
    pub fn base(&self) -> usize {
        match self.kind {
            StatKind::Depletable(min, _) => min,
            StatKind::Static(value) => value,
        }
    }

    /// Sets the base value of this stat. It performs some validation checks on value
    /// to be set, however if the validation fails, then this function will fail silently.
    /// Stat Name: Min-Max -- Stats By Endgame
    /// Hp: 0-999 -- 500
    /// Sp: 0-999 -- 250
    /// Friendship 0-15 -- 6-15
    /// Other Stats: 0-999 -- 150
    #[allow(dead_code)]
    pub fn set_base(&mut self, value: usize) {
        let condition = if self.name == StatName::Friendship {
            value > 0 && value <= 15 // value to set is between 0 and 15 if the stat is Friendship.
        } else {
            value > 0 && value <= 999 // value to set is between 0 and 999 for every other stat.
        };
        self.kind = match self.kind {
            StatKind::Depletable(_, max) => {
                if condition {
                    StatKind::Depletable(value, max)
                } else {
                    return;
                }
            }
            StatKind::Static(_) => {
                if condition {
                    StatKind::Static(value)
                } else {
                    return;
                }
            }
        };
    }

    /// Get the calculated value of this stat. The calculation takes all additive
    /// modifiers and firstly adds them to the base stat. Then secondly it multiplies
    /// the multiplicative modifiers. This ensures that every stat buff/debuff is
    /// going to be meaningful. Doing it in the reverse order will reduce the effect.
    pub fn value(&self) -> f64 {
        // Get the current value of the stat regardless of its kind.
        let mut current_value = self.base() as f64;

        // Additive modifiers are added first, and multiplicated modifiers are
        // multiplied second. This means that buffs/debuffs will be more devastating.
        match (self.modifiers.additive, self.modifiers.multiplicative) {
            (Modifier::Plus(x), Modifier::Mult(y)) => {
                current_value += x;
                current_value *= y;
            }
            _ => unreachable!(),
        }

        current_value
    }
}
