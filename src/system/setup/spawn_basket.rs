use bevy::{prelude::{Commands, SpriteBundle, AssetServer, Res, Assets, ResMut, Transform}, sprite::ColorMaterial, math::Vec3};

pub struct Basket;

pub fn spawn_basket(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // spawn basket sprite
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(asset_server.load("sprite/basket.png").into()),
            transform: Transform {
                translation: Vec3::new(0.0, -224.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Basket);
}