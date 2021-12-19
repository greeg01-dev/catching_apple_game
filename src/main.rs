use bevy::{prelude::{App, DefaultPlugins, ClearColor, Color, IntoSystem}, window::WindowDescriptor};

mod system;

fn main() {
   App::build()
        .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 500.0,
            title: String::from("Catching Apple Game"),
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_startup_system(system::setup::setup.system())
        .add_startup_system(system::setup::spawn_camera.system())
        .add_startup_system(system::setup::spawn_basket.system())
        .add_system(system::spawn_apple.system())
        .add_system(system::move_basket.system())
        .add_system(system::move_apple.system())
        .add_system(system::catch_apple.system())
        .run();
}