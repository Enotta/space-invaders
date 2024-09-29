use bevy::prelude::*;

use crate::{LOGIC_HEIGHT,
            border::{Border, BorderTexture}, 
            building::{Building, BuildingTexture}, 
            score::{self, BestScore, CurrentScore}, 
            starship::{Starship, StarshipTexture}};

/// Game over screen if all buildings are destroyed. Restart on ENTER
pub fn run(
    // Game over screen
    mut commands: Commands,
    buildings: Query<Entity, With<Building>>,
    entities: Query<Entity, (Without<Camera>, Without<Window>)>,
    texts: Query<(Entity, &Text)>,
    mut cur_score: ResMut<CurrentScore>,
    mut best_score: ResMut<BestScore>,

    // Restart game
    keyboard_input: Res<ButtonInput<KeyCode>>,
    border_commands: Commands,
    starship_commands: Commands,
    building_commands: Commands,
    score_commands: Commands,
    border_texture: Res<BorderTexture>,
    starship_texture: Res<StarshipTexture>,
    building_texture: Res<BuildingTexture>
) {
    if buildings.iter().len() == 0 {
        if cur_score.0 > best_score.0 {
            best_score.0 = cur_score.0;
        }

        commands.spawn(Text2dBundle {
            transform: Transform {
                translation: Vec3::new(0.0, LOGIC_HEIGHT / 2.45, 0.0),
                ..default()
            },
            text: Text::from_section(
                "Game Over! \nYour score: ".to_owned() + &cur_score.0.to_string() + "\nYour best score: " + &best_score.0.to_string() + "\n\nPress ENTER to restart",
                TextStyle {
                            font_size: 60.0,
                            color: Color::WHITE,
                            ..default()
                       }
            ).with_justify(JustifyText::Center),
            ..default()
        });

        entities.iter().for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        });

        if keyboard_input.pressed(KeyCode::Enter) {
            cur_score.0 = 0;
            Border::load(border_commands, border_texture);
            Starship::spawn(starship_commands, starship_texture);
            Building::spawn(building_commands, building_texture);
            score::CurrentScore::spawn(score_commands);
        }
    }
    else {
        for (text_bundle, text) in texts.iter() {
            if text.sections[0].value.starts_with("Game Over!") {
                commands.entity(text_bundle).despawn();
            }
        }
    }
}