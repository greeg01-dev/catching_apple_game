use bevy::prelude::{Commands, OrthographicCameraBundle};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}