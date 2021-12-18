use bevy::prelude::Commands;

mod spawn_camera;
mod spawn_basket;

pub use spawn_camera::*;
pub use spawn_basket::*;

pub struct SpawnAppleCooldown(pub f32);

pub fn setup(mut commands: Commands) {
    commands.insert_resource(SpawnAppleCooldown(0.0));
}