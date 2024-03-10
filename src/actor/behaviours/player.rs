use crate::actor::actions::movement::MovementAction;
use crate::input::{InputConfig, InputMap};
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerBehaviour;

pub(super) fn player_movement(
    mut query: Query<&mut MovementAction, With<PlayerBehaviour>>,
    input_config: Res<InputConfig>,
    input: Res<Input<ScanCode>>,
) {
    let left = input.any_pressed(input_config.get_scan_codes(InputMap::Left));
    let right = input.any_pressed(input_config.get_scan_codes(InputMap::Right));
    let down = input.any_pressed(input_config.get_scan_codes(InputMap::Down));
    let up = input.any_pressed(input_config.get_scan_codes(InputMap::Up));
    
    let input_vec: Vec2 = Vec2 {
        x: right as i32 as f32 - left as i32 as f32,
        y: up as i32 as f32 - down as i32 as f32,
    }.normalize_or_zero();

    for mut movement in query.iter_mut() {
        movement.direction = input_vec;
    }
}
