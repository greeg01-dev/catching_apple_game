use bevy::{prelude::{Query, With, Res}, text::Text};

use super::setup::{ScoreText, Score};

pub fn update_score_text(
    mut score_text_query: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>
) {
    let mut score_text = score_text_query.single_mut().unwrap();
    if score_text.sections[0].value != format!("{}", score.0) {
        score_text.sections[0].value = format!("{}", score.0);
    }
}