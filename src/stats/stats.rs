use crate::stats::{Stat, StatGrowth, StatKind, StatName};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
/// Represents the stats of an entity. Certain restrictions are placed upon
/// this struct, such that Vec<Stats>.len() == StatNameVariants. It contains
/// a list of all valid stats that an entity can have.
pub struct Stats {
    /// The list of valid stats.
    pub stats: Vec<Stat>,
}

impl Stats {
    #[allow(dead_code)]
    pub fn new(stats: Vec<Stat>) -> Self {
        Self { stats }
    }
}

impl Default for Stats {
    fn default() -> Self {
        use self::{StatKind::*, StatName::*};
        Self {
            stats: vec![
                Stat::new(HealthPoints, Depletable(0, 0), StatGrowth::Slow),
                Stat::new(SkillPoints, Depletable(0, 0), StatGrowth::Slow),
                Stat::new(Defense, Static(0), StatGrowth::Slow),
                Stat::new(SpecialDefense, Static(0), StatGrowth::Slow),
                Stat::new(Attack, Static(0), StatGrowth::Slow),
                Stat::new(SpecialAttack, Static(0), StatGrowth::Slow),
                Stat::new(Speed, Static(0), StatGrowth::Slow),
                Stat::new(Evasion, Static(0), StatGrowth::Slow),
                Stat::new(Friendship, Static(0), StatGrowth::Slow),
            ],
        }
    }
}

impl Index<StatName> for Stats {
    type Output = Stat;

    fn index(&self, index: StatName) -> &Self::Output {
        match self.stats.iter().find(|&stat| stat.name == index) {
            Some(value) => value,
            None => panic!("Invalid Stat {:#?}", index),
        }
    }
}

impl IndexMut<StatName> for Stats {
    fn index_mut(&mut self, index: StatName) -> &mut Self::Output {
        match self.stats.iter_mut().find(|stat| stat.name == index) {
            Some(value) => value,
            None => panic!("Invalid Stat {:#?}", index),
        }
    }
}
