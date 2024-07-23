use bevy::{color::palettes::tailwind, prelude::*};

use crate::game::{CurrentLevel, GameState, Levels};

use super::prelude::*;

pub fn spawn_level_selection(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    levels: Res<Levels>,
) {
    info!("Spawn LevelSelection");
    commands
        .spawn((
            StateScoped(GameState::LevelSelection),
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(50.0)),
                    position_type: PositionType::Absolute,
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_wrap: FlexWrap::Wrap,
                    column_gap: Val::Px(20.0),
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: tailwind::STONE_950.into(),
                ..default()
            },
        ))
        .with_children(|wrapper| {
            wrapper.spawn(TextBundle::from_section(
                "Level Selection",
                TextStyle {
                    font: asset_server.load("Kalam-Light.ttf"),
                    font_size: 80.0,
                    ..default()
                },
            ));
            for level in levels.iter() {
                wrapper
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(180.0),
                                height: Val::Px(180.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        },
                        LevelSelectionButton::from(level),
                    ))
                    .with_children(|selector| {
                        selector.spawn(TextBundle::from_section(
                            level.id.to_string(),
                            TextStyle {
                                font: asset_server.load("Kalam-Bold.ttf"),
                                font_size: 32.0,
                                ..default()
                            },
                        ));
                        let score: String;
                        if let Some(existing_score) = level.cycles {
                            score = existing_score.to_string();
                        } else {
                            score = "∞".to_string();
                        };
                        selector.spawn(TextBundle::from_section(
                            score,
                            TextStyle {
                                font: asset_server.load("Kalam-Regular.ttf"),
                                font_size: 70.0,
                                ..default()
                            },
                        ));
                    });
            }
        });
}

pub fn interaction_on_level_selection_buttons(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &LevelSelectionButton),
        Changed<Interaction>,
    >,
    mut game_state: ResMut<NextState<GameState>>,
    mut current_level: ResMut<CurrentLevel>,
    levels: Res<Levels>,
) {
    for (interaction, mut background_color, level_selection_button) in query.iter_mut() {
        *background_color = match (*interaction, level_selection_button.unlocked) {
            (Interaction::Pressed, true) => {
                game_state.set(GameState::CharacterSelection);
                current_level.0 = Some(levels.id(level_selection_button.level));
                tailwind::LIME_300.into()
            }
            (Interaction::Hovered, true) => tailwind::LIME_500.into(),
            (Interaction::None, true) => tailwind::LIME_800.into(),
            (_, false) => tailwind::STONE_700.into(),
        }
    }
}
