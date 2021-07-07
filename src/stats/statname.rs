#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[allow(dead_code)]
/// Represents all possible stats present in the game. This only
/// gives the name of the stat, which can be used to index into
/// the Stats struct to get its corresponding value.
pub enum StatName {
    HealthPoints,
    SkillPoints,
    Defense,
    SpecialDefense,
    Attack,
    SpecialAttack,
    Speed,
    Evasion,
    Friendship,
}
