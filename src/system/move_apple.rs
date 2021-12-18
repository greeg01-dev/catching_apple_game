use bevy::{prelude::{Query, Transform, With, Res, Entity, Commands}, core::Time};

use super::Apple;

pub fn move_apple(
    mut apple_query: Query<(&mut Transform, Entity), With<Apple>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (mut apple_transform, apple_entity) in apple_query.iter_mut() {
        // make the apples be fallen
        apple_transform.translation.y -= time.delta_seconds() * 400.0;
        // despawn the apple
        if apple_transform.translation.y <= -270.0 {
            commands.entity(apple_entity).despawn();
        }
    }
}