pub mod player_bullet;

use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EmitProjectile<player_bullet::PlayerBullet>>()
            .add_systems(Update, on_emit_projectile::<player_bullet::PlayerBullet>);
    }
}


pub trait Projectile: Send + Sync + 'static {
    fn components(self) -> impl Bundle;
}

#[derive(Event)]
pub struct EmitProjectile<T: Projectile>(T);

pub(super) fn on_emit_projectile<T: Projectile>(mut commands: Commands, mut event: EventReader<EmitProjectile<T>>) {
    for ep in event.read() {
        commands.spawn(ep.0.components());
    }
}
