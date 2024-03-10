mod actor;
mod input;
mod mechanics;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::input::InputPlugin;

use crate::actor::actions::ActionsPlugin;
use crate::actor::behaviours::BehaviourPlugin;
use crate::actor::actors::{ActorsPlugin, SpawnActor};
use crate::actor::actors::player::Player;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Gravity(Vec2::ZERO))

        .add_plugins((
            PhysicsPlugins::default(),
            InputPlugin,
            ActorsPlugin,
            ActionsPlugin,
            BehaviourPlugin,
        ))


        .add_systems(Startup, setup)

        .run()
}


fn setup(mut commands: Commands, mut spawn: EventWriter<SpawnActor<Player>>) {
    commands.spawn(Camera2dBundle::default());

    spawn.send(SpawnActor {
        pos: Vec2 { x: 100.0, y: 100.0 },
        _ph: std::marker::PhantomData,
    });
}
