use bevy::{prelude::{App, DefaultPlugins, ClearColor, Color}, window::WindowDescriptor};

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
        .run();
}