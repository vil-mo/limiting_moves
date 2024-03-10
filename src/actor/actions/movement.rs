use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

#[derive(Component)]
pub struct MovementAction {
    pub direction: Vec2,
    pub max_speed: f32,
    pub acceleration: f32,
}


impl MovementAction {
    pub fn new(max_speed: f32, acceleration: f32) -> Self {
        Self {
            direction: Vec2::ZERO,
            max_speed,
            acceleration,
        }
    }
}

pub(super) fn apply_movement(mut query: Query<(&mut LinearVelocity, &MovementAction)>) {
    for (mut velocity, movement) in query.iter_mut() {
        let target = movement.direction * movement.max_speed;

        velocity.0 = velocity.0.lerp(target, movement.acceleration);
        
    }
}
