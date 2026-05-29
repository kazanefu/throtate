use crate::JpFont;
use crate::button::SizeUpButtonBundle;
use crate::keyboard_button::SelectableButton;
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

fn continue_button_bundle(font: &Handle<Font>) -> impl Bundle {
    (
        Button,
        ContinueButton,
        SelectableButton { id: 0 },
        SizeUpButtonBundle::new(1.1, 10.0),
        UiTransform::default(),
        Node {
            width: percent(40),
            height: percent(10),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor::default(),
        children![(
            Text::new("コンティニュー"),
            TextFont {
                font: font.clone(),
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

fn result_text_bundle(font: &Handle<Font>, score: &Score) -> impl Bundle {
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
                        Text::new("タイム: "),
                        TextFont {
                            font: font.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ),
                    (
                        Text::new(format!("{}", score.time)),
                        TextFont {
                            font: font.clone(),
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
                        Text::new("死亡数: "),
                        TextFont {
                            font: font.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ),
                    (
                        Text::new(format!("{}", score.death)),
                        TextFont {
                            font: font.clone(),
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

fn spawn_result_ui(mut commands: Commands, font: Res<JpFont>, score: Res<Score>) {
    let canvas = commands.spawn(result_canvas_bundle()).id();
    let text = commands.spawn(result_text_bundle(font.get(), &score)).id();
    let continue_button = commands.spawn(continue_button_bundle(font.get())).id();
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
