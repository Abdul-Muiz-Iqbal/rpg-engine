#![allow(dead_code)]

use bracket_random::prelude::RandomNumberGenerator;
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// The global static instance of the random number generator. This generator from bracket-lib has
/// the features of using dice notation for its number generation, which make it perfect for stat growth.
static RNG: Lazy<Mutex<RandomNumberGenerator>> =
    Lazy::new(|| Mutex::new(RandomNumberGenerator::new()));

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
/// Represents the speeds at which a stat can increase on level up.
/// Each variant corresponds to a speed represented in dice notation:
/// Slow -> 1d2
/// Medium -> 1d3
/// Fast -> 2d2
/// The custom variant stores a string in dice notation that will be used for things
/// that are too widely different to be categorizable into small parts.
pub enum StatGrowth {
    Slow,
    Medium,
    Fast,
    /// Stores a string in dice notation.
    Custom(&'static str),
}

impl StatGrowth {
    /// Generates a random number according the the stat growth variant. This random number
    /// determines how much increase there is in the stat.
    pub fn roll(&self) -> i32 {
        match *self {
            // Note: If something explodes, it's probably the Mutex.
            Self::Fast => RNG.lock().unwrap().roll_str("3d2").unwrap(),
            Self::Medium => RNG.lock().unwrap().roll_str("1d3").unwrap(),
            Self::Slow => RNG.lock().unwrap().roll_str("1d2").unwrap(),
            Self::Custom(dice) => RNG.lock().unwrap().roll_str(dice).unwrap(),
        }
    }
}
