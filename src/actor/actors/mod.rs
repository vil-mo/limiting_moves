use std::marker::PhantomData;
use bevy::prelude::*;
use self::player::{Player, spawn_player};

pub mod player;

pub struct ActorsPlugin;

impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnActor<Player>>();
        app.add_systems(Update, spawn_player);
    }
}


pub trait Actor {}

#[derive(Event, Default)]
pub struct SpawnActor<A: Actor> {
    pub pos: Vec2,
    pub _ph: PhantomData<A>
}
