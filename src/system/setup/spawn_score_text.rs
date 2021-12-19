use bevy::{prelude::{Commands, Res, AssetServer, VerticalAlign, HorizontalAlign, Transform, Color}, text::{Text2dBundle, Text, TextSection, TextStyle, TextAlignment}, math::Vec3};

pub struct ScoreText;

pub fn spawn_score_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: String::from("0"),
                        style: TextStyle {
                            font: asset_server.load("font/square.ttf"),
                            font_size: 100.0,
                            color: Color::BLACK
                        }
                    }
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Bottom,
                    horizontal: HorizontalAlign::Left
                }
            },
            transform: Transform {
                translation: Vec3::new(370.0, 220.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreText);
}