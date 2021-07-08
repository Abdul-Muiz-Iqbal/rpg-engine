#![allow(dead_code)]

use crate::stats::{Stat, StatKind, Stats};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
/// Contains the data a single level stores. This includes the current level,
/// the current experience an enitity has, and how much experience is required
/// for the next level.
pub struct LevelData {
    /// The current level. Must be a value between 1 and 99 (inclusive)
    level: u8,
    /// The current experience an entity has. Must always be < experience_for_next_level
    current_experience: usize,
    /// The experience required for the next level. This is the maximum value required,
    /// and does not decrease as the current_experience increases. Rather, it is the threshold
    /// for levelling up.
    experience_for_next_level: usize,
}

impl LevelData {
    /// The base experience is the multiplying factor that decides how large the
    /// amount of experience will be.
    const BASE_XP: f64 = 1000.0;
    /// The exponent determines the gap between the levels. The larger the exponent,
    /// the larger the gap.
    const EXPONENT: f64 = 1.5;

    /// Create an instance of LevelData given a level and current_experience.
    /// experience_for_next_level is automatically calculated.
    pub fn new(level: u8, current_experience: usize) -> Self {
        let experience_for_next_level = Self::experience_for_level(level);
        match (level >= 1 && level <= 99, current_experience < experience_for_next_level) {
            (true, true) => Self { level, current_experience, experience_for_next_level },
            (false, true) => panic!("Level must be between 1 and 99"),
            (true, false) => panic!("Current experience must be less than experience required for the next level"),
            (false, false) => panic!("Level must be between 1 and 99 and Current experience must be less than experience required for the next level")
        }
    }

    /// Sets the current experience. If the experience is greater than or equal to that required
    /// for the next level, increment the level, store the excess and recalculate the experience
    /// required for the next level.
    pub fn set_current_experience(&mut self, experience: usize) {
        if experience < self.experience_for_next_level {
            self.current_experience = experience;
        } else {
            // Calculate excess experience and store it, increment the level,
            // and recalculate the experience required for the next level.
            self.set_current_experience(self.current_experience - self.experience_for_next_level);
            self.level += 1;
            self.experience_for_next_level = Self::experience_for_level(self.level);
        }
    }

    /// Calculates the amount of experience required to level up given the current level.
    /// The formula used is as follows:
    /// floor(baseXP * (currentLevel ^ exponent))
    /// This creates a simple curve that gets steeper as we increase the level.
    pub fn experience_for_level(level: u8) -> usize {
        let level = level as f64;
        (Self::BASE_XP * ((level as f64).powf(Self::EXPONENT))).floor() as _
    }

    /// Creates a new stat table that represents the increase in stats after a level up.
    pub fn create_stat_table(&self) -> Stats {
        let mut stats = Stats::default();
        stats.stats.iter_mut().for_each(|stat| {
            let kind = {
                let v = stat.base() + (stat.stat_growth.roll() as usize);
                match stat.kind {
                    StatKind::Depletable(min, _) => StatKind::Depletable(min, v),
                    StatKind::Static(_) => StatKind::Static(v),
                }
            };
            *stat = Stat::new(stat.name, kind, stat.stat_growth);
        });

        stats
    }
}
