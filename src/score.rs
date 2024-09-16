use bevy::prelude::*;

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
        commands.spawn(TextBundle {
                style: Style {
                    top: Val::Percent(5.0),
                    left: Val::Percent(42.0),
                    ..default()
                },
                text: Text::from_section(
                    "Score: 0",
                    TextStyle {
                        font_size: 60.0,
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
        mut texts: Query<&mut Text>,
        cur_score: Res<Self>
    ) {
        for mut text in texts.iter_mut() {
            if text.sections[0].value.starts_with("Score: ") {
                text.sections[0].value = "Score: ".to_owned() + &cur_score.0.to_string();
            }
        }
    }
}
