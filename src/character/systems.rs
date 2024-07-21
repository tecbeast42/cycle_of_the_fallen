use bevy::{color::palettes::tailwind, prelude::*};

use crate::game::{CurrentLevel, GameState};

use super::prelude::CharacterSelectionButton;

pub fn spawn_character_selection(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_level: Res<CurrentLevel>,
) {
    info!("Spawn CharacterSelection");
    commands
        .spawn((
            StateScoped(GameState::CharacterSelection),
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
                background_color: tailwind::PURPLE_900.into(),
                ..default()
            },
        ))
        .with_children(|wrapper| {
            wrapper.spawn(TextBundle::from_section(
                "Character Selection",
                TextStyle {
                    font: asset_server.load("Kalam-Light.ttf"),
                    font_size: 80.0,
                    ..default()
                },
            ));
            let characters =
                current_level.0.clone().map(|l| l.characters).expect(
                    "Expected a current level to be set in system spawn_character_selection",
                );
            for character in characters.iter() {
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
                        CharacterSelectionButton(character.clone()),
                    ))
                    .with_children(|selector| {
                        selector.spawn(TextBundle::from_section(
                            format!("{character:?}"),
                            TextStyle {
                                font: asset_server.load("Kalam-Bold.ttf"),
                                font_size: 32.0,
                                ..default()
                            },
                        ));
                    });
            }
        });
}

pub fn interaction_on_character_selection_buttons(
    mut query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &CharacterSelectionButton,
        ),
        Changed<Interaction>,
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color, _character_selection_button) in query.iter_mut() {
        *background_color = match *interaction {
            Interaction::Pressed => {
                game_state.set(GameState::Play);
                // TODO now do something with the char choosen
                tailwind::LIME_300.into()
            }
            Interaction::Hovered => tailwind::LIME_500.into(),
            Interaction::None => tailwind::LIME_800.into(),
        }
    }
}
