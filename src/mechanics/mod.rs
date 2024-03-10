pub mod energy;
pub mod projectile;

use bevy::prelude::*;
use crate::actor::actions::ActionSet;
use crate::actor::behaviours::BehaviourSet;
use energy::EnergyText;

pub struct MechanicsPlugin;

impl Plugin for MechanicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(projectile::ProjectilePlugin)

            .add_systems(Startup, startup)

            .add_systems(Update, (
                energy::update_energy.after(BehaviourSet).before(ActionSet),
                energy::update_energy_text,
            ));
    }
}



fn startup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 16.0,
                ..default()
            },
        )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            }),
        EnergyText,
    ));

}
