pub mod player;

use crate::actor::actions::ActionSet;
use bevy::prelude::*;

pub struct BehaviourPlugin;

impl Plugin for BehaviourPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, BehaviourSet.before(ActionSet));

        app.add_systems(
            Update,
            (player::player_movement).in_set(BehaviourSet),
        );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct BehaviourSet;
