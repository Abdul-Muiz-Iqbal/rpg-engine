#![allow(dead_code)]
use super::Entity;

#[derive(Debug, Default)]
/// Represents the current state of the party. Party members are divided into the
/// active party and the reserved party. Only active_party members are allowed in
/// battles, while the reserved party members can be switched with active members
/// outside of battle.
pub struct Party {
    /// The party members who are allowed in battle. MUST be <= 4
    active_party: Vec<Entity>,
    /// The party members who are `reserved` and do not participate in battles unless
    /// switched out.
    reserved_party: Vec<Entity>,
}

impl Party {
    /// Create a new instance of a Party, given the active party and reserved party.
    /// Make sure that the active party length is less than or equal to four.
    pub fn new(active_party: Vec<Entity>, reserved_party: Vec<Entity>) -> Self {
        if active_party.len() <= 4 {
            Self {
                active_party,
                reserved_party,
            }
        } else {
            panic!("There can only be four active party members.");
        }
    }

    /// Adds a new party member into the active party as long as the active party
    /// is smaller than four.
    pub fn add_active(&mut self, party_member: Entity) {
        if self.active_party.len() >= 4 {
            panic!("There can only be four active party members.");
        }
        self.active_party.push(party_member);
    }

    /// Adds a new party member into the reserved party.
    pub fn add_reserved(&mut self, party_member: Entity) {
        self.reserved_party.push(party_member);
    }

    /// Removes the party member from a party given its id. Silently fails
    /// if there are no party members with that id.
    pub fn remove_by_id(&mut self, party: Vec<Entity>, id: usize) -> Vec<Entity> {
        party
            .into_iter()
            .filter(|entity| entity.id != id)
            .collect::<Vec<Entity>>()
    }

    /// Removes the party member from the active party.
    pub fn remove_active(&mut self, entity: Entity) {
        self.active_party = self.remove_by_id(self.active_party.clone(), entity.id);
    }

    /// Removes the party member from the reserved party.
    pub fn remove_reserved(&mut self, entity: Entity) {
        self.reserved_party = self.remove_by_id(self.reserved_party.clone(), entity.id);
    }

    /// Switches an active party member with a reserved party member.
    pub fn switch_members(&mut self, active_member: Entity, reserved_member: Entity) {
        self.remove_active(active_member.clone());
        self.add_reserved(active_member);
        self.add_active(reserved_member.clone());
        self.remove_reserved(reserved_member);
    }
}
