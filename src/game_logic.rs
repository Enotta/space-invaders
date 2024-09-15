use bevy::prelude::*;

use crate::building::Building;

pub fn check_loss(
    mut commands: Commands,
    buildings: Query<Entity, With<Building>>,
    entities: Query<Entity, (Without<Camera>, Without<Window>)>
) {
    if buildings.iter().len() == 0 {
        commands.spawn(TextBundle {
            style: Style {
                left: Val::Percent(42.0),
                top: Val::Percent(50.0),
                ..default()
            },
            text: Text::from_section(
                "Game Over!",
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..default()
                }
            ),
            ..default()
        });

        entities.iter().for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        });
    }
}