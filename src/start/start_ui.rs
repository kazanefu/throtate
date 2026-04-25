use crate::state::GameState;
use crate::utils::FONT_PATH;
use bevy::prelude::*;
const EXPLANATION_TEXT: &str = r#"
ここにゲームの説明を書く
"#;

pub struct StartUiPlugin;

impl Plugin for StartUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Start), spawn_start_ui)
            .add_systems(
                Update,
                update_start_button.run_if(in_state(GameState::Start)),
            );
    }
}

#[derive(Component)]
struct StartButton;

fn start_button_bundle(asset_server: &AssetServer) -> impl Bundle {
    (
        Button,
        StartButton,
        Node {
            width: percent(20),
            height: percent(10),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.9, 0.2)),
        children![(
            Text::new("Start"),
            TextFont {
                font: asset_server.load(FONT_PATH),
                font_size: 40.0,
                ..default()
            },
            TextLayout::new_with_justify(Justify::Center),
            TextColor(Color::srgb(0.2, 0.2, 0.2))
        )],
    )
}

fn explanation_text_bundle(asset_server: &AssetServer) -> impl Bundle {
    (
        Text::new(EXPLANATION_TEXT),
        TextFont {
            font: asset_server.load(FONT_PATH),
            font_size: 40.0,
            ..default()
        },
        TextLayout::new_with_justify(Justify::Center),
        TextColor::WHITE,
    )
}

fn start_canvas_bundle() -> impl Bundle {
    (
        DespawnOnExit(GameState::Start),
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

fn spawn_start_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let canvas = commands.spawn(start_canvas_bundle()).id();
    let explanation_text = commands.spawn(explanation_text_bundle(&asset_server)).id();
    let start_button = commands.spawn(start_button_bundle(&asset_server)).id();
    commands
        .entity(canvas)
        .add_children(&[explanation_text, start_button]);
}

type StartButtonInputs = (Changed<Interaction>, With<StartButton>);

fn update_start_button(
    mut game_state: ResMut<NextState<GameState>>,
    mut query: Query<(&Interaction, &mut BackgroundColor), StartButtonInputs>,
) {
    for (interaction, mut background_color) in &mut query {
        match interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.2, 1.0, 0.3);
                game_state.set(GameState::CourseSelection);
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgb(0.1, 0.8, 0.4);
            }
            Interaction::None => {
                background_color.0 = Color::srgb(0.0, 0.5, 0.5);
            }
        }
    }
}
