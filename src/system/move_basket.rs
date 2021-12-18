use bevy::{prelude::{Query, Transform, With, Res, KeyCode}, input::Input, core::Time};

use super::setup::Basket;

pub fn move_basket(
    mut basket_query: Query<&mut Transform, With<Basket>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let mut basket_transform = basket_query.single_mut().unwrap();

    // move basket when Left Button is pressed
    if keyboard_input.pressed(KeyCode::Left) {
        basket_transform.translation.x = (basket_transform.translation.x - time.delta_seconds() * 350.0).max(-360.0);
    }
    // move basket when Right Button is pressed
    if keyboard_input.pressed(KeyCode::Right) {
        basket_transform.translation.x = (basket_transform.translation.x + time.delta_seconds() * 350.0).min(360.0);
    }
}