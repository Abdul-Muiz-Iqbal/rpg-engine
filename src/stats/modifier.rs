use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[allow(dead_code)]
///     Represents a buff or a debuff that is applied to a specific stat.
/// Buffs/Debuffs can be Additive, or Multiplicative and there can be no
/// buffs/debuffs applied at all.
///     These modifiers are used by certain items, skill, and equipment
/// in order to provide bonuses in and outside battle. For example the
/// Annoy skill would double your attack but halve your defense. To do
/// that, you would create Modifiers of Mult(1) on Attack and Mult(-0.5)
/// on defense.
pub enum Modifier {
    /// The additive variant. This adds its inner value to the stat value.
    Plus(f64),
    /// The multiplicative variant. This multiplies its inner value by the stat value
    /// and adds the rest to the stat value.
    Mult(f64),
    /// Represents the absence of any buff/debuff.
    None,
}

impl Add for Modifier {
    type Output = Modifier;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Plus(x), Self::Plus(y)) => Self::Plus(x + y),
            (Self::Mult(x), Self::Mult(y)) => Self::Mult(x * y),
            (Self::None, Self::None) => Self::None,
            _ => panic!("Modifiers can only be added to modifiers of the same variant!"),
        }
    }
}

impl AddAssign for Modifier {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Modifier {
    type Output = Modifier;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Plus(x), Self::Plus(y)) => Self::Plus(x - y),
            (Self::Mult(x), Self::Mult(y)) => Self::Mult(x / y),
            (Self::None, Self::None) => Self::None,
            _ => panic!("Modifiers can only be subtracted from modifiers of the same variant!"),
        }
    }
}

impl SubAssign for Modifier {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[allow(dead_code)]
/// A container type of a list of modifiers. It contains two fields each of type
/// Modifier, which is an enum. additive will always hold an Add variant, while
/// multiplicative will always hold the Mult variant. As such, in some instances,
/// optimization can occur as we know that the multiplicative field will _never_
/// be Add.
pub struct Modifiers {
    /// This field always contains the variant Add of Modifier.
    pub additive: Modifier,
    /// This field always contains the variant Mult of Modifier.
    pub multiplicative: Modifier,
}

impl Default for Modifiers {
    fn default() -> Self {
        Self {
            additive: Modifier::Plus(0.0),
            multiplicative: Modifier::Mult(1.0),
        }
    }
}

impl Modifiers {
    #[allow(dead_code)]
    pub fn new(additive: Modifier, multiplicative: Modifier) -> Self {
        Self {
            additive,
            multiplicative,
        }
    }

    /// Adds one modifier to this stat. Allows modifiers to be stacked
    /// onto each other. If you add Add(5) and Add(2), it becomes Add(7).
    #[allow(dead_code)]
    pub fn add(&mut self, modifier: Modifier) {
        match modifier {
            Modifier::Mult(_) => self.multiplicative += modifier,
            Modifier::Plus(_) => self.additive += modifier,
            Modifier::None => {}
        }
    }

    /// Removes one modifier to this stat. Allows modifiers to be stacked
    /// onto each other. If you remove Add(5) from Add(2), it becomes Add(-3).
    #[allow(dead_code)]
    pub fn remove(&mut self, modifier: Modifier) {
        match modifier {
            Modifier::Mult(_) => self.multiplicative -= modifier,
            Modifier::Plus(_) => self.additive -= modifier,
            Modifier::None => {}
        }
    }
}
