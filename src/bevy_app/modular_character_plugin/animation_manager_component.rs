use super::CharacterId;
use bevy::math::u64;
use bevy::prelude::*;
use std::collections::HashMap;

pub type Timestamp = u64;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum ActionSequenceStates {
    ApproachingTarget,
    Swinging,
    Returning,
    Recentering,
    HitRecovery,
}

#[derive(Component, Default)]
pub struct AnimationManagerComponent {
    pub active_states: HashMap<ActionSequenceStates, Option<Timestamp>>,
    pub destination: Option<Transform>,
    pub last_location: Option<Transform>,
    pub target_rotation: Option<Quat>,
    pub last_rotation: Option<Quat>,
    pub current_targets: Option<Vec<CharacterId>>,
}
