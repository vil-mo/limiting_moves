use bevy::prelude::*;

use crate::actor::actions::movement::MovementAction;

#[derive(Component)]
pub struct Energy {
    pub value: f32,
    /// Gain `accumulation_rate` energy per one seconds when standing still
    pub accumulation_rate: f32,
    /// Lose `spending_rate` energy per one seconds when moving
    pub spending_rate: f32,
}


pub(super) fn update_energy(mut query: Query<(&mut Energy, &mut MovementAction)>, time: Res<Time<Virtual>>) {
    for (mut energy, mut movement) in query.iter_mut() {
        if movement.direction == Vec2::ZERO {
            energy.value += energy.accumulation_rate * time.delta_seconds();
        } else {
            energy.value -= energy.spending_rate * time.delta_seconds();
            energy.value = energy.value.max(0.0);
        }

        if energy.value <= 0.0 {
            movement.direction = movement.direction * 0.1;
        }
    }
}



#[derive(Component)]
pub(super) struct EnergyText;

pub(super) fn update_energy_text(
    character_query: Query<&Energy>,
    mut text_query: Query<&mut Text, With<EnergyText>>,
) {
    let Ok(energy) = character_query.get_single() else {return};

    text_query.single_mut().sections[0].value = format!(
        "Energy: {:.4}", energy.value
    );
}
