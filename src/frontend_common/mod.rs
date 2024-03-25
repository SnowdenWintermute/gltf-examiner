use crate::bevy_app::modular_character_plugin::CharacterId;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharacterPartCategories {
    Head,
    Torso,
    Leg,
    Weapon,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PartsByName {
    pub heads: HashSet<String>,
    pub torsos: HashSet<String>,
    pub legs: HashSet<String>,
    pub weapons: HashSet<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CharacterPartSelection {
    pub character_id: CharacterId,
    pub name: String,
    pub category: CharacterPartCategories,
}
