use bevy::prelude::*;
use crate::mechanics::projectile::Projectile;

pub struct PlayerBullet;

impl Projectile for PlayerBullet {
    fn components(self) -> impl Bundle {
        TransformBundle {..default()}
    }
}