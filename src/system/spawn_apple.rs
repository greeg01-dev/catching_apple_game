use bevy::{prelude::{Commands, SpriteBundle, AssetServer, Res, Assets, ResMut, Transform}, sprite::ColorMaterial, math::Vec3, core::Time};
use rand::Rng;

use super::setup::SpawnAppleCooldown;

pub struct Apple;

pub fn spawn_apple(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut cooldown: ResMut<SpawnAppleCooldown>
) {
    // if the timer is finished
    if cooldown.0 <= 0.0 {
        // spawn apple sprite
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(asset_server.load("sprite/apple.png").into()),
                transform: Transform {
                    translation: Vec3::new(rand::thread_rng().gen_range(-400.0..=400.0), 250.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Apple);

            cooldown.0 = rand::thread_rng().gen_range(1.0..=3.0); // make the cooldown to random value from 3.0 to 5.0
    }
    else {
        cooldown.0 -= time.delta_seconds(); // update the timer
    }
}