use bevy::prelude::*;

use crate::{LOGIC_HEIGHT, LOGIC_WIDTH};

/// Best score among current session games' scores
#[derive(Resource)]
pub struct BestScore(pub usize);

/// Current game score
#[derive(Resource)]
pub struct CurrentScore(pub usize);

/// Load best & current score resources
pub fn load_scores(
    mut commands: Commands
) {
    commands.insert_resource(CurrentScore(0));
    commands.insert_resource(BestScore(0));
}

impl CurrentScore {
    /// Spawn text with current score
    pub fn spawn(
        mut commands: Commands
    ) {
        commands.spawn(Text2dBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, LOGIC_HEIGHT / 2.45, 0.0),
                    ..default()
                },
                text: Text::from_section(
                    "Score: 0",
                    TextStyle {
                        font_size: 90.0,
                        color: Color::WHITE,
                        ..default()
                    }
                ),
                ..default()
            }
        );
    }

    /// Update score on the
    pub fn display(
        mut texts: Query<(&mut Text, &mut Transform)>,
        cur_score: Res<Self>,
        windows: Query<&mut Window>
    ) {
        let window = windows.single();

        for (mut text, mut transform) in texts.iter_mut() {
            transform.scale = Vec3::new( (window.width() as f32) / LOGIC_WIDTH, (window.height() as f32) / LOGIC_HEIGHT, 1.0);

            if text.sections[0].value.starts_with("Score: ") {
                text.sections[0].value = "Score: ".to_owned() + &cur_score.0.to_string();
            }
        }
    }
}
