use crate::JpFont;
use crate::button::SizeUpButtonBundle;
use crate::config::GameConfig;
use crate::state::GameState;
use bevy::prelude::*;
const EXPLANATION_TEXT: &str = r#"
概要:
    ぐるぐる回してから離すことで移動してゴールを目指すゲームです。

登場するもの:
    プレイヤー: 隕石の見た目

操作方法:
    Spaceキーで拘束して回転と拘束を解くのを切り替える
    矢印キーで回転軸の相対座標と回転方向を切り替える
        左:水色軸反時計回転,
        右:ピンク軸時計回転,
        下:水色軸時計回転,
        上:ピンク軸反時計回転
    Rキーでチェックポイントに戻る
    Escapeキーで起動時の画面に戻る
"#;

pub struct StartUiPlugin;

impl Plugin for StartUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Start), spawn_start_ui)
            .add_systems(
                Update,
                (update_start_button, scroll_system).run_if(in_state(GameState::Start)),
            );
    }
}

#[derive(Component)]
struct StartButton;

fn start_button_bundle(font: &Handle<Font>) -> impl Bundle {
    (
        Button,
        StartButton,
        SizeUpButtonBundle::new(1.2, 10.0),
        UiTransform::default(),
        Node {
            width: Val::Px(280.0),
            min_width: percent(20),
            min_height: Val::Px(72.0),
            padding: UiRect::axes(Val::Px(24.0), Val::Px(16.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.9, 0.2)),
        children![(
            Node {
                max_width: percent(100),
                ..default()
            },
            Text::new("スタート"),
            TextFont {
                font: font.clone(),
                font_size: 32.0,
                ..default()
            },
            TextLayout::new_with_justify(Justify::Center),
            TextColor(Color::srgb(0.2, 0.2, 0.2))
        )],
    )
}

fn explanation_text_bundle(font: &Handle<Font>) -> impl Bundle {
    (
        Node {
            max_width: percent(100),
            ..default()
        },
        Text::new(EXPLANATION_TEXT),
        TextFont {
            font: font.clone(),
            font_size: 32.0,
            ..default()
        },
        TextLayout::new_with_justify(Justify::Left),
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
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            overflow: Overflow::clip(),
            padding: UiRect::top(Val::Px(24.0)),
            ..default()
        },
    )
}

fn spawn_start_ui(mut commands: Commands, font: Res<JpFont>) {
    let canvas = commands.spawn(start_canvas_bundle()).id();
    let sub_canvas = commands.spawn(start_sub_canvas_bundle()).id();
    let explanation_text = commands.spawn(explanation_text_bundle(font.get())).id();
    let start_button = commands.spawn(start_button_bundle(font.get())).id();
    commands
        .entity(sub_canvas)
        .add_children(&[explanation_text, start_button]);
    commands.entity(canvas).add_child(sub_canvas);
}

type StartButtonInputs = (Changed<Interaction>, With<StartButton>);

fn update_start_button(
    mut game_state: ResMut<NextState<GameState>>,
    mut query: Query<(&Interaction, &mut BackgroundColor), StartButtonInputs>,
    key: Res<ButtonInput<KeyCode>>,
    config: Res<GameConfig>,
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
    if key.just_pressed(config.input.next) {
        game_state.set(GameState::CourseSelection);
    }
}

use crate::course_selection::selection_ui::ScrollContent;
use bevy::input::mouse::MouseWheel;

fn start_sub_canvas_bundle() -> impl Bundle {
    (
        ScrollContent,
        Node {
            width: percent(100),
            min_height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            margin: UiRect::top(Val::Px(12.0)),
            row_gap: Val::Px(24.0),
            ..default()
        },
    )
}

fn scroll_system(
    mut wheel: MessageReader<MouseWheel>,
    mut query: Query<&mut Node, With<ScrollContent>>,
    mut offset: Local<f32>,
) {
    for ev in wheel.read() {
        *offset += ev.y * 20.0;
        *offset = offset.clamp(-1000.0, 0.0);

        for mut node in &mut query {
            node.margin.top = Val::Px(12.0 + *offset);
        }
    }
}
