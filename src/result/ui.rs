use crate::FONT_PATH;
use crate::playing::score::Score;
use crate::state::GameState;
use bevy::prelude::*;

pub struct ResultUiPlugin;

impl Plugin for ResultUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Result), spawn_result_ui)
            .add_systems(
                Update,
                update_continue_button.run_if(in_state(GameState::Result)),
            );
    }
}

#[derive(Component)]
struct ContinueButton;

fn continue_button_bundle(asset_server: &AssetServer) -> impl Bundle {
    (
        Button,
        ContinueButton,
        Node {
            width: percent(20),
            height: percent(10),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor::default(),
        children![(
            Text::new("Continue"),
            TextFont {
                font: asset_server.load(FONT_PATH),
                font_size: 40.0,
                ..default()
            },
            TextLayout::new_with_justify(Justify::Center),
            TextColor(Color::srgb(0.0, 0.0, 0.0)),
        )],
    )
}

#[derive(Component)]
struct ResultTextUi;

fn result_text_bundle(asset_server: &AssetServer, score: &Score) -> impl Bundle {
    (
        ResultTextUi,
        Node {
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Node::default(),
                children![
                    (
                        Text::new("Time: "),
                        TextFont {
                            font: asset_server.load(FONT_PATH),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ),
                    (
                        Text::new(format!("{}", score.time)),
                        TextFont {
                            font: asset_server.load(FONT_PATH),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.0, 0.9, 1.0)),
                    ),
                ]
            ),
            (
                Node::default(),
                children![
                    (
                        Text::new("Death: "),
                        TextFont {
                            font: asset_server.load(FONT_PATH),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ),
                    (
                        Text::new(format!("{}", score.death)),
                        TextFont {
                            font: asset_server.load(FONT_PATH),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.3, 0.0)),
                    ),
                ]
            ),
        ],
    )
}

fn result_canvas_bundle() -> impl Bundle {
    (
        DespawnOnExit(GameState::Result),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
    )
}

fn spawn_result_ui(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    let canvas = commands.spawn(result_canvas_bundle()).id();
    let text = commands
        .spawn(result_text_bundle(&asset_server, &score))
        .id();
    let continue_button = commands.spawn(continue_button_bundle(&asset_server)).id();
    commands
        .entity(canvas)
        .add_children(&[text, continue_button]);
}

type ContinueButtonInputs = (Changed<Interaction>, With<ContinueButton>);

fn update_continue_button(
    mut game_state: ResMut<NextState<GameState>>,
    mut query: Query<(&Interaction, &mut BackgroundColor), ContinueButtonInputs>,
) {
    for (interaction, mut background_color) in &mut query {
        match interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(1.0, 1.0, 0.2);
                game_state.set(GameState::CourseSelection);
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgb(0.8, 0.8, 0.2);
            }
            Interaction::None => {
                background_color.0 = Color::srgb(0.5, 0.5, 0.2);
            }
        }
    }
}
