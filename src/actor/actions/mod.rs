pub mod movement;
mod emit_projectile;

use crate::actor::behaviours::BehaviourSet;
use bevy::prelude::*;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, ActionSet.after(BehaviourSet));

        app.add_systems(Update, (movement::apply_movement,).in_set(ActionSet));
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct ActionSet;
