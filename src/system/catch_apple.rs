use bevy::{prelude::{Query, Transform, With, Entity, Commands, ResMut}, sprite::collide_aabb::collide, math::Vec2};

use super::{setup::{Basket, Score}, Apple};

pub fn catch_apple(
    basket_query: Query<&Transform, With<Basket>>,
    apple_query: Query<(&Transform, Entity), With<Apple>>,
    mut commands: Commands,
    mut score: ResMut<Score>
) {
    for (apple_transform, apple_entity) in apple_query.iter() {
        let basket_transform = basket_query.single().unwrap();

        // when the apple touches the basket
        if collide(
            apple_transform.translation,
            Vec2::new(40.0, 40.0),
            basket_transform.translation,
            Vec2::new(80.0, 52.0)
        ).is_some() {
            score.0 += 1; // add score
            commands.entity(apple_entity).despawn(); // despawn the apple entity
        }
    }
}