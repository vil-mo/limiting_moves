use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_xpbd_2d::prelude::*;
use crate::actor::actions::movement::MovementAction;
use crate::actor::actors::{Actor, SpawnActor};
use crate::actor::behaviours::player::PlayerBehaviour;
use crate::mechanics::energy::Energy;


const PLAYER_SPEED: f32 = 700.0;
const PLAYER_ACCELERATION: f32 = 0.30;
const PLAYER_RADIUS: f32 = 20.0;

#[derive(Default)]
pub struct Player;

impl Actor for Player {}

pub(super) fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, mut events: EventReader<SpawnActor<Player>>) {
    for event in events.read() {
        commands.spawn((
            MaterialMesh2dBundle {
                transform: Transform::from_translation(Vec3::from((event.pos, 0.0))),
                mesh: meshes.add(shape::Circle::new(PLAYER_RADIUS).into()).into(),
                material: materials.add(ColorMaterial::from(Color::rgb(0.0, 1.0, 0.0))),

                ..default()
            },

            RigidBody::Dynamic,
            Collider::ball(PLAYER_RADIUS),

            PlayerBehaviour,
            MovementAction::new(PLAYER_SPEED, PLAYER_ACCELERATION),
            Energy {
                value: 10.0,
                accumulation_rate: 1.0,
                spending_rate: 2.0,
            }
        ));
    }
}
