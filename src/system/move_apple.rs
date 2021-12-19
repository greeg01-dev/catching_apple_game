use bevy::{prelude::{Query, Transform, With, Res, Entity, Commands, ResMut}, core::Time};

use super::{Apple, setup::Score};

pub fn move_apple(
    mut apple_query: Query<(&mut Transform, Entity), With<Apple>>,
    mut commands: Commands,
    time: Res<Time>,
    mut score: ResMut<Score>
) {
    for (mut apple_transform, apple_entity) in apple_query.iter_mut() {
        // make the apples be fallen
        apple_transform.translation.y -= time.delta_seconds() * 400.0;
        // despawn the apple
        if apple_transform.translation.y <= -270.0 {
            commands.entity(apple_entity).despawn();
            if score.0 > 0 {
                score.0 -= 1; // subtract score
            }
        }
    }
}