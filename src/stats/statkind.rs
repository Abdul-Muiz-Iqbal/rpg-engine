#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[allow(dead_code)]
/// Represents the type of a stat. Some stats such as HP are
/// depletable during battle. It can have a max value of 100 but
/// have a current value of 15. On the other hand, stats can be
/// static, such that they remain the same throughout battle,
/// as long as modifiers aren't in effect.
pub enum StatKind {
    /// Variant representing a stat that can change during battles without modifiers.
    /// It holds the current_value and the maximum_value of that stat. Eg: Depletable(0, 100)
    Depletable(usize, usize),
    /// Variant representing a stat whose value cannot change during battles unless modifiers
    /// are used. It holds its value that can't be depleted.
    Static(usize),
}
